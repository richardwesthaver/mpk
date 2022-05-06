//! MPK CONFIG
//!
//! Configuration types
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::str::FromStr;
use std::time::{Duration, SystemTime};

use mpk_util::expand_tilde;

mod err;
pub use err::{Error, Result};

pub const DEFAULT_PATH: &str = "~/mpk";
pub const CONFIG_FILE: &str = "mpk.toml";
pub const DB_FILE: &str = "mpk.db";

/// MPK Configuration
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Config {
  pub fs: FsConfig,
  pub db: DbConfig,
  pub jack: JackConfig,
  pub metro: MetroConfig,
  pub sesh: SeshConfig,
  pub net: NetworkConfig,
  pub engine: EngineConfig,
}

impl Config {
  pub fn new(fs: FsConfig, db: DbConfig, jack: JackConfig) -> Result<Config> {
    Ok(Config {
      fs,
      db,
      engine: EngineConfig::default(),
      sesh: SeshConfig::default(),
      jack,
      metro: MetroConfig::default(),
      net: NetworkConfig::default(),
    })
  }

  pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
    let toml_string = toml::to_string_pretty(self).expect("TOML serialization failed");
    let path = expand_tilde(path.as_ref()).unwrap();

    if path.is_dir() {
      let path = &path.join(CONFIG_FILE);
      fs::write(path, toml_string)?;
    } else if path.is_file() {
      fs::write(path, toml_string)?;
    } else if !path.exists() {
      let prefix = path.parent().unwrap();
      if !prefix.exists() {
        fs::create_dir_all(prefix)?;
      }
      fs::write(path, toml_string)?;
    }
    Ok(())
  }

  pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
    let content = fs::read(expand_tilde(path).unwrap()).unwrap();
    let config: Config = toml::from_slice(&content)?;
    Ok(config)
  }

  pub fn build(&self) -> Result<()> {
    let root = expand_tilde(&self.fs.root()).unwrap();
    if !root.exists() {
      fs::create_dir(&root)?;
    }
    for i in
      ["analysis", "samples", "sesh", "plugins", "patches", "tracks"].map(|x| root.join(x))
    {
      if !i.exists() {
        fs::create_dir(root.join(i))?;
      }
    }
    Ok(())
  }
}

/// Files/Folders Config. Internal directories are contained in
/// ROOT. External directories are optional and user-defined.
#[derive(Serialize, Deserialize, Clone)]
pub struct FsConfig {
  pub root: String,
  pub ext_samples: Option<Vec<String>>,
  pub ext_tracks: Option<Vec<String>>,
  pub ext_projects: Option<Vec<String>>,
  pub ext_plugins: Option<Vec<String>>,
  pub ext_patches: Option<Vec<String>>,
}

impl Default for FsConfig {
  fn default() -> Self {
    FsConfig {
      root: DEFAULT_PATH.into(),
      ext_samples: None,
      ext_tracks: None,
      ext_projects: None,
      ext_plugins: None,
      ext_patches: None,
    }
  }
}

impl FsConfig {
  pub fn new<P: AsRef<Path>>(root: P) -> Result<Self> {
    let root = root.as_ref().to_str().unwrap().to_string();
    Ok(FsConfig {
      root,
      ..Default::default()
    })
  }

  pub fn root(&self) -> PathBuf {
    PathBuf::from(&self.root)
  }

  pub fn get_path(&self, path: &str) -> Result<PathBuf> {
    match path {
      "root" => Ok(expand_tilde(PathBuf::from(&self.root)).unwrap()),
      "analysis" => {
        Ok(expand_tilde([&self.root, "analysis"].iter().collect::<PathBuf>()).unwrap())
      }
      "samples" => {
        Ok(expand_tilde([&self.root, "samples"].iter().collect::<PathBuf>()).unwrap())
      }
      "tracks" => {
        Ok(expand_tilde([&self.root, "tracks"].iter().collect::<PathBuf>()).unwrap())
      }
      "sesh" => {
        Ok(expand_tilde([&self.root, "sesh"].iter().collect::<PathBuf>()).unwrap())
      }
      "plugins" => {
        Ok(expand_tilde([&self.root, "plugins"].iter().collect::<PathBuf>()).unwrap())
      }
      "patches" => {
        Ok(expand_tilde([&self.root, "patches"].iter().collect::<PathBuf>()).unwrap())
      }
      e => Err(Error::NotFound(e.to_string())),
    }
  }

  pub fn get_ext_paths(&self, path: &str) -> Option<Vec<PathBuf>> {
    match path {
      "samples" => {
        if let Some(ps) = &self.ext_samples {
          Some(ps.iter().map(|p| PathBuf::from(p)).collect::<Vec<_>>())
        } else {
          None
        }
      }
      "tracks" => {
        if let Some(ps) = &self.ext_tracks {
          Some(ps.iter().map(|p| PathBuf::from(p)).collect::<Vec<_>>())
        } else {
          None
        }
      }
      "projects" => {
        if let Some(ps) = &self.ext_projects {
          Some(ps.iter().map(|p| PathBuf::from(p)).collect::<Vec<_>>())
        } else {
          None
        }
      }
      "plugins" => {
        if let Some(ps) = &self.ext_plugins {
          Some(ps.iter().map(|p| PathBuf::from(p)).collect::<Vec<_>>())
        } else {
          None
        }
      }
      "patches" => {
        if let Some(ps) = &self.ext_patches {
          Some(ps.iter().map(|p| PathBuf::from(p)).collect::<Vec<_>>())
        } else {
          None
        }
      }
      _ => None,
    }
  }
}

impl From<Config> for FsConfig {
  fn from(cfg: Config) -> FsConfig {
    cfg.fs
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DbMode {
  Small,
  Fast,
}

impl FromStr for DbMode {
  type Err = Error;
  fn from_str(mode: &str) -> Result<DbMode> {
    match mode {
      "small" => Ok(DbMode::Small),
      "fast" => Ok(DbMode::Fast),
      e => Err(Error::BadDbMode(e.to_string())),
    }
  }
}

/// Database Configuration.
/// Allow configuration of the MPK DB.
#[derive(Serialize, Deserialize, Clone)]
pub struct DbConfig {
  pub path: PathBuf,
  pub mode: DbMode,
  pub cache_capacity: u64,
  pub print_on_drop: bool,
  pub use_compression: bool,
  pub compression_factor: i32,
}

impl Default for DbConfig {
  fn default() -> Self {
    DbConfig {
      path: PathBuf::from(
        [DEFAULT_PATH, &MAIN_SEPARATOR.to_string(), DB_FILE].concat(),
      ),
      mode: DbMode::Fast,
      cache_capacity: 1024 * 1024 * 1024, // 1gb
      print_on_drop: false,
      use_compression: false,
      compression_factor: 5,
    }
  }
}

impl From<Config> for DbConfig {
  fn from(cfg: Config) -> DbConfig {
    cfg.db
  }
}

/// Engine Configuration.
#[derive(Serialize, Deserialize, Clone)]
pub struct EngineConfig {
  pub socket: String,
}

impl Default for EngineConfig {
  fn default() -> Self {
    EngineConfig {
      socket: "127.0.0.1:0".to_string(),
    }
  }
}

/// JACK Configuration.
#[derive(Serialize, Deserialize, Clone)]
pub struct JackConfig {
  name: String,
  audio: String,
  midi: String,
  device: String,
  realtime: bool,
  auto: char,
  temp: bool,
  rate: u32,
  period: u16,
  n_periods: u8,
  port_max: Option<u16>,
  internal_session: Option<String>,
}

impl Default for JackConfig {
  fn default() -> Self {
    JackConfig {
      name: "mpk".into(),
      audio: "alsa".into(),
      midi: "seq".into(),
      device: "default".into(),
      realtime: true,
      auto: ' ',
      temp: false,
      rate: 44100,
      period: 1024,
      n_periods: 2,
      port_max: None,
      internal_session: None,
    }
  }
}

impl JackConfig {
  pub fn new() -> Result<Self> {
    Ok(JackConfig {
      ..Default::default()
    })
  }
}

impl From<Config> for JackConfig {
  fn from(cfg: Config) -> JackConfig {
    cfg.jack
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MetroConfig {
  pub bpm: u16,
  pub time_sig: (u8, u8),
  pub tic: Option<PathBuf>,
  pub toc: Option<PathBuf>,
}

impl Default for MetroConfig {
  fn default() -> Self {
    MetroConfig {
      bpm: 120,
      time_sig: (4, 4),
      tic: if let Ok(t) = std::env::var("MPK_METRO_TIC") {
        Some(t.into())
      } else {
        None
      },
      toc: if let Ok(t) = std::env::var("MPK_METRO_TOC") {
        Some(t.into())
      } else {
        None
      },
    }
  }
}

impl From<Config> for MetroConfig {
  fn from(cfg: Config) -> MetroConfig {
    cfg.metro
  }
}

/// Configuration for Sessions.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SeshConfig {
  client_addr: String,
  nsm_url: Option<String>,
}

impl Default for SeshConfig {
  fn default() -> Self {
    SeshConfig {
      client_addr: "127.0.0.1:0".to_string(),
      nsm_url: None,
    }
  }
}

impl From<Config> for SeshConfig {
  fn from(cfg: Config) -> SeshConfig {
    cfg.sesh
  }
}

/// Network configuration for HTTP API clients.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkConfig {
  pub socket: String,
  pub freesound: Option<ClientConfig>,
  pub musicbrainz: Option<ClientConfig>,
  pub youtube: Option<ClientConfig>,
  pub spotify: Option<ClientConfig>,
}

impl Default for NetworkConfig {
  fn default() -> Self {
    NetworkConfig {
      socket: "127.0.0.1:0".to_string(),
      freesound: None,
      musicbrainz: None,
      youtube: None,
      spotify: None,
    }
  }
}

impl From<Config> for NetworkConfig {
  fn from(cfg: Config) -> NetworkConfig {
    cfg.net
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClientConfig {
  pub client_id: String,
  pub client_secret: String,
  pub redirect_url: String,
  pub access_token: Option<String>,
  pub refresh_token: Option<String>,
  pub scopes: Option<Vec<String>>,
  pub expires: Option<u64>,
}

impl ClientConfig {
  pub fn update(
    &mut self,
    access_token: &str,
    refresh_token: &str,
    expires_in: Duration,
    scopes: &[String],
  ) {
    self.access_token = Some(access_token.to_string());
    self.refresh_token = Some(refresh_token.to_string());
    self.scopes = Some(scopes.to_vec());
    let expires = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .expect("SystemTime is before UNIX_EPOCH!?")
      + expires_in;
    self.expires = Some(expires.as_secs());
  }
}
