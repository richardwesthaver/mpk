//! MPK_HTTP
pub use reqwest::Client;

mod err;
pub use err::{Error, Result};

pub mod coverartarchive;
pub mod freesound;
pub mod musicbrainz;

/// OS-specific browser command. supports Win/Mac/Linux
pub fn open_browser(url: &str) {
  if cfg!(target_os = "windows") {
    // https://stackoverflow.com/a/49115945
    std::process::Command::new("rundll32.exe")
      .args(&["url.dll,FileProtocolHandler", url])
      .status()
      .expect("failed to open file");
  } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
    // https://dwheeler.com/essays/open-files-urls.html
    #[cfg(target_os = "macos")]
    let cmd = "open";
    #[cfg(target_os = "linux")]
    let cmd = "xdg-open";

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      std::process::Command::new(cmd)
        .arg(url)
        .status()
        .expect("failed to open file");
    }
  } else {
    unimplemented!() //Ignore others
  }
}

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
