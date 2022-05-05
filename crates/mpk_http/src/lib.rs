//! MPK_HTTP
pub use reqwest::Client;

mod err;
pub use err::{Error, Result};

pub mod coverartarchive;
pub mod freesound;
pub mod musicbrainz;

#[cfg(test)]
mod tests {
  use super::*;
  use mpk_config::{Config, NetworkConfig};
  #[tokio::test]
  async fn async_simple_get() {
    Client::new()
      .get("https://example.com")
      .send()
      .await
      .unwrap();
  }

  #[tokio::test]
  async fn freesound_auth() {
    let config = NetworkConfig::from(Config::load("~/mpk/mpk.toml").unwrap())
      .freesound
      .unwrap();
    freesound::FreeSoundClient::new_with_config(&config)
      .auth(true)
      .await
      .unwrap();
  }
}
