use ratatui::layout::{Alignment, Rect};
use ratatui::text::{Span, Text};
use ratatui::widgets::{Cell, ListItem, Paragraph, Row};
use ratatui::Frame;

use crate::app::context_clues::{build_context_clue_string, BARE_POPUP_CONTEXT_CLUES};
use crate::app::radarr::radarr_context_clues::SYSTEM_TASKS_CONTEXT_CLUES;
use crate::app::App;
use crate::models::radarr_models::Task;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS};
use crate::models::Route;
use crate::ui::radarr_ui::radarr_ui_utils::style_log_list_item;
use crate::ui::radarr_ui::system::{
  draw_queued_events, draw_system_ui_layout, extract_task_props, TASK_TABLE_CONSTRAINTS,
  TASK_TABLE_HEADERS,
};
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{borderless_block, title_block};
use crate::ui::widgets::loading_block::LoadingBlock;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::{
  draw_help_footer_and_get_content_area, draw_large_popup_over, draw_list_box,
  draw_medium_popup_over, draw_prompt_box, draw_prompt_popup_over, DrawUi, ListProps,
};

#[cfg(test)]
#[path = "system_details_ui_tests.rs"]
mod system_details_ui_tests;

pub(super) struct SystemDetailsUi;

impl DrawUi for SystemDetailsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return SYSTEM_DETAILS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
      match active_radarr_block {
        ActiveRadarrBlock::SystemLogs => {
          draw_large_popup_over(f, app, area, draw_system_ui_layout, draw_logs_popup)
        }
        ActiveRadarrBlock::SystemTasks | ActiveRadarrBlock::SystemTaskStartConfirmPrompt => {
          draw_large_popup_over(f, app, area, draw_system_ui_layout, draw_tasks_popup)
        }
        ActiveRadarrBlock::SystemQueuedEvents => {
          draw_medium_popup_over(f, app, area, draw_system_ui_layout, draw_queued_events)
        }
        ActiveRadarrBlock::SystemUpdates => {
          draw_large_popup_over(f, app, area, draw_system_ui_layout, draw_updates_popup)
        }
        _ => (),
      }
    }
  }
}

fn draw_logs_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_list_box(
    f,
    area,
    |log| {
      let log_line = log.to_string();
      let level = log.text.split('|').collect::<Vec<&str>>()[1].to_string();

      style_log_list_item(ListItem::new(Text::from(Span::raw(log_line))), level)
    },
    ListProps {
      content: &mut app.data.radarr_data.log_details,
      title: "Log Details",
      is_loading: app.is_loading,
      is_popup: true,
      help: Some(format!(
        "<↑↓←→> scroll | {}",
        build_context_clue_string(&BARE_POPUP_CONTEXT_CLUES)
      )),
    },
  );
}

fn draw_tasks_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let tasks_popup_table = |f: &mut Frame<'_>, app: &mut App<'_>, area: Rect| {
    let help_footer = Some(build_context_clue_string(&SYSTEM_TASKS_CONTEXT_CLUES));
    // let context_area = draw_help_footer_and_get_content_area(
    //   f,
    //   area,
    //   help_footer,
    // );
    let tasks_row_mapping = |task: &Task| {
      let task_props = extract_task_props(task);

      Row::new(vec![
        Cell::from(task_props.name),
        Cell::from(task_props.interval),
        Cell::from(task_props.last_execution),
        Cell::from(task_props.last_duration),
        Cell::from(task_props.next_execution),
      ])
      .primary()
    };
    let tasks_table = ManagarrTable::new(Some(&mut app.data.radarr_data.tasks), tasks_row_mapping)
      .block(borderless_block())
      .loading(app.is_loading)
      .margin(1)
      .footer(help_footer)
      .footer_alignment(Alignment::Center)
      .headers(TASK_TABLE_HEADERS)
      .constraints(TASK_TABLE_CONSTRAINTS);

    f.render_widget(title_block("Tasks"), area);
    f.render_widget(tasks_table, area);
  };

  if matches!(
    app.get_current_route(),
    Route::Radarr(ActiveRadarrBlock::SystemTaskStartConfirmPrompt, _)
  ) {
    draw_prompt_popup_over(f, app, area, tasks_popup_table, draw_start_task_prompt)
  } else {
    tasks_popup_table(f, app, area);
  }
}

fn draw_start_task_prompt(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_prompt_box(
    f,
    area,
    "Start Task",
    format!(
      "Do you want to manually start this task: {}?",
      app.data.radarr_data.tasks.current_selection().name
    )
    .as_str(),
    app.data.radarr_data.prompt_confirm,
  );
}

fn draw_updates_popup(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  f.render_widget(title_block("Updates"), area);

  let content_area = draw_help_footer_and_get_content_area(
    f,
    area,
    Some(format!(
      "<↑↓> scroll | {}",
      build_context_clue_string(&BARE_POPUP_CONTEXT_CLUES)
    )),
  );
  let updates = app.data.radarr_data.updates.get_text();
  let block = borderless_block();

  if !updates.is_empty() {
    let updates_paragraph = Paragraph::new(Text::from(updates))
      .block(block)
      .scroll((app.data.radarr_data.updates.offset, 0));

    f.render_widget(updates_paragraph, content_area);
  } else {
    f.render_widget(LoadingBlock::new(app.is_loading, block), content_area);
  }
}
