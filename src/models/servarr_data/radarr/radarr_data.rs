use crate::app::context_clues::{
  build_context_clue_string, BLOCKLIST_CONTEXT_CLUES, DOWNLOADS_CONTEXT_CLUES,
  INDEXERS_CONTEXT_CLUES, ROOT_FOLDERS_CONTEXT_CLUES, SYSTEM_CONTEXT_CLUES,
};
use crate::app::radarr::radarr_context_clues::{
  COLLECTIONS_CONTEXT_CLUES, LIBRARY_CONTEXT_CLUES, MANUAL_MOVIE_SEARCH_CONTEXTUAL_CONTEXT_CLUES,
  MANUAL_MOVIE_SEARCH_CONTEXT_CLUES, MOVIE_DETAILS_CONTEXT_CLUES,
};
use crate::models::radarr_models::{
  AddMovieSearchResult, BlocklistItem, Collection, CollectionMovie, DownloadRecord,
  IndexerSettings, Movie, RadarrTask,
};
use crate::models::servarr_data::modals::{EditIndexerModal, IndexerTestResultModalItem};
use crate::models::servarr_data::radarr::modals::{
  AddMovieModal, EditCollectionModal, EditMovieModal, MovieDetailsModal,
};
use crate::models::servarr_models::{DiskSpace, Indexer, QueueEvent, RootFolder};
use crate::models::stateful_list::StatefulList;
use crate::models::stateful_table::StatefulTable;
use crate::models::{
  BlockSelectionState, HorizontallyScrollableText, Route, ScrollableText, TabRoute, TabState,
};
use crate::network::radarr_network::RadarrEvent;
use bimap::BiMap;
use chrono::{DateTime, Utc};
use strum::EnumIter;

#[cfg(test)]
#[path = "radarr_data_tests.rs"]
mod radarr_data_tests;

#[cfg(test)]
#[path = "radarr_test_utils.rs"]
pub mod radarr_test_utils;

pub struct RadarrData<'a> {
  pub root_folders: StatefulTable<RootFolder>,
  pub disk_space_vec: Vec<DiskSpace>,
  pub version: String,
  pub start_time: DateTime<Utc>,
  pub movies: StatefulTable<Movie>,
  pub selected_block: BlockSelectionState<'a, ActiveRadarrBlock>,
  pub downloads: StatefulTable<DownloadRecord>,
  pub indexers: StatefulTable<Indexer>,
  pub blocklist: StatefulTable<BlocklistItem>,
  pub quality_profile_map: BiMap<i64, String>,
  pub tags_map: BiMap<i64, String>,
  pub collections: StatefulTable<Collection>,
  pub collection_movies: StatefulTable<CollectionMovie>,
  pub logs: StatefulList<HorizontallyScrollableText>,
  pub log_details: StatefulList<HorizontallyScrollableText>,
  pub tasks: StatefulTable<RadarrTask>,
  pub queued_events: StatefulTable<QueueEvent>,
  pub updates: ScrollableText,
  pub main_tabs: TabState,
  pub movie_info_tabs: TabState,
  pub add_movie_search: Option<HorizontallyScrollableText>,
  pub add_movie_modal: Option<AddMovieModal>,
  pub add_searched_movies: Option<StatefulTable<AddMovieSearchResult>>,
  pub edit_movie_modal: Option<EditMovieModal>,
  pub edit_collection_modal: Option<EditCollectionModal>,
  pub edit_indexer_modal: Option<EditIndexerModal>,
  pub edit_root_folder: Option<HorizontallyScrollableText>,
  pub indexer_settings: Option<IndexerSettings>,
  pub indexer_test_errors: Option<String>,
  pub indexer_test_all_results: Option<StatefulTable<IndexerTestResultModalItem>>,
  pub movie_details_modal: Option<MovieDetailsModal>,
  pub prompt_confirm: bool,
  pub prompt_confirm_action: Option<RadarrEvent>,
  pub delete_movie_files: bool,
  pub add_list_exclusion: bool,
}

impl RadarrData<'_> {
  pub fn reset_delete_movie_preferences(&mut self) {
    self.delete_movie_files = false;
    self.add_list_exclusion = false;
  }

  pub fn reset_movie_info_tabs(&mut self) {
    self.movie_details_modal = None;
    self.movie_info_tabs.index = 0;
  }
}

impl<'a> Default for RadarrData<'a> {
  fn default() -> RadarrData<'a> {
    RadarrData {
      root_folders: StatefulTable::default(),
      disk_space_vec: Vec::new(),
      version: String::new(),
      start_time: DateTime::default(),
      movies: StatefulTable::default(),
      selected_block: BlockSelectionState::default(),
      downloads: StatefulTable::default(),
      indexers: StatefulTable::default(),
      blocklist: StatefulTable::default(),
      quality_profile_map: BiMap::default(),
      tags_map: BiMap::default(),
      collections: StatefulTable::default(),
      collection_movies: StatefulTable::default(),
      logs: StatefulList::default(),
      log_details: StatefulList::default(),
      tasks: StatefulTable::default(),
      queued_events: StatefulTable::default(),
      updates: ScrollableText::default(),
      add_movie_search: None,
      add_movie_modal: None,
      add_searched_movies: None,
      edit_movie_modal: None,
      edit_collection_modal: None,
      edit_indexer_modal: None,
      edit_root_folder: None,
      indexer_settings: None,
      indexer_test_errors: None,
      indexer_test_all_results: None,
      movie_details_modal: None,
      prompt_confirm: false,
      prompt_confirm_action: None,
      delete_movie_files: false,
      add_list_exclusion: false,
      main_tabs: TabState::new(vec![
        TabRoute {
          title: "Library".to_string(),
          route: ActiveRadarrBlock::Movies.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&LIBRARY_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "Collections".to_string(),
          route: ActiveRadarrBlock::Collections.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&COLLECTIONS_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "Downloads".to_string(),
          route: ActiveRadarrBlock::Downloads.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&DOWNLOADS_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "Blocklist".to_string(),
          route: ActiveRadarrBlock::Blocklist.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&BLOCKLIST_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "Root Folders".to_string(),
          route: ActiveRadarrBlock::RootFolders.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&ROOT_FOLDERS_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "Indexers".to_string(),
          route: ActiveRadarrBlock::Indexers.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&INDEXERS_CONTEXT_CLUES)),
          config: None,
        },
        TabRoute {
          title: "System".to_string(),
          route: ActiveRadarrBlock::System.into(),
          help: String::new(),
          contextual_help: Some(build_context_clue_string(&SYSTEM_CONTEXT_CLUES)),
          config: None,
        },
      ]),
      movie_info_tabs: TabState::new(vec![
        TabRoute {
          title: "Details".to_string(),
          route: ActiveRadarrBlock::MovieDetails.into(),
          help: build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
          config: None,
        },
        TabRoute {
          title: "History".to_string(),
          route: ActiveRadarrBlock::MovieHistory.into(),
          help: build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
          config: None,
        },
        TabRoute {
          title: "File".to_string(),
          route: ActiveRadarrBlock::FileInfo.into(),
          help: build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
          config: None,
        },
        TabRoute {
          title: "Cast".to_string(),
          route: ActiveRadarrBlock::Cast.into(),
          help: build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
          config: None,
        },
        TabRoute {
          title: "Crew".to_string(),
          route: ActiveRadarrBlock::Crew.into(),
          help: build_context_clue_string(&MOVIE_DETAILS_CONTEXT_CLUES),
          contextual_help: None,
          config: None,
        },
        TabRoute {
          title: "Manual Search".to_string(),
          route: ActiveRadarrBlock::ManualSearch.into(),
          help: build_context_clue_string(&MANUAL_MOVIE_SEARCH_CONTEXT_CLUES),
          contextual_help: Some(build_context_clue_string(
            &MANUAL_MOVIE_SEARCH_CONTEXTUAL_CONTEXT_CLUES,
          )),
          config: None,
        },
      ]),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
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
  Blocklist,
  BlocklistClearAllItemsPrompt,
  BlocklistItemDetails,
  BlocklistSortPrompt,
  Collections,
  CollectionsSortPrompt,
  CollectionDetails,
  Cast,
  Crew,
  DeleteBlocklistItemPrompt,
  DeleteDownloadPrompt,
  DeleteIndexerPrompt,
  DeleteMoviePrompt,
  DeleteMovieConfirmPrompt,
  DeleteMovieToggleDeleteFile,
  DeleteMovieToggleAddListExclusion,
  DeleteRootFolderPrompt,
  Downloads,
  EditCollectionPrompt,
  EditCollectionConfirmPrompt,
  EditCollectionRootFolderPathInput,
  EditCollectionSelectMinimumAvailability,
  EditCollectionSelectQualityProfile,
  EditCollectionToggleSearchOnAdd,
  EditCollectionToggleMonitored,
  EditIndexerPrompt,
  EditIndexerConfirmPrompt,
  EditIndexerApiKeyInput,
  EditIndexerNameInput,
  EditIndexerSeedRatioInput,
  EditIndexerToggleEnableRss,
  EditIndexerToggleEnableAutomaticSearch,
  EditIndexerToggleEnableInteractiveSearch,
  EditIndexerPriorityInput,
  EditIndexerUrlInput,
  EditIndexerTagsInput,
  EditMoviePrompt,
  EditMovieConfirmPrompt,
  EditMoviePathInput,
  EditMovieSelectMinimumAvailability,
  EditMovieSelectQualityProfile,
  EditMovieTagsInput,
  EditMovieToggleMonitored,
  FileInfo,
  FilterCollections,
  FilterCollectionsError,
  FilterMovies,
  FilterMoviesError,
  Indexers,
  AllIndexerSettingsPrompt,
  IndexerSettingsAvailabilityDelayInput,
  IndexerSettingsConfirmPrompt,
  IndexerSettingsMaximumSizeInput,
  IndexerSettingsMinimumAgeInput,
  IndexerSettingsRetentionInput,
  IndexerSettingsRssSyncIntervalInput,
  IndexerSettingsToggleAllowHardcodedSubs,
  IndexerSettingsTogglePreferIndexerFlags,
  IndexerSettingsWhitelistedSubtitleTagsInput,
  ManualSearch,
  ManualSearchSortPrompt,
  ManualSearchConfirmPrompt,
  MovieDetails,
  MovieHistory,
  #[default]
  Movies,
  MoviesSortPrompt,
  RootFolders,
  System,
  SystemLogs,
  SystemQueuedEvents,
  SystemTasks,
  SystemTaskStartConfirmPrompt,
  SystemUpdates,
  TestIndexer,
  TestAllIndexers,
  UpdateAndScanPrompt,
  UpdateAllCollectionsPrompt,
  UpdateAllMoviesPrompt,
  UpdateDownloadsPrompt,
  SearchCollection,
  SearchCollectionError,
  SearchMovie,
  SearchMovieError,
  ViewMovieOverview,
}

pub static LIBRARY_BLOCKS: [ActiveRadarrBlock; 7] = [
  ActiveRadarrBlock::Movies,
  ActiveRadarrBlock::MoviesSortPrompt,
  ActiveRadarrBlock::SearchMovie,
  ActiveRadarrBlock::SearchMovieError,
  ActiveRadarrBlock::FilterMovies,
  ActiveRadarrBlock::FilterMoviesError,
  ActiveRadarrBlock::UpdateAllMoviesPrompt,
];
pub static COLLECTIONS_BLOCKS: [ActiveRadarrBlock; 7] = [
  ActiveRadarrBlock::Collections,
  ActiveRadarrBlock::CollectionsSortPrompt,
  ActiveRadarrBlock::SearchCollection,
  ActiveRadarrBlock::SearchCollectionError,
  ActiveRadarrBlock::FilterCollections,
  ActiveRadarrBlock::FilterCollectionsError,
  ActiveRadarrBlock::UpdateAllCollectionsPrompt,
];
pub static INDEXERS_BLOCKS: [ActiveRadarrBlock; 3] = [
  ActiveRadarrBlock::DeleteIndexerPrompt,
  ActiveRadarrBlock::Indexers,
  ActiveRadarrBlock::TestIndexer,
];
pub static ROOT_FOLDERS_BLOCKS: [ActiveRadarrBlock; 3] = [
  ActiveRadarrBlock::RootFolders,
  ActiveRadarrBlock::AddRootFolderPrompt,
  ActiveRadarrBlock::DeleteRootFolderPrompt,
];
pub static BLOCKLIST_BLOCKS: [ActiveRadarrBlock; 5] = [
  ActiveRadarrBlock::Blocklist,
  ActiveRadarrBlock::BlocklistItemDetails,
  ActiveRadarrBlock::DeleteBlocklistItemPrompt,
  ActiveRadarrBlock::BlocklistClearAllItemsPrompt,
  ActiveRadarrBlock::BlocklistSortPrompt,
];
pub static ADD_MOVIE_BLOCKS: [ActiveRadarrBlock; 10] = [
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
pub const ADD_MOVIE_SELECTION_BLOCKS: &[&[ActiveRadarrBlock]] = &[
  &[ActiveRadarrBlock::AddMovieSelectRootFolder],
  &[ActiveRadarrBlock::AddMovieSelectMonitor],
  &[ActiveRadarrBlock::AddMovieSelectMinimumAvailability],
  &[ActiveRadarrBlock::AddMovieSelectQualityProfile],
  &[ActiveRadarrBlock::AddMovieTagsInput],
  &[ActiveRadarrBlock::AddMovieConfirmPrompt],
];
pub static EDIT_COLLECTION_BLOCKS: [ActiveRadarrBlock; 7] = [
  ActiveRadarrBlock::EditCollectionPrompt,
  ActiveRadarrBlock::EditCollectionConfirmPrompt,
  ActiveRadarrBlock::EditCollectionRootFolderPathInput,
  ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
  ActiveRadarrBlock::EditCollectionSelectQualityProfile,
  ActiveRadarrBlock::EditCollectionToggleSearchOnAdd,
  ActiveRadarrBlock::EditCollectionToggleMonitored,
];
pub const EDIT_COLLECTION_SELECTION_BLOCKS: &[&[ActiveRadarrBlock]] = &[
  &[ActiveRadarrBlock::EditCollectionToggleMonitored],
  &[ActiveRadarrBlock::EditCollectionSelectMinimumAvailability],
  &[ActiveRadarrBlock::EditCollectionSelectQualityProfile],
  &[ActiveRadarrBlock::EditCollectionRootFolderPathInput],
  &[ActiveRadarrBlock::EditCollectionToggleSearchOnAdd],
  &[ActiveRadarrBlock::EditCollectionConfirmPrompt],
];
pub static EDIT_MOVIE_BLOCKS: [ActiveRadarrBlock; 7] = [
  ActiveRadarrBlock::EditMoviePrompt,
  ActiveRadarrBlock::EditMovieConfirmPrompt,
  ActiveRadarrBlock::EditMoviePathInput,
  ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
  ActiveRadarrBlock::EditMovieSelectQualityProfile,
  ActiveRadarrBlock::EditMovieTagsInput,
  ActiveRadarrBlock::EditMovieToggleMonitored,
];
pub const EDIT_MOVIE_SELECTION_BLOCKS: &[&[ActiveRadarrBlock]] = &[
  &[ActiveRadarrBlock::EditMovieToggleMonitored],
  &[ActiveRadarrBlock::EditMovieSelectMinimumAvailability],
  &[ActiveRadarrBlock::EditMovieSelectQualityProfile],
  &[ActiveRadarrBlock::EditMoviePathInput],
  &[ActiveRadarrBlock::EditMovieTagsInput],
  &[ActiveRadarrBlock::EditMovieConfirmPrompt],
];
pub static DOWNLOADS_BLOCKS: [ActiveRadarrBlock; 3] = [
  ActiveRadarrBlock::Downloads,
  ActiveRadarrBlock::DeleteDownloadPrompt,
  ActiveRadarrBlock::UpdateDownloadsPrompt,
];
pub static MOVIE_DETAILS_BLOCKS: [ActiveRadarrBlock; 10] = [
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
pub static COLLECTION_DETAILS_BLOCKS: [ActiveRadarrBlock; 2] = [
  ActiveRadarrBlock::CollectionDetails,
  ActiveRadarrBlock::ViewMovieOverview,
];
pub static DELETE_MOVIE_BLOCKS: [ActiveRadarrBlock; 4] = [
  ActiveRadarrBlock::DeleteMoviePrompt,
  ActiveRadarrBlock::DeleteMovieConfirmPrompt,
  ActiveRadarrBlock::DeleteMovieToggleDeleteFile,
  ActiveRadarrBlock::DeleteMovieToggleAddListExclusion,
];
pub const DELETE_MOVIE_SELECTION_BLOCKS: &[&[ActiveRadarrBlock]] = &[
  &[ActiveRadarrBlock::DeleteMovieToggleDeleteFile],
  &[ActiveRadarrBlock::DeleteMovieToggleAddListExclusion],
  &[ActiveRadarrBlock::DeleteMovieConfirmPrompt],
];
pub static EDIT_INDEXER_BLOCKS: [ActiveRadarrBlock; 11] = [
  ActiveRadarrBlock::EditIndexerPrompt,
  ActiveRadarrBlock::EditIndexerConfirmPrompt,
  ActiveRadarrBlock::EditIndexerApiKeyInput,
  ActiveRadarrBlock::EditIndexerNameInput,
  ActiveRadarrBlock::EditIndexerSeedRatioInput,
  ActiveRadarrBlock::EditIndexerToggleEnableRss,
  ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch,
  ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch,
  ActiveRadarrBlock::EditIndexerPriorityInput,
  ActiveRadarrBlock::EditIndexerUrlInput,
  ActiveRadarrBlock::EditIndexerTagsInput,
];
pub const EDIT_INDEXER_TORRENT_SELECTION_BLOCKS: &[&[ActiveRadarrBlock]] = &[
  &[
    ActiveRadarrBlock::EditIndexerNameInput,
    ActiveRadarrBlock::EditIndexerUrlInput,
  ],
  &[
    ActiveRadarrBlock::EditIndexerToggleEnableRss,
    ActiveRadarrBlock::EditIndexerApiKeyInput,
  ],
  &[
    ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch,
    ActiveRadarrBlock::EditIndexerSeedRatioInput,
  ],
  &[
    ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch,
    ActiveRadarrBlock::EditIndexerTagsInput,
  ],
  &[
    ActiveRadarrBlock::EditIndexerPriorityInput,
    ActiveRadarrBlock::EditIndexerConfirmPrompt,
  ],
  &[
    ActiveRadarrBlock::EditIndexerConfirmPrompt,
    ActiveRadarrBlock::EditIndexerConfirmPrompt,
  ],
];
pub const EDIT_INDEXER_NZB_SELECTION_BLOCKS: &[&[ActiveRadarrBlock]] = &[
  &[
    ActiveRadarrBlock::EditIndexerNameInput,
    ActiveRadarrBlock::EditIndexerUrlInput,
  ],
  &[
    ActiveRadarrBlock::EditIndexerToggleEnableRss,
    ActiveRadarrBlock::EditIndexerApiKeyInput,
  ],
  &[
    ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch,
    ActiveRadarrBlock::EditIndexerTagsInput,
  ],
  &[
    ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch,
    ActiveRadarrBlock::EditIndexerPriorityInput,
  ],
  &[
    ActiveRadarrBlock::EditIndexerConfirmPrompt,
    ActiveRadarrBlock::EditIndexerConfirmPrompt,
  ],
];
pub static INDEXER_SETTINGS_BLOCKS: [ActiveRadarrBlock; 10] = [
  ActiveRadarrBlock::AllIndexerSettingsPrompt,
  ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
  ActiveRadarrBlock::IndexerSettingsConfirmPrompt,
  ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
  ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
  ActiveRadarrBlock::IndexerSettingsRetentionInput,
  ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
  ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs,
  ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags,
  ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
];
pub const INDEXER_SETTINGS_SELECTION_BLOCKS: &[&[ActiveRadarrBlock]] = &[
  &[
    ActiveRadarrBlock::IndexerSettingsMinimumAgeInput,
    ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput,
  ],
  &[
    ActiveRadarrBlock::IndexerSettingsRetentionInput,
    ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput,
  ],
  &[
    ActiveRadarrBlock::IndexerSettingsMaximumSizeInput,
    ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput,
  ],
  &[
    ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags,
    ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs,
  ],
  &[
    ActiveRadarrBlock::IndexerSettingsConfirmPrompt,
    ActiveRadarrBlock::IndexerSettingsConfirmPrompt,
  ],
];
pub static SYSTEM_DETAILS_BLOCKS: [ActiveRadarrBlock; 5] = [
  ActiveRadarrBlock::SystemLogs,
  ActiveRadarrBlock::SystemQueuedEvents,
  ActiveRadarrBlock::SystemTasks,
  ActiveRadarrBlock::SystemTaskStartConfirmPrompt,
  ActiveRadarrBlock::SystemUpdates,
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
