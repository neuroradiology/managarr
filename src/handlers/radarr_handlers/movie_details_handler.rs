use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::Scrollable;
use crate::network::radarr_network::RadarrEvent;

pub(super) struct MovieDetailsHandler<'a> {
  key: &'a Key,
  app: &'a mut App,
  active_radarr_block: &'a ActiveRadarrBlock,
}

impl<'a> KeyEventHandler<'a, ActiveRadarrBlock> for MovieDetailsHandler<'a> {
  fn with(
    key: &'a Key,
    app: &'a mut App,
    active_block: &'a ActiveRadarrBlock,
  ) -> MovieDetailsHandler<'a> {
    MovieDetailsHandler {
      key,
      app,
      active_radarr_block: active_block,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn handle_scroll_up(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_up(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_up(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_up(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_up(),
      ActiveRadarrBlock::ManualSearch => self.app.data.radarr_data.movie_releases.scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_down(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_down(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_down(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_down(),
      ActiveRadarrBlock::ManualSearch => self.app.data.radarr_data.movie_releases.scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_to_top(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_to_top(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_to_top(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_to_top(),
      ActiveRadarrBlock::ManualSearch => self.app.data.radarr_data.movie_releases.scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_to_bottom(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_to_bottom(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_to_bottom(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_to_bottom(),
      ActiveRadarrBlock::ManualSearch => {
        self.app.data.radarr_data.movie_releases.scroll_to_bottom()
      }
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::FileInfo
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew
      | ActiveRadarrBlock::ManualSearch => match self.key {
        _ if *self.key == DEFAULT_KEYBINDINGS.left.key => {
          self.app.data.radarr_data.movie_info_tabs.previous();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .radarr_data
              .movie_info_tabs
              .get_active_route()
              .clone(),
          );
        }
        _ if *self.key == DEFAULT_KEYBINDINGS.right.key => {
          self.app.data.radarr_data.movie_info_tabs.next();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .radarr_data
              .movie_info_tabs
              .get_active_route()
              .clone(),
          );
        }
        _ => (),
      },
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt
      | ActiveRadarrBlock::RefreshAndScanPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action =
            Some(RadarrEvent::TriggerAutomaticSearch);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::RefreshAndScanPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::RefreshAndScan);
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::FileInfo
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew
      | ActiveRadarrBlock::ManualSearch => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_movie_info_tabs();
      }
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt
      | ActiveRadarrBlock::RefreshAndScanPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match *self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::FileInfo
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew
      | ActiveRadarrBlock::ManualSearch => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AutomaticallySearchMoviePrompt.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::RefreshAndScanPrompt.into());
        }
        _ => (),
      },
      _ => (),
    }
  }
}
