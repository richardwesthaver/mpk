//! MPK_HTTP
pub use mpk_config::ClientConfig;
pub use reqwest::Client;

mod err;
pub use err::{Error, Result};

pub mod coverartarchive;
pub mod freesound;
pub mod musicbrainz;

pub const USER_AGENT: &str =
  concat!("mpk/", env!("CARGO_PKG_VERSION"), " (https://rwest.io)");

pub trait ClientExt {}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use mpk_config::Config;

  use super::*;

  fn fs_client() -> freesound::FreeSoundClient {
    let config = Config::load("~/mpk/mpk.toml")
      .unwrap()
      .net
      .freesound
      .unwrap();
    freesound::FreeSoundClient::new_with_config(&config)
  }
  fn mb_client() -> musicbrainz::MusicBrainzClient {
    musicbrainz::MusicBrainzClient::new()
  }

  #[tokio::test]
  async fn async_simple_get() {
    Client::new()
      .get("https://example.com")
      .send()
      .await
      .unwrap();
  }

  // #[tokio::test]
  // async fn fs_auth() {
  //   let mut client = fs_client();
  //   client
  //     .auth(true)
  //     .await
  //     .unwrap();

  //   let mut cfg = Config::load("~/mpk/mpk.toml").unwrap();
  //   client.save_to_config(&mut cfg);
  //   cfg.write("~/mpk/mpk.toml").unwrap();
  // }

  #[tokio::test]
  async fn fs_sound_id() {
    let req = freesound::FreeSoundRequest::Sound { id: 1234 };
    let res = fs_client().request(req).await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
  }

  #[tokio::test]
  async fn mb_search_raw() {
    let res = mb_client()
      .get_raw("https://musicbrainz.org/ws/2/artist?query=death+grips")
      .await
      .unwrap();
    assert_eq!(res.status().as_u16(), 200);
  }

  #[tokio::test]
  async fn mb_search() {
    let req = musicbrainz::MusicBrainzRequest::Search {
      resource: musicbrainz::MusicBrainzResource::from_str("artist").unwrap(),
      query: "death grips",
      limit: 200,
      offset: 0,
    };
    let res = mb_client().request(req).await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
  }
}
