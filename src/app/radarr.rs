use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::IntoEnumIterator;

use crate::app::{App, Route};
use crate::models::radarr_models::{
  AddMovieSearchResult, Collection, CollectionMovie, Credit, DiskSpace, DownloadRecord,
  MinimumAvailability, Monitor, Movie, MovieHistoryItem, Release, ReleaseField, RootFolder,
};
use crate::models::{
  BlockSelectionState, HorizontallyScrollableText, ScrollableText, StatefulList, StatefulTable,
  TabRoute, TabState,
};
use crate::network::radarr_network::RadarrEvent;

pub struct RadarrData<'a> {
  pub root_folders: StatefulTable<RootFolder>,
  pub disk_space_vec: Vec<DiskSpace>,
  pub version: String,
  pub start_time: DateTime<Utc>,
  pub movies: StatefulTable<Movie>,
  pub filtered_movies: StatefulTable<Movie>,
  pub add_searched_movies: StatefulTable<AddMovieSearchResult>,
  pub monitor_list: StatefulList<Monitor>,
  pub minimum_availability_list: StatefulList<MinimumAvailability>,
  pub quality_profile_list: StatefulList<String>,
  pub root_folder_list: StatefulList<RootFolder>,
  pub selected_block: BlockSelectionState<'a, ActiveRadarrBlock>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub quality_profile_map: BiMap<u64, String>,
  pub tags_map: BiMap<u64, String>,
  pub movie_details: ScrollableText,
  pub file_details: String,
  pub audio_details: String,
  pub video_details: String,
  pub movie_history: StatefulTable<MovieHistoryItem>,
  pub movie_cast: StatefulTable<Credit>,
  pub movie_crew: StatefulTable<Credit>,
  pub movie_releases: StatefulTable<Release>,
  pub movie_releases_sort: StatefulList<ReleaseField>,
  pub collections: StatefulTable<Collection>,
  pub filtered_collections: StatefulTable<Collection>,
  pub collection_movies: StatefulTable<CollectionMovie>,
  pub prompt_confirm_action: Option<RadarrEvent>,
  pub main_tabs: TabState,
  pub movie_info_tabs: TabState,
  pub search: HorizontallyScrollableText,
  pub filter: HorizontallyScrollableText,
  pub edit_path: HorizontallyScrollableText,
  pub edit_tags: HorizontallyScrollableText,
  pub edit_monitored: Option<bool>,
  pub edit_search_on_add: Option<bool>,
  pub sort_ascending: Option<bool>,
  pub prompt_confirm: bool,
  pub delete_movie_files: bool,
  pub add_list_exclusion: bool,
  pub is_searching: bool,
  pub is_filtering: bool,
}

impl<'a> RadarrData<'a> {
  pub fn reset_movie_collection_table(&mut self) {
    self.collection_movies = StatefulTable::default();
  }

  pub fn reset_delete_movie_preferences(&mut self) {
    self.delete_movie_files = false;
    self.add_list_exclusion = false;
  }

  pub fn reset_search(&mut self) {
    self.is_searching = false;
    self.search = HorizontallyScrollableText::default();
    self.filter = HorizontallyScrollableText::default();
    self.filtered_movies = StatefulTable::default();
    self.filtered_collections = StatefulTable::default();
    self.add_searched_movies = StatefulTable::default();
  }

  pub fn reset_filter(&mut self) {
    self.is_filtering = false;
    self.filter = HorizontallyScrollableText::default();
    self.filtered_movies = StatefulTable::default();
    self.filtered_collections = StatefulTable::default();
  }

  pub fn reset_add_edit_media_fields(&mut self) {
    self.edit_monitored = None;
    self.edit_search_on_add = None;
    self.edit_path = HorizontallyScrollableText::default();
    self.edit_tags = HorizontallyScrollableText::default();
    self.reset_preferences_selections();
  }

  pub fn reset_movie_info_tabs(&mut self) {
    self.file_details = String::default();
    self.audio_details = String::default();
    self.video_details = String::default();
    self.movie_details = ScrollableText::default();
    self.movie_history = StatefulTable::default();
    self.movie_cast = StatefulTable::default();
    self.movie_crew = StatefulTable::default();
    self.movie_releases = StatefulTable::default();
    self.movie_releases_sort = StatefulList::default();
    self.sort_ascending = None;
    self.movie_info_tabs.index = 0;
  }

  pub fn reset_preferences_selections(&mut self) {
    self.monitor_list = StatefulList::default();
    self.minimum_availability_list = StatefulList::default();
    self.quality_profile_list = StatefulList::default();
    self.root_folder_list = StatefulList::default();
  }

  pub fn populate_preferences_lists(&mut self) {
    self.monitor_list.set_items(Vec::from_iter(Monitor::iter()));
    self
      .minimum_availability_list
      .set_items(Vec::from_iter(MinimumAvailability::iter()));
    let mut quality_profile_names: Vec<String> =
      self.quality_profile_map.right_values().cloned().collect();
    quality_profile_names.sort();
    self.quality_profile_list.set_items(quality_profile_names);
    self
      .root_folder_list
      .set_items(self.root_folders.items.to_vec());
  }

  pub fn populate_edit_movie_fields(&mut self) {
    self.populate_preferences_lists();
    let Movie {
      path,
      tags,
      monitored,
      minimum_availability,
      quality_profile_id,
      ..
    } = if self.filtered_movies.items.is_empty() {
      self.movies.current_selection()
    } else {
      self.filtered_movies.current_selection()
    };

    self.edit_path = path.clone().into();
    self.edit_tags = tags
      .iter()
      .map(|tag_id| {
        self
          .tags_map
          .get_by_left(&tag_id.as_u64().unwrap())
          .unwrap()
          .clone()
      })
      .collect::<Vec<String>>()
      .join(", ")
      .into();
    self.edit_monitored = Some(*monitored);

    let minimum_availability_index = self
      .minimum_availability_list
      .items
      .iter()
      .position(|ma| ma == minimum_availability);
    self
      .minimum_availability_list
      .state
      .select(minimum_availability_index);

    let quality_profile_name = self
      .quality_profile_map
      .get_by_left(&quality_profile_id.as_u64().unwrap())
      .unwrap();
    let quality_profile_index = self
      .quality_profile_list
      .items
      .iter()
      .position(|profile| profile == quality_profile_name);
    self
      .quality_profile_list
      .state
      .select(quality_profile_index);
  }

  pub fn populate_edit_collection_fields(&mut self) {
    self.populate_preferences_lists();
    let Collection {
      root_folder_path,
      monitored,
      search_on_add,
      minimum_availability,
      quality_profile_id,
      ..
    } = if self.filtered_collections.items.is_empty() {
      self.collections.current_selection()
    } else {
      self.filtered_collections.current_selection()
    };

    self.edit_path = root_folder_path.clone().unwrap_or_default().into();
    self.edit_monitored = Some(*monitored);
    self.edit_search_on_add = Some(*search_on_add);

    let minimum_availability_index = self
      .minimum_availability_list
      .items
      .iter()
      .position(|ma| ma == minimum_availability);
    self
      .minimum_availability_list
      .state
      .select(minimum_availability_index);

    let quality_profile_name = self
      .quality_profile_map
      .get_by_left(&quality_profile_id.as_u64().unwrap())
      .unwrap();
    let quality_profile_index = self
      .quality_profile_list
      .items
      .iter()
      .position(|profile| profile == quality_profile_name);
    self
      .quality_profile_list
      .state
      .select(quality_profile_index);
  }
}

impl<'a> Default for RadarrData<'a> {
  fn default() -> RadarrData<'a> {
    RadarrData {
      root_folders: StatefulTable::default(),
      disk_space_vec: Vec::new(),
      version: String::default(),
      start_time: DateTime::default(),
      movies: StatefulTable::default(),
      add_searched_movies: StatefulTable::default(),
      monitor_list: StatefulList::default(),
      minimum_availability_list: StatefulList::default(),
      quality_profile_list: StatefulList::default(),
      root_folder_list: StatefulList::default(),
      selected_block: BlockSelectionState::default(),
      filtered_movies: StatefulTable::default(),
      downloads: StatefulTable::default(),
      quality_profile_map: BiMap::default(),
      tags_map: BiMap::default(),
      file_details: String::default(),
      audio_details: String::default(),
      video_details: String::default(),
      movie_details: ScrollableText::default(),
      movie_history: StatefulTable::default(),
      movie_cast: StatefulTable::default(),
      movie_crew: StatefulTable::default(),
      movie_releases: StatefulTable::default(),
      movie_releases_sort: StatefulList::default(),
      collections: StatefulTable::default(),
      filtered_collections: StatefulTable::default(),
      collection_movies: StatefulTable::default(),
      prompt_confirm_action: None,
      search: HorizontallyScrollableText::default(),
      filter: HorizontallyScrollableText::default(),
      edit_path: HorizontallyScrollableText::default(),
      edit_tags: HorizontallyScrollableText::default(),
      edit_monitored: None,
      edit_search_on_add: None,
      sort_ascending: None,
      is_searching: false,
      is_filtering: false,
      prompt_confirm: false,
      delete_movie_files: false,
      add_list_exclusion: false,
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Library",
          route: ActiveRadarrBlock::Movies.into(),
          help: "",
          contextual_help: Some("<a> add | <e> edit | <del> delete | <s> search | <f> filter | <r> refresh | <u> update all | <enter> details | <esc> cancel filter"),
        },
        TabRoute {
          title: "Downloads",
          route: ActiveRadarrBlock::Downloads.into(),
          help: "",
          contextual_help: Some("<r> refresh | <del> delete"),
        },
        TabRoute {
          title: "Collections",
          route: ActiveRadarrBlock::Collections.into(),
          help: "",
          contextual_help: Some("<s> search | <e> edit | <f> filter | <r> refresh | <u> update all | <enter> details | <esc> cancel filter"),
        },
        TabRoute {
          title: "Root Folders",
          route: ActiveRadarrBlock::RootFolders.into(),
          help: "",
          contextual_help: Some("<a> add | <del> delete | <r> refresh"),
        },
        TabRoute {
          title: "System",
          route: ActiveRadarrBlock::System.into(),
          help: "",
          contextual_help: Some("<enter> select menu item | <esc> go back to menu selection")
        }
      ]),
      movie_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Details",
          route: ActiveRadarrBlock::MovieDetails.into(),
          help: "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close",
          contextual_help: None
        },
        TabRoute {
          title: "History",
          route: ActiveRadarrBlock::MovieHistory.into(),
          help: "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close",
          contextual_help: None
        },
        TabRoute {
          title: "File",
          route: ActiveRadarrBlock::FileInfo.into(),
          help: "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close",
          contextual_help: None,
        },
        TabRoute {
          title: "Cast",
          route: ActiveRadarrBlock::Cast.into(),
          help: "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close",
          contextual_help: None,
        },
        TabRoute {
          title: "Crew",
          route: ActiveRadarrBlock::Crew.into(),
          help: "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close",
          contextual_help: None,
        },
        TabRoute {
          title: "Manual Search",
          route: ActiveRadarrBlock::ManualSearch.into(),
          help: "<r> refresh | <u> update | <e> edit | <o> sort | <s> auto search | <esc> close",
          contextual_help: Some("<enter> details")
        }
      ]),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ActiveRadarrBlock {
  AddMovieAlreadyInLibrary,
  AddMovieSearchInput,
  AddMovieSearchResults,
  AddMoviePrompt,
  AddMovieSelectMinimumAvailability,
  AddMovieSelectQualityProfile,
  AddMovieSelectMonitor,
  AddMovieSelectRootFolder,
  AddMovieConfirmPrompt,
  AddMovieTagsInput,
  AddMovieEmptySearchResults,
  AddRootFolderPrompt,
  AutomaticallySearchMoviePrompt,
  Collections,
  CollectionDetails,
  Cast,
  Crew,
  DeleteMoviePrompt,
  DeleteMovieConfirmPrompt,
  DeleteMovieToggleDeleteFile,
  DeleteMovieToggleAddListExclusion,
  DeleteDownloadPrompt,
  DeleteRootFolderPrompt,
  Downloads,
  EditCollectionPrompt,
  EditCollectionConfirmPrompt,
  EditCollectionRootFolderPathInput,
  EditCollectionSelectMinimumAvailability,
  EditCollectionSelectQualityProfile,
  EditCollectionToggleSearchOnAdd,
  EditCollectionToggleMonitored,
  EditMoviePrompt,
  EditMovieConfirmPrompt,
  EditMoviePathInput,
  EditMovieSelectMinimumAvailability,
  EditMovieSelectQualityProfile,
  EditMovieTagsInput,
  EditMovieToggleMonitored,
  FileInfo,
  FilterCollections,
  FilterMovies,
  ManualSearch,
  ManualSearchSortPrompt,
  ManualSearchConfirmPrompt,
  MovieDetails,
  MovieHistory,
  #[default]
  Movies,
  RootFolders,
  System,
  UpdateAndScanPrompt,
  UpdateAllCollectionsPrompt,
  UpdateAllMoviesPrompt,
  UpdateDownloadsPrompt,
  SearchMovie,
  SearchCollection,
  ViewMovieOverview,
}

pub const ADD_MOVIE_BLOCKS: [ActiveRadarrBlock; 10] = [
  ActiveRadarrBlock::AddMovieSearchInput,
  ActiveRadarrBlock::AddMovieSearchResults,
  ActiveRadarrBlock::AddMovieEmptySearchResults,
  ActiveRadarrBlock::AddMoviePrompt,
  ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
  ActiveRadarrBlock::AddMovieSelectMonitor,
  ActiveRadarrBlock::AddMovieSelectQualityProfile,
  ActiveRadarrBlock::AddMovieSelectRootFolder,
  ActiveRadarrBlock::AddMovieAlreadyInLibrary,
  ActiveRadarrBlock::AddMovieTagsInput,
];
pub const ADD_MOVIE_SELECTION_BLOCKS: [ActiveRadarrBlock; 6] = [
  ActiveRadarrBlock::AddMovieSelectRootFolder,
  ActiveRadarrBlock::AddMovieSelectMonitor,
  ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
  ActiveRadarrBlock::AddMovieSelectQualityProfile,
  ActiveRadarrBlock::AddMovieTagsInput,
  ActiveRadarrBlock::AddMovieConfirmPrompt,
];
pub const EDIT_COLLECTION_BLOCKS: [ActiveRadarrBlock; 7] = [
  ActiveRadarrBlock::EditCollectionPrompt,
  ActiveRadarrBlock::EditCollectionConfirmPrompt,
  ActiveRadarrBlock::EditCollectionRootFolderPathInput,
  ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
  ActiveRadarrBlock::EditCollectionSelectQualityProfile,
  ActiveRadarrBlock::EditCollectionToggleSearchOnAdd,
  ActiveRadarrBlock::EditCollectionToggleMonitored,
];
pub const EDIT_COLLECTION_SELECTION_BLOCKS: [ActiveRadarrBlock; 6] = [
  ActiveRadarrBlock::EditCollectionToggleMonitored,
  ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
  ActiveRadarrBlock::EditCollectionSelectQualityProfile,
  ActiveRadarrBlock::EditCollectionRootFolderPathInput,
  ActiveRadarrBlock::EditCollectionToggleSearchOnAdd,
  ActiveRadarrBlock::EditCollectionConfirmPrompt,
];
pub const EDIT_MOVIE_BLOCKS: [ActiveRadarrBlock; 7] = [
  ActiveRadarrBlock::EditMoviePrompt,
  ActiveRadarrBlock::EditMovieConfirmPrompt,
  ActiveRadarrBlock::EditMoviePathInput,
  ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
  ActiveRadarrBlock::EditMovieSelectQualityProfile,
  ActiveRadarrBlock::EditMovieTagsInput,
  ActiveRadarrBlock::EditMovieToggleMonitored,
];
pub const EDIT_MOVIE_SELECTION_BLOCKS: [ActiveRadarrBlock; 6] = [
  ActiveRadarrBlock::EditMovieToggleMonitored,
  ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
  ActiveRadarrBlock::EditMovieSelectQualityProfile,
  ActiveRadarrBlock::EditMoviePathInput,
  ActiveRadarrBlock::EditMovieTagsInput,
  ActiveRadarrBlock::EditMovieConfirmPrompt,
];
pub const MOVIE_DETAILS_BLOCKS: [ActiveRadarrBlock; 10] = [
  ActiveRadarrBlock::MovieDetails,
  ActiveRadarrBlock::MovieHistory,
  ActiveRadarrBlock::FileInfo,
  ActiveRadarrBlock::Cast,
  ActiveRadarrBlock::Crew,
  ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
  ActiveRadarrBlock::UpdateAndScanPrompt,
  ActiveRadarrBlock::ManualSearch,
  ActiveRadarrBlock::ManualSearchSortPrompt,
  ActiveRadarrBlock::ManualSearchConfirmPrompt,
];
pub const COLLECTION_DETAILS_BLOCKS: [ActiveRadarrBlock; 2] = [
  ActiveRadarrBlock::CollectionDetails,
  ActiveRadarrBlock::ViewMovieOverview,
];
pub const SEARCH_BLOCKS: [ActiveRadarrBlock; 2] = [
  ActiveRadarrBlock::SearchMovie,
  ActiveRadarrBlock::SearchCollection,
];
pub const FILTER_BLOCKS: [ActiveRadarrBlock; 2] = [
  ActiveRadarrBlock::FilterMovies,
  ActiveRadarrBlock::FilterCollections,
];
pub const DELETE_MOVIE_BLOCKS: [ActiveRadarrBlock; 4] = [
  ActiveRadarrBlock::DeleteMoviePrompt,
  ActiveRadarrBlock::DeleteMovieConfirmPrompt,
  ActiveRadarrBlock::DeleteMovieToggleDeleteFile,
  ActiveRadarrBlock::DeleteMovieToggleAddListExclusion,
];
pub const DELETE_MOVIE_SELECTION_BLOCKS: [ActiveRadarrBlock; 3] = [
  ActiveRadarrBlock::DeleteMovieToggleDeleteFile,
  ActiveRadarrBlock::DeleteMovieToggleAddListExclusion,
  ActiveRadarrBlock::DeleteMovieConfirmPrompt,
];

impl From<ActiveRadarrBlock> for Route {
  fn from(active_radarr_block: ActiveRadarrBlock) -> Route {
    Route::Radarr(active_radarr_block, None)
  }
}

impl From<(ActiveRadarrBlock, Option<ActiveRadarrBlock>)> for Route {
  fn from(value: (ActiveRadarrBlock, Option<ActiveRadarrBlock>)) -> Route {
    Route::Radarr(value.0, value.1)
  }
}

impl<'a> App<'a> {
  pub(super) async fn dispatch_by_radarr_block(&mut self, active_radarr_block: &ActiveRadarrBlock) {
    match active_radarr_block {
      ActiveRadarrBlock::Collections => {
        self
          .dispatch_network_event(RadarrEvent::GetCollections.into())
          .await;
      }
      ActiveRadarrBlock::CollectionDetails => {
        self.is_loading = true;
        self.populate_movie_collection_table().await;
        self.is_loading = false;
      }
      ActiveRadarrBlock::Downloads => {
        self
          .dispatch_network_event(RadarrEvent::GetDownloads.into())
          .await;
      }
      ActiveRadarrBlock::RootFolders => {
        self
          .dispatch_network_event(RadarrEvent::GetRootFolders.into())
          .await;
      }
      ActiveRadarrBlock::Movies => {
        self
          .dispatch_network_event(RadarrEvent::GetMovies.into())
          .await;
        self
          .dispatch_network_event(RadarrEvent::GetDownloads.into())
          .await;
      }
      ActiveRadarrBlock::AddMovieSearchResults => {
        self
          .dispatch_network_event(RadarrEvent::SearchNewMovie.into())
          .await;
      }
      ActiveRadarrBlock::MovieDetails | ActiveRadarrBlock::FileInfo => {
        self
          .dispatch_network_event(RadarrEvent::GetMovieDetails.into())
          .await;
      }
      ActiveRadarrBlock::MovieHistory => {
        self
          .dispatch_network_event(RadarrEvent::GetMovieHistory.into())
          .await;
      }
      ActiveRadarrBlock::Cast | ActiveRadarrBlock::Crew => {
        if self.data.radarr_data.movie_cast.items.is_empty()
          || self.data.radarr_data.movie_crew.items.is_empty()
        {
          self
            .dispatch_network_event(RadarrEvent::GetMovieCredits.into())
            .await;
        }
      }
      ActiveRadarrBlock::ManualSearch => {
        if self.data.radarr_data.movie_releases.items.is_empty() {
          self
            .dispatch_network_event(RadarrEvent::GetReleases.into())
            .await;
        }
      }
      _ => (),
    }

    self.check_for_prompt_action().await;
    self.reset_tick_count();
  }

  async fn check_for_prompt_action(&mut self) {
    if self.data.radarr_data.prompt_confirm {
      self.data.radarr_data.prompt_confirm = false;
      if let Some(radarr_event) = &self.data.radarr_data.prompt_confirm_action {
        self.dispatch_network_event((*radarr_event).into()).await;
        self.should_refresh = true;
        self.data.radarr_data.prompt_confirm_action = None;
      }
    }
  }

  pub(super) async fn radarr_on_tick(
    &mut self,
    active_radarr_block: ActiveRadarrBlock,
    is_first_render: bool,
  ) {
    if is_first_render {
      self
        .dispatch_network_event(RadarrEvent::GetQualityProfiles.into())
        .await;
      self
        .dispatch_network_event(RadarrEvent::GetTags.into())
        .await;
      self
        .dispatch_network_event(RadarrEvent::GetRootFolders.into())
        .await;
      self
        .dispatch_network_event(RadarrEvent::GetOverview.into())
        .await;
      self
        .dispatch_network_event(RadarrEvent::GetStatus.into())
        .await;
      self.dispatch_by_radarr_block(&active_radarr_block).await;
    }

    if self.should_refresh {
      self.dispatch_by_radarr_block(&active_radarr_block).await;
    }

    if self.is_routing || self.tick_count % self.tick_until_poll == 0 {
      self.dispatch_by_radarr_block(&active_radarr_block).await;
      self.refresh_metadata().await;
    }
  }

  async fn refresh_metadata(&mut self) {
    self
      .dispatch_network_event(RadarrEvent::GetQualityProfiles.into())
      .await;
    self
      .dispatch_network_event(RadarrEvent::GetTags.into())
      .await;
    self
      .dispatch_network_event(RadarrEvent::GetRootFolders.into())
      .await;
    self
      .dispatch_network_event(RadarrEvent::GetDownloads.into())
      .await;
  }

  async fn populate_movie_collection_table(&mut self) {
    let collection_movies = if !self.data.radarr_data.filtered_collections.items.is_empty() {
      self
        .data
        .radarr_data
        .filtered_collections
        .current_selection()
        .clone()
        .movies
        .unwrap_or_default()
    } else {
      self
        .data
        .radarr_data
        .collections
        .current_selection()
        .clone()
        .movies
        .unwrap_or_default()
    };
    self
      .data
      .radarr_data
      .collection_movies
      .set_items(collection_movies);
  }
}

#[cfg(test)]
#[macro_use]
pub mod radarr_test_utils {
  use crate::app::radarr::RadarrData;
  use crate::models::radarr_models::{
    AddMovieSearchResult, Collection, CollectionMovie, Credit, MinimumAvailability, Monitor, Movie,
    MovieHistoryItem, Release, ReleaseField, RootFolder,
  };
  use crate::models::ScrollableText;

  pub fn create_test_radarr_data<'a>() -> RadarrData<'a> {
    let mut radarr_data = RadarrData {
      is_searching: true,
      is_filtering: true,
      delete_movie_files: true,
      add_list_exclusion: true,
      search: "test search".to_owned().into(),
      filter: "test filter".to_owned().into(),
      edit_path: "test path".to_owned().into(),
      edit_tags: "usenet, test".to_owned().into(),
      edit_monitored: Some(true),
      edit_search_on_add: Some(true),
      file_details: "test file details".to_owned(),
      audio_details: "test audio details".to_owned(),
      video_details: "test video details".to_owned(),
      movie_details: ScrollableText::with_string("test movie details".to_owned()),
      ..RadarrData::default()
    };
    radarr_data
      .movie_history
      .set_items(vec![MovieHistoryItem::default()]);
    radarr_data.movie_cast.set_items(vec![Credit::default()]);
    radarr_data.movie_crew.set_items(vec![Credit::default()]);
    radarr_data
      .movie_releases
      .set_items(vec![Release::default()]);
    radarr_data.movie_info_tabs.index = 1;
    radarr_data.monitor_list.set_items(vec![Monitor::default()]);
    radarr_data
      .minimum_availability_list
      .set_items(vec![MinimumAvailability::default()]);
    radarr_data
      .quality_profile_list
      .set_items(vec![String::default()]);
    radarr_data
      .root_folder_list
      .set_items(vec![RootFolder::default()]);
    radarr_data
      .movie_releases_sort
      .set_items(vec![ReleaseField::default()]);
    radarr_data.sort_ascending = Some(true);
    radarr_data
      .filtered_movies
      .set_items(vec![Movie::default()]);
    radarr_data
      .filtered_collections
      .set_items(vec![Collection::default()]);
    radarr_data
      .add_searched_movies
      .set_items(vec![AddMovieSearchResult::default()]);
    radarr_data
      .collection_movies
      .set_items(vec![CollectionMovie::default()]);

    radarr_data
  }

  #[macro_export]
  macro_rules! assert_search_reset {
    ($radarr_data:expr) => {
      assert!(!$radarr_data.is_searching);
      assert!($radarr_data.search.text.is_empty());
      assert!($radarr_data.filter.text.is_empty());
      assert!($radarr_data.filtered_movies.items.is_empty());
      assert!($radarr_data.filtered_collections.items.is_empty());
      assert!($radarr_data.add_searched_movies.items.is_empty());
    };
  }

  #[macro_export]
  macro_rules! assert_edit_media_reset {
    ($radarr_data:expr) => {
      assert!($radarr_data.edit_monitored.is_none());
      assert!($radarr_data.edit_search_on_add.is_none());
      assert!($radarr_data.edit_path.text.is_empty());
      assert!($radarr_data.edit_tags.text.is_empty());
    };
  }

  #[macro_export]
  macro_rules! assert_filter_reset {
    ($radarr_data:expr) => {
      assert!(!$radarr_data.is_filtering);
      assert!($radarr_data.filter.text.is_empty());
      assert!($radarr_data.filtered_movies.items.is_empty());
      assert!($radarr_data.filtered_collections.items.is_empty());
    };
  }

  #[macro_export]
  macro_rules! assert_movie_info_tabs_reset {
    ($radarr_data:expr) => {
      assert!($radarr_data.file_details.is_empty());
      assert!($radarr_data.audio_details.is_empty());
      assert!($radarr_data.video_details.is_empty());
      assert!($radarr_data.movie_details.get_text().is_empty());
      assert!($radarr_data.movie_history.items.is_empty());
      assert!($radarr_data.movie_cast.items.is_empty());
      assert!($radarr_data.movie_crew.items.is_empty());
      assert!($radarr_data.movie_releases.items.is_empty());
      assert!($radarr_data.movie_releases_sort.items.is_empty());
      assert!($radarr_data.sort_ascending.is_none());
      assert_eq!($radarr_data.movie_info_tabs.index, 0);
    };
  }

  #[macro_export]
  macro_rules! assert_preferences_selections_reset {
    ($radarr_data:expr) => {
      assert!($radarr_data.monitor_list.items.is_empty());
      assert!($radarr_data.minimum_availability_list.items.is_empty());
      assert!($radarr_data.quality_profile_list.items.is_empty());
      assert!($radarr_data.root_folder_list.items.is_empty());
    };
  }
}

#[cfg(test)]
mod tests {
  mod radarr_data_tests {
    use bimap::BiMap;
    use chrono::{DateTime, Utc};
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;
    use serde_json::Number;
    use strum::IntoEnumIterator;

    use crate::app::radarr::radarr_test_utils::create_test_radarr_data;
    use crate::app::radarr::{ActiveRadarrBlock, RadarrData};
    use crate::models::radarr_models::{
      Collection, MinimumAvailability, Monitor, Movie, RootFolder,
    };
    use crate::models::Route;
    use crate::models::StatefulTable;
    use crate::models::{BlockSelectionState, HorizontallyScrollableText};

    #[test]
    fn test_from_tuple_to_route_with_context() {
      assert_eq!(
        Route::from((
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::AddMovieSearchResults)
        )),
        Route::Radarr(
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::AddMovieSearchResults)
        )
      );
    }

    #[test]
    fn test_reset_movie_collection_table() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_movie_collection_table();

      assert!(radarr_data.collection_movies.items.is_empty());
    }

    #[test]
    fn test_reset_delete_movie_preferences() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_delete_movie_preferences();

      assert!(!radarr_data.delete_movie_files);
      assert!(!radarr_data.add_list_exclusion);
    }

    #[test]
    fn test_reset_search() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_search();

      assert_search_reset!(radarr_data);
    }

    #[test]
    fn test_reset_filter() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_filter();

      assert_filter_reset!(radarr_data);
    }

    #[test]
    fn test_reset_movie_info_tabs() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_movie_info_tabs();

      assert_movie_info_tabs_reset!(radarr_data);
    }

    #[test]
    fn test_reset_add_edit_media_fields() {
      let mut radarr_data = RadarrData {
        edit_monitored: Some(true),
        edit_search_on_add: Some(true),
        edit_path: "test path".to_owned().into(),
        edit_tags: "test tag".to_owned().into(),
        ..RadarrData::default()
      };

      radarr_data.reset_add_edit_media_fields();

      assert_edit_media_reset!(radarr_data);
    }

    #[test]
    fn test_reset_preferences_selections() {
      let mut radarr_data = create_test_radarr_data();

      radarr_data.reset_preferences_selections();

      assert_preferences_selections_reset!(radarr_data);
    }

    #[test]
    fn test_populate_preferences_lists() {
      let root_folder = RootFolder {
        id: Number::from(1),
        path: "/nfs".to_owned(),
        accessible: true,
        free_space: Number::from(219902325555200u64),
        unmapped_folders: None,
      };
      let mut radarr_data = RadarrData {
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        ..RadarrData::default()
      };
      radarr_data
        .root_folders
        .set_items(vec![root_folder.clone()]);

      radarr_data.populate_preferences_lists();

      assert_eq!(
        radarr_data.monitor_list.items,
        Vec::from_iter(Monitor::iter())
      );
      assert_eq!(
        radarr_data.minimum_availability_list.items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        radarr_data.quality_profile_list.items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_eq!(radarr_data.root_folder_list.items, vec![root_folder]);
    }

    #[rstest]
    fn test_populate_edit_movie_fields(#[values(true, false)] test_filtered_movies: bool) {
      let mut radarr_data = RadarrData {
        edit_path: HorizontallyScrollableText::default(),
        edit_tags: HorizontallyScrollableText::default(),
        edit_monitored: None,
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        tags_map: BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]),
        filtered_movies: StatefulTable::default(),
        ..create_test_radarr_data()
      };
      let movie = Movie {
        path: "/nfs/movies/Test".to_owned(),
        monitored: true,
        quality_profile_id: Number::from(2222),
        minimum_availability: MinimumAvailability::Released,
        tags: vec![Number::from(1), Number::from(2)],
        ..Movie::default()
      };

      if test_filtered_movies {
        radarr_data.filtered_movies.set_items(vec![movie]);
      } else {
        radarr_data.movies.set_items(vec![movie]);
      }

      radarr_data.populate_edit_movie_fields();

      assert_eq!(
        radarr_data.minimum_availability_list.items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        radarr_data.minimum_availability_list.current_selection(),
        &MinimumAvailability::Released
      );
      assert_eq!(
        radarr_data.quality_profile_list.items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_str_eq!(
        radarr_data.quality_profile_list.current_selection(),
        "HD - 1080p"
      );
      assert_str_eq!(radarr_data.edit_path.text, "/nfs/movies/Test");
      assert_str_eq!(radarr_data.edit_tags.text, "usenet, test");
      assert_eq!(radarr_data.edit_monitored, Some(true));
    }

    #[rstest]
    fn test_populate_edit_collection_fields(
      #[values(true, false)] test_filtered_collections: bool,
    ) {
      let mut radarr_data = RadarrData {
        edit_path: HorizontallyScrollableText::default(),
        edit_monitored: None,
        edit_search_on_add: None,
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        filtered_collections: StatefulTable::default(),
        ..create_test_radarr_data()
      };
      let collection = Collection {
        root_folder_path: Some("/nfs/movies/Test".to_owned()),
        monitored: true,
        search_on_add: true,
        quality_profile_id: Number::from(2222),
        minimum_availability: MinimumAvailability::Released,
        ..Collection::default()
      };

      if test_filtered_collections {
        radarr_data.filtered_collections.set_items(vec![collection]);
      } else {
        radarr_data.collections.set_items(vec![collection]);
      }

      radarr_data.populate_edit_collection_fields();

      assert_eq!(
        radarr_data.minimum_availability_list.items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        radarr_data.minimum_availability_list.current_selection(),
        &MinimumAvailability::Released
      );
      assert_eq!(
        radarr_data.quality_profile_list.items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_str_eq!(
        radarr_data.quality_profile_list.current_selection(),
        "HD - 1080p"
      );
      assert_str_eq!(radarr_data.edit_path.text, "/nfs/movies/Test");
      assert_eq!(radarr_data.edit_monitored, Some(true));
      assert_eq!(radarr_data.edit_search_on_add, Some(true));
    }

    #[test]
    fn test_radarr_data_defaults() {
      let radarr_data = RadarrData::default();

      assert!(radarr_data.root_folders.items.is_empty());
      assert_eq!(radarr_data.disk_space_vec, Vec::new());
      assert!(radarr_data.version.is_empty());
      assert_eq!(radarr_data.start_time, <DateTime<Utc>>::default());
      assert!(radarr_data.movies.items.is_empty());
      assert!(radarr_data.add_searched_movies.items.is_empty());
      assert!(radarr_data.monitor_list.items.is_empty());
      assert!(radarr_data.minimum_availability_list.items.is_empty());
      assert!(radarr_data.quality_profile_list.items.is_empty());
      assert!(radarr_data.root_folder_list.items.is_empty());
      assert_eq!(radarr_data.selected_block, BlockSelectionState::default());
      assert!(radarr_data.filtered_movies.items.is_empty());
      assert!(radarr_data.downloads.items.is_empty());
      assert!(radarr_data.quality_profile_map.is_empty());
      assert!(radarr_data.tags_map.is_empty());
      assert!(radarr_data.file_details.is_empty());
      assert!(radarr_data.audio_details.is_empty());
      assert!(radarr_data.video_details.is_empty());
      assert!(radarr_data.movie_details.get_text().is_empty());
      assert!(radarr_data.movie_history.items.is_empty());
      assert!(radarr_data.movie_cast.items.is_empty());
      assert!(radarr_data.movie_crew.items.is_empty());
      assert!(radarr_data.movie_releases.items.is_empty());
      assert!(radarr_data.movie_releases_sort.items.is_empty());
      assert!(radarr_data.collections.items.is_empty());
      assert!(radarr_data.filtered_collections.items.is_empty());
      assert!(radarr_data.collection_movies.items.is_empty());
      assert!(radarr_data.prompt_confirm_action.is_none());
      assert!(radarr_data.search.text.is_empty());
      assert!(radarr_data.filter.text.is_empty());
      assert!(radarr_data.edit_path.text.is_empty());
      assert!(radarr_data.edit_tags.text.is_empty());
      assert!(radarr_data.edit_monitored.is_none());
      assert!(radarr_data.edit_search_on_add.is_none());
      assert!(radarr_data.sort_ascending.is_none());
      assert!(!radarr_data.is_searching);
      assert!(!radarr_data.is_filtering);
      assert!(!radarr_data.prompt_confirm);
      assert!(!radarr_data.delete_movie_files);
      assert!(!radarr_data.add_list_exclusion);

      assert_eq!(radarr_data.main_tabs.tabs.len(), 5);

      assert_str_eq!(radarr_data.main_tabs.tabs[0].title, "Library");
      assert_eq!(
        radarr_data.main_tabs.tabs[0].route,
        ActiveRadarrBlock::Movies.into()
      );
      assert!(radarr_data.main_tabs.tabs[0].help.is_empty());
      assert_eq!(radarr_data.main_tabs.tabs[0].contextual_help,
                 Some("<a> add | <e> edit | <del> delete | <s> search | <f> filter | <r> refresh | <u> update all | <enter> details | <esc> cancel filter"));

      assert_str_eq!(radarr_data.main_tabs.tabs[1].title, "Downloads");
      assert_eq!(
        radarr_data.main_tabs.tabs[1].route,
        ActiveRadarrBlock::Downloads.into()
      );
      assert!(radarr_data.main_tabs.tabs[1].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[1].contextual_help,
        Some("<r> refresh | <del> delete")
      );

      assert_str_eq!(radarr_data.main_tabs.tabs[2].title, "Collections");
      assert_eq!(
        radarr_data.main_tabs.tabs[2].route,
        ActiveRadarrBlock::Collections.into()
      );
      assert!(radarr_data.main_tabs.tabs[2].help.is_empty());
      assert_eq!(radarr_data.main_tabs.tabs[2].contextual_help,
                 Some("<s> search | <e> edit | <f> filter | <r> refresh | <u> update all | <enter> details | <esc> cancel filter"));

      assert_str_eq!(radarr_data.main_tabs.tabs[3].title, "Root Folders");
      assert_eq!(
        radarr_data.main_tabs.tabs[3].route,
        ActiveRadarrBlock::RootFolders.into()
      );
      assert!(radarr_data.main_tabs.tabs[3].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[3].contextual_help,
        Some("<a> add | <del> delete | <r> refresh")
      );

      assert_str_eq!(radarr_data.main_tabs.tabs[4].title, "System");
      assert_eq!(
        radarr_data.main_tabs.tabs[4].route,
        ActiveRadarrBlock::System.into()
      );
      assert!(radarr_data.main_tabs.tabs[4].help.is_empty());
      assert_eq!(
        radarr_data.main_tabs.tabs[4].contextual_help,
        Some("<enter> select menu item | <esc> go back to menu selection")
      );

      assert_eq!(radarr_data.movie_info_tabs.tabs.len(), 6);

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[0].title, "Details");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[0].route,
        ActiveRadarrBlock::MovieDetails.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[0].help,
        "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close"
      );
      assert!(radarr_data.movie_info_tabs.tabs[0]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[1].title, "History");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[1].route,
        ActiveRadarrBlock::MovieHistory.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[1].help,
        "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close"
      );
      assert!(radarr_data.movie_info_tabs.tabs[1]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[2].title, "File");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[2].route,
        ActiveRadarrBlock::FileInfo.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[2].help,
        "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close"
      );
      assert!(radarr_data.movie_info_tabs.tabs[2]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[3].title, "Cast");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[3].route,
        ActiveRadarrBlock::Cast.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[3].help,
        "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close"
      );
      assert!(radarr_data.movie_info_tabs.tabs[3]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[4].title, "Crew");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[4].route,
        ActiveRadarrBlock::Crew.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[4].help,
        "<r> refresh | <u> update | <e> edit | <s> auto search | <esc> close"
      );
      assert!(radarr_data.movie_info_tabs.tabs[4]
        .contextual_help
        .is_none());

      assert_str_eq!(radarr_data.movie_info_tabs.tabs[5].title, "Manual Search");
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[5].route,
        ActiveRadarrBlock::ManualSearch.into()
      );
      assert_str_eq!(
        radarr_data.movie_info_tabs.tabs[5].help,
        "<r> refresh | <u> update | <e> edit | <o> sort | <s> auto search | <esc> close"
      );
      assert_eq!(
        radarr_data.movie_info_tabs.tabs[5].contextual_help,
        Some("<enter> details")
      );
    }
  }

  mod active_radarr_block_tests {
    use pretty_assertions::assert_eq;

    use crate::app::radarr::{
      ActiveRadarrBlock, ADD_MOVIE_SELECTION_BLOCKS, DELETE_MOVIE_SELECTION_BLOCKS,
      EDIT_COLLECTION_SELECTION_BLOCKS, EDIT_MOVIE_SELECTION_BLOCKS,
    };

    #[test]
    fn test_add_movie_prompt_block_order() {
      let mut add_movie_block_iter = ADD_MOVIE_SELECTION_BLOCKS.iter();

      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectRootFolder
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectMonitor
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieSelectQualityProfile
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieTagsInput
      );
      assert_eq!(
        add_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::AddMovieConfirmPrompt
      );
    }

    #[test]
    fn test_edit_movie_prompt_block_order() {
      let mut edit_movie_block_iter = EDIT_MOVIE_SELECTION_BLOCKS.iter();

      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieToggleMonitored
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieSelectMinimumAvailability
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieSelectQualityProfile
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMoviePathInput
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieTagsInput
      );
      assert_eq!(
        edit_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditMovieConfirmPrompt
      );
    }

    #[test]
    fn test_edit_collection_prompt_block_order() {
      let mut edit_collection_block_iter = EDIT_COLLECTION_SELECTION_BLOCKS.iter();

      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionToggleMonitored
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionSelectMinimumAvailability
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionSelectQualityProfile
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionToggleSearchOnAdd
      );
      assert_eq!(
        edit_collection_block_iter.next().unwrap(),
        &ActiveRadarrBlock::EditCollectionConfirmPrompt
      );
    }

    #[test]
    fn test_delete_movie_prompt_block_order() {
      let mut delete_movie_block_iter = DELETE_MOVIE_SELECTION_BLOCKS.iter();
      assert_eq!(
        delete_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::DeleteMovieToggleDeleteFile
      );
      assert_eq!(
        delete_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::DeleteMovieToggleAddListExclusion
      );
      assert_eq!(
        delete_movie_block_iter.next().unwrap(),
        &ActiveRadarrBlock::DeleteMovieConfirmPrompt
      );
    }
  }

  mod radarr_tests {
    use pretty_assertions::assert_eq;
    use tokio::sync::mpsc;

    use crate::app::radarr::ActiveRadarrBlock;
    use crate::app::App;
    use crate::models::radarr_models::{Collection, CollectionMovie, Credit, Release};
    use crate::models::StatefulTable;
    use crate::network::radarr_network::RadarrEvent;
    use crate::network::NetworkEvent;

    #[tokio::test]
    async fn test_dispatch_by_collections_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::Collections)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetCollections.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_collection_details_block() {
      let (mut app, _) = construct_app_unit();

      app.data.radarr_data.collections.set_items(vec![Collection {
        movies: Some(vec![CollectionMovie::default()]),
        ..Collection::default()
      }]);

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::CollectionDetails)
        .await;

      assert!(!app.is_loading);
      assert!(!app.data.radarr_data.collection_movies.items.is_empty());
      assert_eq!(app.tick_count, 0);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_dispatch_by_collection_details_block_with_add_movie() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::AddMovie);

      app.data.radarr_data.collections.set_items(vec![Collection {
        movies: Some(vec![CollectionMovie::default()]),
        ..Collection::default()
      }]);

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::CollectionDetails)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::AddMovie.into()
      );
      assert!(!app.data.radarr_data.collection_movies.items.is_empty());
      assert_eq!(app.tick_count, 0);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_dispatch_by_downloads_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::Downloads)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_root_folders_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::RootFolders)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetRootFolders.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_movies_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::Movies)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovies.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_add_movie_search_results_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::AddMovieSearchResults)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::SearchNewMovie.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_movie_details_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::MovieDetails)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieDetails.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_file_info_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::FileInfo)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieDetails.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_movie_history_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::MovieHistory)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetMovieHistory.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app.data.radarr_data.movie_cast = StatefulTable::default();
        app.data.radarr_data.movie_crew = StatefulTable::default();

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(app.is_loading);
        assert_eq!(
          sync_network_rx.recv().await.unwrap(),
          RadarrEvent::GetMovieCredits.into()
        );
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks_movie_cast_non_empty() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app
          .data
          .radarr_data
          .movie_cast
          .set_items(vec![Credit::default()]);

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(app.is_loading);
        assert_eq!(
          sync_network_rx.recv().await.unwrap(),
          RadarrEvent::GetMovieCredits.into()
        );
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks_movie_crew_non_empty() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app
          .data
          .radarr_data
          .movie_crew
          .set_items(vec![Credit::default()]);

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(app.is_loading);
        assert_eq!(
          sync_network_rx.recv().await.unwrap(),
          RadarrEvent::GetMovieCredits.into()
        );
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_cast_crew_blocks_cast_and_crew_non_empty() {
      let mut app = App::default();

      for active_radarr_block in &[ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew] {
        app
          .data
          .radarr_data
          .movie_cast
          .set_items(vec![Credit::default()]);
        app
          .data
          .radarr_data
          .movie_crew
          .set_items(vec![Credit::default()]);

        app.dispatch_by_radarr_block(active_radarr_block).await;

        assert!(!app.is_loading);
        assert!(!app.data.radarr_data.prompt_confirm);
        assert_eq!(app.tick_count, 0);
      }
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_search_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetReleases.into()
      );
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_search_block_movie_releases_non_empty() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movie_releases
        .set_items(vec![Release::default()]);

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
        .await;

      assert!(!app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_search_block_is_loading() {
      let mut app = App {
        is_loading: true,
        ..App::default()
      };

      app
        .dispatch_by_radarr_block(&ActiveRadarrBlock::ManualSearch)
        .await;

      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_check_for_prompt_action_no_prompt_confirm() {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = false;

      app.check_for_prompt_action().await;

      assert!(!app.data.radarr_data.prompt_confirm);
      assert!(!app.should_refresh);
    }

    #[tokio::test]
    async fn test_check_for_prompt_action() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::GetStatus);

      app.check_for_prompt_action().await;

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetStatus.into()
      );
      assert!(app.should_refresh);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[tokio::test]
    async fn test_radarr_refresh_metadata() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.is_routing = true;

      app.refresh_metadata().await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_first_render() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app.radarr_on_tick(ActiveRadarrBlock::Downloads, true).await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetOverview.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetStatus.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_not_routing() {
      let mut app = App::default();

      app
        .radarr_on_tick(ActiveRadarrBlock::Downloads, false)
        .await;

      assert!(!app.is_routing);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_routing() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.is_routing = true;

      app
        .radarr_on_tick(ActiveRadarrBlock::Downloads, false)
        .await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_should_refresh() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.should_refresh = true;

      app
        .radarr_on_tick(ActiveRadarrBlock::Downloads, false)
        .await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
      assert!(app.should_refresh);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_radarr_on_tick_network_tick_frequency() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.tick_count = 2;
      app.tick_until_poll = 2;

      app
        .radarr_on_tick(ActiveRadarrBlock::Downloads, false)
        .await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        RadarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_populate_movie_collection_table_unfiltered() {
      let mut app = App::default();
      app.data.radarr_data.collections.set_items(vec![Collection {
        movies: Some(vec![CollectionMovie::default()]),
        ..Collection::default()
      }]);

      app.populate_movie_collection_table().await;

      assert!(!app.data.radarr_data.collection_movies.items.is_empty());
    }

    #[tokio::test]
    async fn test_populate_movie_collection_table_filtered() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_collections
        .set_items(vec![Collection {
          movies: Some(vec![CollectionMovie::default()]),
          ..Collection::default()
        }]);

      app.populate_movie_collection_table().await;

      assert!(!app.data.radarr_data.collection_movies.items.is_empty());
    }

    fn construct_app_unit<'a>() -> (App<'a>, mpsc::Receiver<NetworkEvent>) {
      let (sync_network_tx, sync_network_rx) = mpsc::channel::<NetworkEvent>(500);
      let mut app = App {
        network_tx: Some(sync_network_tx),
        tick_count: 1,
        ..App::default()
      };
      app.data.radarr_data.prompt_confirm = true;

      (app, sync_network_rx)
    }
  }
}
