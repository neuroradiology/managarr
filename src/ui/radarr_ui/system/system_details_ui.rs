use tui::backend::Backend;
use tui::layout::Rect;
use tui::text::{Span, Text};
use tui::widgets::{Cell, ListItem, Paragraph, Row};
use tui::Frame;

use crate::app::key_binding::{build_keymapping_string, BARE_POPUP_KEY_MAPPINGS};
use crate::app::radarr::radarr_key_mappings::SYSTEM_TASKS_KEY_MAPPINGS;
use crate::app::radarr::{ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS};
use crate::app::App;
use crate::models::Route;
use crate::ui::radarr_ui::radarr_ui_utils::determine_log_style_by_level;
use crate::ui::radarr_ui::system::{
  draw_queued_events, draw_system_ui_layout, extract_task_props, TASK_TABLE_CONSTRAINTS,
  TASK_TABLE_HEADERS,
};
use crate::ui::utils::{borderless_block, style_primary, title_block};
use crate::ui::{
  draw_help_and_get_content_rect, draw_large_popup_over, draw_list_box, draw_medium_popup_over,
  draw_prompt_box, draw_prompt_popup_over, draw_table, loading, DrawUi, ListProps, TableProps,
};

#[cfg(test)]
#[path = "system_details_ui_tests.rs"]
mod system_details_ui_tests;

pub(super) struct SystemDetailsUi {}

impl DrawUi for SystemDetailsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return SYSTEM_DETAILS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
      match active_radarr_block {
        ActiveRadarrBlock::SystemLogs => {
          draw_large_popup_over(f, app, content_rect, draw_system_ui_layout, draw_logs_popup)
        }
        ActiveRadarrBlock::SystemTasks | ActiveRadarrBlock::SystemTaskStartConfirmPrompt => {
          draw_large_popup_over(
            f,
            app,
            content_rect,
            draw_system_ui_layout,
            draw_tasks_popup,
          )
        }
        ActiveRadarrBlock::SystemQueuedEvents => draw_medium_popup_over(
          f,
          app,
          content_rect,
          draw_system_ui_layout,
          draw_queued_events,
        ),
        ActiveRadarrBlock::SystemUpdates => draw_large_popup_over(
          f,
          app,
          content_rect,
          draw_system_ui_layout,
          draw_updates_popup,
        ),
        _ => (),
      }
    }
  }
}

fn draw_logs_popup<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  draw_list_box(
    f,
    area,
    |log| {
      let log_line = log.to_string();
      let level = log.text.split('|').collect::<Vec<&str>>()[1];
      let style = determine_log_style_by_level(level);

      ListItem::new(Text::from(Span::raw(log_line))).style(style)
    },
    ListProps {
      content: &mut app.data.radarr_data.log_details,
      title: "Log Details",
      is_loading: app.is_loading,
      is_popup: true,
      help: Some(format!(
        "<↑↓←→> scroll | {}",
        build_keymapping_string(&BARE_POPUP_KEY_MAPPINGS)
      )),
    },
  );
}

fn draw_tasks_popup<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let tasks_popup_table = |f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect| {
    f.render_widget(title_block("Tasks"), area);

    let context_area = draw_help_and_get_content_rect(
      f,
      area,
      Some(build_keymapping_string(&SYSTEM_TASKS_KEY_MAPPINGS)),
    );

    draw_table(
      f,
      context_area,
      borderless_block(),
      TableProps {
        content: &mut app.data.radarr_data.tasks,
        table_headers: TASK_TABLE_HEADERS.to_vec(),
        constraints: TASK_TABLE_CONSTRAINTS.to_vec(),
        help: None,
      },
      |task| {
        let task_props = extract_task_props(task);

        Row::new(vec![
          Cell::from(task_props.name),
          Cell::from(task_props.interval),
          Cell::from(task_props.last_execution),
          Cell::from(task_props.last_duration),
          Cell::from(task_props.next_execution),
        ])
        .style(style_primary())
      },
      app.is_loading,
      true,
    )
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

fn draw_start_task_prompt<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, prompt_area: Rect) {
  draw_prompt_box(
    f,
    prompt_area,
    "Start Task",
    format!(
      "Do you want to manually start this task: {}?",
      app.data.radarr_data.tasks.current_selection().name
    )
    .as_str(),
    app.data.radarr_data.prompt_confirm,
  );
}

fn draw_updates_popup<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  f.render_widget(title_block("Updates"), area);

  let content_rect = draw_help_and_get_content_rect(
    f,
    area,
    Some(format!(
      "<↑↓> scroll | {}",
      build_keymapping_string(&BARE_POPUP_KEY_MAPPINGS)
    )),
  );
  let updates = app.data.radarr_data.updates.get_text();
  let block = borderless_block();

  if !updates.is_empty() {
    let updates_paragraph = Paragraph::new(Text::from(updates))
      .block(block)
      .scroll((app.data.radarr_data.updates.offset, 0));

    f.render_widget(updates_paragraph, content_rect);
  } else {
    loading(f, block, content_rect, app.is_loading);
  }
}
