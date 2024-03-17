#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use ratatui::style::Style;
  use ratatui::text::Text;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, MOVIE_DETAILS_BLOCKS};
  use crate::ui::radarr_ui::library::movie_details_ui::{decorate_peer_style, MovieDetailsUi, style_from_download_status};
  use crate::ui::DrawUi;
  use crate::ui::styles::ManagarrStyle;

  #[test]
  fn test_movie_details_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if MOVIE_DETAILS_BLOCKS.contains(&active_radarr_block) {
        assert!(MovieDetailsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!MovieDetailsUi::accepts(active_radarr_block.into()));
      }
    });
  }
  
  #[rstest]
  #[case("Downloading", true, "", Style::new().downloading())]
  #[case("Downloaded", true, "", Style::new().downloaded())]
  #[case("Awaiting Import", true, "", Style::new().awaiting_import())]
  #[case("Missing", false, "", Style::new().unmonitored_missing())]
  #[case("Missing", false, "", Style::new().unmonitored_missing())]
  #[case("Missing", true, "released", Style::new().missing())]
  #[case("", true, "", Style::new().downloaded())]
  fn test_style_from_download_status(
    #[case] download_status: &str,
    #[case] is_monitored: bool,
    #[case] movie_status: &str,
    #[case] expected_style: Style,
  ) {
    assert_eq!(style_from_download_status(download_status, is_monitored, movie_status.to_owned()), expected_style);
  }
  
  #[rstest]
  #[case(0, 0, PeerStyle::Failure)]
  #[case(1, 2, PeerStyle::Warning)]
  #[case(4, 2, PeerStyle::Success)]
  fn test_decorate_peer_style(
    #[case] seeders: u64,
    #[case] leechers: u64,
    #[case] expected_style: PeerStyle,
  ) {
    let text = Text::from("test");
    match expected_style {
      PeerStyle::Failure => assert_eq!(decorate_peer_style(seeders, leechers, text.clone()), text.failure()),
      PeerStyle::Warning => assert_eq!(decorate_peer_style(seeders, leechers, text.clone()), text.warning()),
      PeerStyle::Success => assert_eq!(decorate_peer_style(seeders, leechers, text.clone()), text.success()),
    }
  }
  
  enum PeerStyle {
    Failure,
    Warning,
    Success,
  }
}
