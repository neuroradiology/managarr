use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};

use serde::Deserialize;
use tui::widgets::{ListState, TableState};

use crate::app::radarr::ActiveRadarrBlock;

pub mod radarr_models;

#[cfg(test)]
#[path = "model_tests.rs"]
mod model_tests;

// Allowing dead code for now since we'll eventually be implementing additional Servarr support and we'll need it then
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
  Radarr(ActiveRadarrBlock, Option<ActiveRadarrBlock>),
  Sonarr,
  Readarr,
  Lidarr,
  Whisparr,
  Bazarr,
  Prowlarr,
  Tautulli,
}

pub trait Scrollable {
  fn scroll_down(&mut self);
  fn scroll_up(&mut self);
  fn scroll_to_top(&mut self);
  fn scroll_to_bottom(&mut self);
}

macro_rules! stateful_iterable {
  ($name:ident, $state:ty) => {
    #[derive(Default)]
    pub struct $name<T> {
      pub state: $state,
      pub items: Vec<T>,
    }

    impl<T> Scrollable for $name<T> {
      fn scroll_down(&mut self) {
        if self.items.is_empty() {
          return;
        }

        let selected_row = match self.state.selected() {
          Some(i) => {
            if i >= self.items.len() - 1 {
              0
            } else {
              i + 1
            }
          }
          None => 0,
        };

        self.state.select(Some(selected_row));
      }

      fn scroll_up(&mut self) {
        if self.items.is_empty() {
          return;
        }

        let selected_row = match self.state.selected() {
          Some(i) => {
            if i == 0 {
              self.items.len() - 1
            } else {
              i - 1
            }
          }
          None => 0,
        };

        self.state.select(Some(selected_row));
      }

      fn scroll_to_top(&mut self) {
        if self.items.is_empty() {
          return;
        }

        self.state.select(Some(0));
      }

      fn scroll_to_bottom(&mut self) {
        if self.items.is_empty() {
          return;
        }

        self.state.select(Some(self.items.len() - 1));
      }
    }

    impl<T> $name<T>
    where
      T: Clone + PartialEq + Eq + Debug,
    {
      pub fn set_items(&mut self, items: Vec<T>) {
        let items_len = items.len();
        self.items = items;
        if !self.items.is_empty() {
          let selected_row = self.state.selected().map_or(0, |i| {
            if i > 0 && i < items_len {
              i
            } else if i >= items_len {
              items_len - 1
            } else {
              0
            }
          });
          self.state.select(Some(selected_row));
        }
      }

      pub fn current_selection(&self) -> &T {
        &self.items[self.state.selected().unwrap_or(0)]
      }
    }
  };
}

stateful_iterable!(StatefulList, ListState);
stateful_iterable!(StatefulTable, TableState);

impl<T> StatefulTable<T>
where
  T: Clone + PartialEq + Eq + Debug,
{
  pub fn select_index(&mut self, index: Option<usize>) {
    self.state.select(index);
  }
}

#[derive(Default)]
pub struct ScrollableText {
  pub items: Vec<String>,
  pub offset: u16,
}

impl ScrollableText {
  pub fn with_string(item: String) -> ScrollableText {
    let items: Vec<&str> = item.split('\n').collect();
    let items: Vec<String> = items.iter().map(|it| it.to_string()).collect();
    ScrollableText { items, offset: 0 }
  }

  pub fn get_text(&self) -> String {
    self.items.join("\n")
  }
}

impl Scrollable for ScrollableText {
  fn scroll_down(&mut self) {
    if self.items.is_empty() {
      return;
    }

    if self.offset < (self.items.len() - 1) as u16 {
      self.offset += 1;
    }
  }

  fn scroll_up(&mut self) {
    if self.items.is_empty() {
      return;
    }

    if self.offset > 0 {
      self.offset -= 1;
    }
  }

  fn scroll_to_top(&mut self) {
    self.offset = 0;
  }

  fn scroll_to_bottom(&mut self) {
    self.offset = (self.items.len() - 1) as u16;
  }
}

#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(from = "String")]
pub struct HorizontallyScrollableText {
  pub text: String,
  pub offset: RefCell<usize>,
}

impl From<String> for HorizontallyScrollableText {
  fn from(text: String) -> HorizontallyScrollableText {
    HorizontallyScrollableText::new(text)
  }
}

impl From<&str> for HorizontallyScrollableText {
  fn from(text: &str) -> HorizontallyScrollableText {
    HorizontallyScrollableText::new(text.to_owned())
  }
}

impl Display for HorizontallyScrollableText {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if *self.offset.borrow() == 0 {
      write!(f, "{}", self.text)
    } else {
      let text_vec = self.text.chars().collect::<Vec<_>>();
      write!(
        f,
        "{}",
        text_vec[*self.offset.borrow()..]
          .iter()
          .cloned()
          .collect::<String>()
      )
    }
  }
}

impl HorizontallyScrollableText {
  pub fn new(text: String) -> HorizontallyScrollableText {
    HorizontallyScrollableText {
      text,
      offset: RefCell::new(0),
    }
  }

  pub fn scroll_left(&self) {
    if *self.offset.borrow() < self.text.len() {
      let new_offset = *self.offset.borrow() + 1;
      *self.offset.borrow_mut() = new_offset;
    }
  }

  pub fn scroll_right(&self) {
    if *self.offset.borrow() > 0 {
      let new_offset = *self.offset.borrow() - 1;
      *self.offset.borrow_mut() = new_offset;
    }
  }

  pub fn scroll_home(&self) {
    *self.offset.borrow_mut() = self.text.len();
  }

  pub fn reset_offset(&self) {
    *self.offset.borrow_mut() = 0;
  }

  pub fn scroll_left_or_reset(&self, width: usize, is_current_selection: bool, can_scroll: bool) {
    if can_scroll && is_current_selection && self.text.len() >= width {
      if *self.offset.borrow() < self.text.len() {
        self.scroll_left();
      } else {
        self.reset_offset();
      }
    } else if *self.offset.borrow() != 0 && !is_current_selection {
      self.reset_offset();
    }
  }

  pub fn drain(&mut self) -> String {
    self.reset_offset();
    self.text.drain(..).collect()
  }

  pub fn pop(&mut self) {
    if *self.offset.borrow() < self.text.len() {
      self
        .text
        .remove(self.text.len() - *self.offset.borrow() - 1);
    }
  }

  pub fn push(&mut self, character: char) {
    self
      .text
      .insert(self.text.len() - *self.offset.borrow(), character);
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TabRoute {
  pub title: &'static str,
  pub route: Route,
  pub help: &'static str,
  pub contextual_help: Option<&'static str>,
}

pub struct TabState {
  pub tabs: Vec<TabRoute>,
  pub index: usize,
}

impl TabState {
  pub fn new(tabs: Vec<TabRoute>) -> TabState {
    TabState { tabs, index: 0 }
  }

  // Allowing this code for now since we'll eventually be implementing additional Servarr support and we'll need it then
  #[allow(dead_code)]
  pub fn set_index(&mut self, index: usize) -> &TabRoute {
    self.index = index;
    &self.tabs[self.index]
  }

  pub fn get_active_route(&self) -> &Route {
    &self.tabs[self.index].route
  }

  pub fn get_active_tab_help(&self) -> &'static str {
    self.tabs[self.index].help
  }

  pub fn get_active_tab_contextual_help(&self) -> Option<&'static str> {
    self.tabs[self.index].contextual_help
  }

  pub fn next(&mut self) {
    self.index = (self.index + 1) % self.tabs.len();
  }

  pub fn previous(&mut self) {
    if self.index > 0 {
      self.index -= 1;
    } else {
      self.index = self.tabs.len() - 1;
    }
  }
}

#[derive(Default, Eq, PartialEq, Debug)]
pub struct BlockSelectionState<'a, T>
where
  T: Sized + Clone + Copy + Default,
{
  pub blocks: &'a [T],
  pub index: usize,
}

impl<'a, T> BlockSelectionState<'a, T>
where
  T: Sized + Clone + Copy + Default,
{
  pub fn new(blocks: &'a [T]) -> BlockSelectionState<'a, T> {
    BlockSelectionState { blocks, index: 0 }
  }

  pub fn get_active_block(&self) -> &T {
    &self.blocks[self.index]
  }

  pub fn next(&mut self) {
    self.index = (self.index + 1) % self.blocks.len();
  }

  pub fn previous(&mut self) {
    if self.index > 0 {
      self.index -= 1;
    } else {
      self.index = self.blocks.len() - 1;
    }
  }
}

#[cfg(test)]
impl<'a, T> BlockSelectionState<'a, T>
where
  T: Sized + Clone + Copy + Default,
{
  pub fn set_index(&mut self, index: usize) {
    self.index = index;
  }
}
