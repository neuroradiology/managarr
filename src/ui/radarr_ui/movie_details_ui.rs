use std::iter;

use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::style::Style;
use tui::text::{Spans, Text};
use tui::widgets::{Cell, Paragraph, Row, Wrap};
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::models::radarr_models::{Credit, MovieHistoryItem, Release};
use crate::models::Route;
use crate::ui::utils::{
  borderless_block, get_width, layout_block_bottom_border, layout_block_top_border,
  spans_info_default, style_bold, style_default, style_failure, style_primary, style_success,
  style_warning, vertical_chunks,
};
use crate::ui::{
  draw_prompt_box, draw_prompt_popup_over, draw_table, draw_tabs, loading, TableProps,
};
use crate::utils::convert_to_gb;

pub(super) fn draw_movie_info_popup<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  let (content_area, _) = draw_tabs(f, area, "Movie Info", &app.data.radarr_data.movie_info_tabs);

  if let Route::Radarr(active_radarr_block) = app.get_current_route() {
    match active_radarr_block {
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt => draw_prompt_popup_over(
        f,
        app,
        content_area,
        draw_movie_info,
        draw_search_movie_prompt,
      ),
      ActiveRadarrBlock::RefreshAndScanPrompt => draw_prompt_popup_over(
        f,
        app,
        content_area,
        draw_movie_info,
        draw_refresh_and_scan_prompt,
      ),
      _ => draw_movie_info(f, app, content_area),
    }
  }
}

fn draw_movie_info<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, area: Rect) {
  if let Route::Radarr(active_radarr_block) =
    app.data.radarr_data.movie_info_tabs.get_active_route()
  {
    match active_radarr_block {
      ActiveRadarrBlock::FileInfo => draw_file_info(f, app, area),
      ActiveRadarrBlock::MovieDetails => draw_movie_details(f, app, area),
      ActiveRadarrBlock::MovieHistory => draw_movie_history(f, app, area),
      ActiveRadarrBlock::Cast => draw_movie_cast(f, app, area),
      ActiveRadarrBlock::Crew => draw_movie_crew(f, app, area),
      _ => (),
    }
  }
}

fn draw_search_movie_prompt<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, prompt_area: Rect) {
  draw_prompt_box(
    f,
    prompt_area,
    "  Confirm Search Movie?  ",
    format!(
      "Do you want to trigger an automatic search of your indexers for the movie: {}?",
      app.data.radarr_data.movies.current_selection().title
    )
    .as_str(),
    &app.data.radarr_data.prompt_confirm,
  );
}

fn draw_refresh_and_scan_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App,
  prompt_area: Rect,
) {
  draw_prompt_box(
    f,
    prompt_area,
    "  Confirm Refresh and Scan?  ",
    format!(
      "Do you want to trigger a refresh and disk scan for the movie: {}?",
      app.data.radarr_data.movies.current_selection().title
    )
    .as_str(),
    &app.data.radarr_data.prompt_confirm,
  );
}

fn draw_file_info<B: Backend>(f: &mut Frame<'_, B>, app: &App, content_area: Rect) {
  let file_info = app.data.radarr_data.file_details.to_owned();

  if !file_info.is_empty() {
    let audio_details = app.data.radarr_data.audio_details.to_owned();
    let video_details = app.data.radarr_data.video_details.to_owned();
    let chunks = vertical_chunks(
      vec![
        Constraint::Length(1),
        Constraint::Length(5),
        Constraint::Length(1),
        Constraint::Length(6),
        Constraint::Length(1),
        Constraint::Length(7),
      ],
      content_area,
    );
    let mut file_details_title = Text::from("File Details");
    let mut audio_details_title = Text::from("Audio Details");
    let mut video_details_title = Text::from("Video Details");
    file_details_title.patch_style(style_bold());
    audio_details_title.patch_style(style_bold());
    video_details_title.patch_style(style_bold());

    let file_details_title_paragraph = Paragraph::new(file_details_title).block(borderless_block());
    let audio_details_title_paragraph =
      Paragraph::new(audio_details_title).block(borderless_block());
    let video_details_title_paragraph =
      Paragraph::new(video_details_title).block(borderless_block());

    let file_details = Text::from(file_info);
    let audio_details = Text::from(audio_details);
    let video_details = Text::from(video_details);

    let file_details_paragraph = Paragraph::new(file_details)
      .block(layout_block_bottom_border())
      .wrap(Wrap { trim: false });
    let audio_details_paragraph = Paragraph::new(audio_details)
      .block(layout_block_bottom_border())
      .wrap(Wrap { trim: false });
    let video_details_paragraph = Paragraph::new(video_details)
      .block(borderless_block())
      .wrap(Wrap { trim: false });

    f.render_widget(file_details_title_paragraph, chunks[0]);
    f.render_widget(file_details_paragraph, chunks[1]);
    f.render_widget(audio_details_title_paragraph, chunks[2]);
    f.render_widget(audio_details_paragraph, chunks[3]);
    f.render_widget(video_details_title_paragraph, chunks[4]);
    f.render_widget(video_details_paragraph, chunks[5]);
  } else {
    loading(f, layout_block_top_border(), content_area, app.is_loading);
  }
}

fn draw_movie_details<B: Backend>(f: &mut Frame<'_, B>, app: &App, content_area: Rect) {
  let movie_details = app.data.radarr_data.movie_details.get_text();
  let block = layout_block_top_border();

  if !movie_details.is_empty() {
    let download_status = app
      .data
      .radarr_data
      .movie_details
      .items
      .iter()
      .find(|&line| line.starts_with("Status: "))
      .unwrap()
      .split(": ")
      .collect::<Vec<&str>>()[1];
    let mut text = Text::from(
      app
        .data
        .radarr_data
        .movie_details
        .items
        .iter()
        .map(|line| {
          let split = line.split(':').collect::<Vec<&str>>();
          let title = format!("{}:", split[0]);

          spans_info_default(title, split[1..].join(":"))
        })
        .collect::<Vec<Spans>>(),
    );
    text.patch_style(determine_style_from_download_status(download_status));

    let paragraph = Paragraph::new(text)
      .block(block)
      .wrap(Wrap { trim: false })
      .scroll((app.data.radarr_data.movie_details.offset, 0));

    f.render_widget(paragraph, content_area);
  } else {
    loading(f, block, content_area, app.is_loading);
  }
}

fn draw_movie_history<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, content_area: Rect) {
  let current_selection = if app.data.radarr_data.movie_history.items.is_empty() {
    MovieHistoryItem::default()
  } else {
    app.data.radarr_data.movie_history.current_selection_clone()
  };
  let block = layout_block_top_border();

  if app.data.radarr_data.movie_history.items.is_empty() && !app.is_loading {
    let no_history_paragraph = Paragraph::new(Text::from("No history"))
      .style(style_default())
      .block(block);

    f.render_widget(no_history_paragraph, content_area);
  } else {
    draw_table(
      f,
      content_area,
      block,
      TableProps {
        content: &mut app.data.radarr_data.movie_history,
        table_headers: vec!["Source Title", "Event Type", "Languages", "Quality", "Date"],
        constraints: vec![
          Constraint::Percentage(34),
          Constraint::Percentage(17),
          Constraint::Percentage(14),
          Constraint::Percentage(14),
          Constraint::Percentage(21),
        ],
      },
      |movie_history_item| {
        let MovieHistoryItem {
          source_title,
          quality,
          languages,
          date,
          event_type,
        } = movie_history_item;

        if current_selection == *movie_history_item
          && movie_history_item.source_title.text.len()
            > (content_area.width as f64 * 0.34) as usize
        {
          source_title.scroll_text();
        } else {
          source_title.reset_offset();
        }

        Row::new(vec![
          Cell::from(source_title.to_string()),
          Cell::from(event_type.to_owned()),
          Cell::from(
            languages
              .iter()
              .map(|language| language.name.to_owned())
              .collect::<Vec<String>>()
              .join(","),
          ),
          Cell::from(quality.quality.name.to_owned()),
          Cell::from(date.to_string()),
        ])
        .style(style_success())
      },
      app.is_loading,
    );
  }
}

fn draw_movie_cast<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, content_area: Rect) {
  draw_table(
    f,
    content_area,
    layout_block_top_border(),
    TableProps {
      content: &mut app.data.radarr_data.movie_cast,
      constraints: iter::repeat(Constraint::Ratio(1, 2)).take(2).collect(),
      table_headers: vec!["Cast Member", "Character"],
    },
    |cast_member| {
      let Credit {
        person_name,
        character,
        ..
      } = cast_member;

      Row::new(vec![
        Cell::from(person_name.to_owned()),
        Cell::from(character.clone().unwrap_or_default()),
      ])
      .style(style_success())
    },
    app.is_loading,
  )
}

fn draw_movie_crew<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, content_area: Rect) {
  draw_table(
    f,
    content_area,
    layout_block_top_border(),
    TableProps {
      content: &mut app.data.radarr_data.movie_crew,
      constraints: iter::repeat(Constraint::Ratio(1, 3)).take(3).collect(),
      table_headers: vec!["Crew Member", "Job", "Department"],
    },
    |crew_member| {
      let Credit {
        person_name,
        job,
        department,
        ..
      } = crew_member;

      Row::new(vec![
        Cell::from(person_name.to_owned()),
        Cell::from(job.clone().unwrap_or_default()),
        Cell::from(department.clone().unwrap_or_default()),
      ])
      .style(style_success())
    },
    app.is_loading,
  );
}

fn draw_movie_releases<B: Backend>(f: &mut Frame<'_, B>, app: &mut App, content_area: Rect) {
  let current_selection = if app.data.radarr_data.movie_releases.items.is_empty() {
    Release::default()
  } else {
    app
      .data
      .radarr_data
      .movie_releases
      .current_selection_clone()
  };

  draw_table(
    f,
    content_area,
    layout_block_top_border(),
    TableProps {
      content: &mut app.data.radarr_data.movie_releases,
      constraints: vec![
        Constraint::Length(8),
        Constraint::Length(10),
        Constraint::Length(1),
        Constraint::Percentage(40),
        Constraint::Percentage(10),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
      ],
      table_headers: vec![
        "Source", "Age", "⛔", "Title", "Indexer", "Size", "Peers", "Language", "Quality",
      ],
    },
    |release| {
      let Release {
        protocol,
        age,
        title,
        indexer,
        size,
        rejected,
        seeders,
        leechers,
        languages,
        quality,
        ..
      } = release;
      let age = format!("{} days", age.as_u64().unwrap());
      title.scroll_or_reset(get_width(content_area), current_selection == *release);
      indexer.scroll_or_reset(get_width(content_area), current_selection == *release);
      let size = convert_to_gb(size.as_u64().unwrap());
      let rejected_str = if *rejected { "⛔" } else { "" };
      let seeders = seeders.as_u64().unwrap();
      let leechers = leechers.as_u64().unwrap();
      let peers = format!("{} / {}", seeders, leechers);
      let language = if languages.is_some() {
        languages.clone().unwrap()[0].name.clone()
      } else {
        String::default()
      };
      let quality = quality.quality.name.clone();

      Row::new(vec![
        Cell::from(protocol.clone()),
        Cell::from(age),
        Cell::from(rejected_str).style(determine_style_from_rejection(*rejected)),
        Cell::from(title.to_string()),
        Cell::from(indexer.to_string()),
        Cell::from(format!("{} GB", size)),
        Cell::from(peers).style(determine_peer_style(seeders, leechers)),
        Cell::from(language),
        Cell::from(quality),
      ])
      .style(style_primary())
    },
    app.is_loading,
  )
}

fn determine_style_from_download_status(download_status: &str) -> Style {
  match download_status {
    "Downloaded" => style_success(),
    "Downloading" => style_warning(),
    "Missing" => style_failure(),
    _ => style_success(),
  }
}

fn determine_style_from_rejection(rejected: bool) -> Style {
  if rejected {
    style_failure()
  } else {
    style_primary()
  }
}

fn determine_peer_style(seeders: u64, leechers: u64) -> Style {
  if seeders == 0 {
    style_failure()
  } else if seeders < leechers {
    style_warning()
  } else {
    style_success()
  }
}
