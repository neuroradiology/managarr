use std::iter;

use chrono::{Duration, Utc};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::ListItem;
use tui::widgets::Paragraph;
use tui::Frame;

use crate::app::radarr::{ActiveRadarrBlock, RadarrData, FILTER_BLOCKS, SEARCH_BLOCKS};
use crate::app::App;
use crate::logos::RADARR_LOGO;
use crate::models::radarr_models::{DiskSpace, DownloadRecord, Movie, RootFolder};
use crate::models::Route;
use crate::ui::draw_selectable_list;
use crate::ui::draw_tabs;
use crate::ui::loading;
use crate::ui::radarr_ui::collections::CollectionsUi;
use crate::ui::radarr_ui::downloads::DownloadsUi;
use crate::ui::radarr_ui::indexers::IndexersUi;
use crate::ui::radarr_ui::library::LibraryUi;
use crate::ui::radarr_ui::root_folders::RootFoldersUi;
use crate::ui::radarr_ui::system::SystemUi;
use crate::ui::utils::{
  borderless_block, horizontal_chunks, layout_block, line_gauge_with_label, line_gauge_with_title,
  show_cursor, style_awaiting_import, style_bold, style_default, style_failure, style_success,
  style_unmonitored, style_warning, title_block, title_block_centered, vertical_chunks_with_margin,
};
use crate::ui::DrawUi;
use crate::utils::convert_to_gb;

mod collections;
mod downloads;
mod indexers;
mod library;
mod radarr_ui_utils;
mod root_folders;
mod system;

#[cfg(test)]
#[path = "radarr_ui_tests.rs"]
mod radarr_ui_tests;

pub(super) struct RadarrUi {}

impl DrawUi for RadarrUi {
  fn accepts(route: Route) -> bool {
    matches!(route, Route::Radarr(_, _))
  }

  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
    let (content_rect, _) = draw_tabs(f, area, "Movies", &app.data.radarr_data.main_tabs);
    let route = *app.get_current_route();

    match route {
      _ if LibraryUi::accepts(route) => LibraryUi::draw(f, app, content_rect),
      _ if CollectionsUi::accepts(route) => CollectionsUi::draw(f, app, content_rect),
      _ if DownloadsUi::accepts(route) => DownloadsUi::draw(f, app, content_rect),
      _ if IndexersUi::accepts(route) => IndexersUi::draw(f, app, content_rect),
      _ if RootFoldersUi::accepts(route) => RootFoldersUi::draw(f, app, content_rect),
      _ if SystemUi::accepts(route) => SystemUi::draw(f, app, content_rect),
      _ => (),
    }
  }

  fn draw_context_row<B: Backend>(f: &mut Frame<'_, B>, app: &App<'_>, area: Rect) {
    let chunks = horizontal_chunks(vec![Constraint::Min(0), Constraint::Length(20)], area);

    let context_chunks = horizontal_chunks(
      vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)],
      chunks[0],
    );

    draw_stats_context(f, app, context_chunks[0]);
    draw_downloads_context(f, app, context_chunks[1]);
    draw_radarr_logo(f, chunks[1]);
  }
}

fn draw_stats_context<B: Backend>(f: &mut Frame<'_, B>, app: &App<'_>, area: Rect) {
  let block = title_block("Stats");

  if !app.data.radarr_data.version.is_empty() {
    f.render_widget(block, area);
    let RadarrData {
      root_folders,
      disk_space_vec,
      start_time,
      ..
    } = &app.data.radarr_data;

    let mut constraints = vec![
      Constraint::Length(1),
      Constraint::Length(1),
      Constraint::Length(1),
    ];

    constraints.append(
      &mut iter::repeat(Constraint::Length(1))
        .take(disk_space_vec.len() + root_folders.items.len() + 1)
        .collect(),
    );

    let chunks = vertical_chunks_with_margin(constraints, area, 1);

    let version_paragraph = Paragraph::new(Text::from(format!(
      "Radarr Version:  {}",
      app.data.radarr_data.version
    )))
    .block(borderless_block())
    .style(style_bold());

    let uptime = Utc::now() - start_time.to_owned();
    let days = uptime.num_days();
    let day_difference = uptime - Duration::days(days);
    let hours = day_difference.num_hours();
    let hour_difference = day_difference - Duration::hours(hours);
    let minutes = hour_difference.num_minutes();
    let seconds = (hour_difference - Duration::minutes(minutes)).num_seconds();

    let uptime_paragraph = Paragraph::new(Text::from(format!(
      "Uptime: {}d {:0width$}:{:0width$}:{:0width$}",
      days,
      hours,
      minutes,
      seconds,
      width = 2
    )))
    .block(borderless_block())
    .style(style_bold());

    let storage =
      Paragraph::new(Text::from("Storage:")).block(borderless_block().style(style_bold()));
    let folders =
      Paragraph::new(Text::from("Root Folders:")).block(borderless_block().style(style_bold()));

    f.render_widget(version_paragraph, chunks[0]);
    f.render_widget(uptime_paragraph, chunks[1]);
    f.render_widget(storage, chunks[2]);

    for i in 0..disk_space_vec.len() {
      let DiskSpace {
        free_space,
        total_space,
      } = &disk_space_vec[i];
      let title = format!("Disk {}", i + 1);
      let ratio = if total_space.as_u64().unwrap() == 0 {
        0f64
      } else {
        1f64 - (free_space.as_u64().unwrap() as f64 / total_space.as_u64().unwrap() as f64)
      };

      let space_gauge = line_gauge_with_label(title.as_str(), ratio);

      f.render_widget(space_gauge, chunks[i + 3]);
    }

    f.render_widget(folders, chunks[disk_space_vec.len() + 3]);

    for i in 0..root_folders.items.len() {
      let RootFolder {
        path, free_space, ..
      } = &root_folders.items[i];
      let space: f64 = convert_to_gb(free_space.as_u64().unwrap());
      let root_folder_space = Paragraph::new(format!("{}: {:.2} GB free", path.to_owned(), space))
        .block(borderless_block())
        .style(style_default());

      f.render_widget(root_folder_space, chunks[i + disk_space_vec.len() + 4])
    }
  } else {
    loading(f, block, area, app.is_loading);
  }
}

fn draw_downloads_context<B: Backend>(f: &mut Frame<'_, B>, app: &App<'_>, area: Rect) {
  let block = title_block("Downloads");
  let downloads_vec = &app.data.radarr_data.downloads.items;

  if !downloads_vec.is_empty() {
    f.render_widget(block, area);

    let constraints = iter::repeat(Constraint::Length(2))
      .take(downloads_vec.len())
      .collect::<Vec<Constraint>>();

    let chunks = vertical_chunks_with_margin(constraints, area, 1);

    for i in 0..downloads_vec.len() {
      let DownloadRecord {
        title,
        sizeleft,
        size,
        ..
      } = &downloads_vec[i];
      let percent = 1f64 - (sizeleft.as_f64().unwrap() / size.as_f64().unwrap());
      let download_gauge = line_gauge_with_title(title, percent);

      f.render_widget(download_gauge, chunks[i]);
    }
  } else {
    loading(f, block, area, app.is_loading);
  }
}

fn determine_row_style(downloads_vec: &[DownloadRecord], movie: &Movie) -> Style {
  if !movie.has_file {
    if let Some(download) = downloads_vec
      .iter()
      .find(|&download| download.movie_id == movie.id)
    {
      if download.status == "downloading" {
        return style_warning();
      }

      if download.status == "completed" {
        return style_awaiting_import();
      }
    }

    return style_failure();
  }

  if !movie.monitored {
    style_unmonitored()
  } else {
    style_success()
  }
}

fn draw_select_minimum_availability_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  popup_area: Rect,
) {
  draw_selectable_list(
    f,
    popup_area,
    &mut app.data.radarr_data.minimum_availability_list,
    |minimum_availability| ListItem::new(minimum_availability.to_display_str().to_owned()),
  );
}

fn draw_select_quality_profile_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  popup_area: Rect,
) {
  draw_selectable_list(
    f,
    popup_area,
    &mut app.data.radarr_data.quality_profile_list,
    |quality_profile| ListItem::new(quality_profile.clone()),
  );
}

fn draw_select_root_folder_popup<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  popup_area: Rect,
) {
  draw_selectable_list(
    f,
    popup_area,
    &mut app.data.radarr_data.root_folder_list,
    |root_folder| ListItem::new(root_folder.path.to_owned()),
  );
}

fn draw_search_box<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let chunks =
    vertical_chunks_with_margin(vec![Constraint::Length(3), Constraint::Min(0)], area, 1);
  if !app.data.radarr_data.is_searching {
    let error_msg = match app.get_current_route() {
      Route::Radarr(active_radarr_block, _) => match active_radarr_block {
        ActiveRadarrBlock::SearchMovie => "Movie not found!",
        ActiveRadarrBlock::SearchCollection => "Collection not found!",
        _ => "",
      },
      _ => "",
    };

    let input = Paragraph::new(error_msg)
      .style(style_failure())
      .block(layout_block());

    f.render_widget(input, chunks[0]);
  } else {
    let default_content = String::default();
    let (block_title, offset, block_content) = match app.get_current_route() {
      Route::Radarr(active_radarr_block, _) => match active_radarr_block {
        _ if SEARCH_BLOCKS.contains(active_radarr_block) => (
          "Search",
          *app.data.radarr_data.search.offset.borrow(),
          &app.data.radarr_data.search.text,
        ),
        _ => ("", 0, &default_content),
      },
      _ => ("", 0, &default_content),
    };

    let input = Paragraph::new(block_content.as_str())
      .style(style_default())
      .block(title_block_centered(block_title));
    show_cursor(f, chunks[0], offset, block_content);

    f.render_widget(input, chunks[0]);
  }
}

fn draw_filter_box<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let chunks =
    vertical_chunks_with_margin(vec![Constraint::Length(3), Constraint::Min(0)], area, 1);
  if !app.data.radarr_data.is_filtering {
    let error_msg = match app.get_current_route() {
      Route::Radarr(active_radarr_block, _) => match active_radarr_block {
        ActiveRadarrBlock::FilterMovies => "No movies found matching filter!",
        ActiveRadarrBlock::FilterCollections => "No collections found matching filter!",
        _ => "",
      },
      _ => "",
    };

    let input = Paragraph::new(error_msg)
      .style(style_failure())
      .block(layout_block());

    f.render_widget(input, chunks[0]);
  } else {
    let default_content = String::default();
    let (block_title, offset, block_content) = match app.get_current_route() {
      Route::Radarr(active_radarr_block, _) => match active_radarr_block {
        _ if FILTER_BLOCKS.contains(active_radarr_block) => (
          "Filter",
          *app.data.radarr_data.filter.offset.borrow(),
          &app.data.radarr_data.filter.text,
        ),
        _ => ("", 0, &default_content),
      },
      _ => ("", 0, &default_content),
    };

    let input = Paragraph::new(block_content.as_str())
      .style(style_default())
      .block(title_block_centered(block_title));
    show_cursor(f, chunks[0], offset, block_content);

    f.render_widget(input, chunks[0]);
  }
}

fn draw_radarr_logo<B: Backend>(f: &mut Frame<'_, B>, area: Rect) {
  let mut logo_text = Text::from(RADARR_LOGO);
  logo_text.patch_style(Style::default().fg(Color::LightYellow));
  let logo = Paragraph::new(logo_text)
    .block(layout_block())
    .alignment(Alignment::Center);
  f.render_widget(logo, area);
}
