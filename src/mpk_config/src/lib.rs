use std::path::{PathBuf, Path};
use std::fs;
use std::io;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

const DEFAULT_PATH: &str = "~/mpk";
const CONFIG_FILE: &str = "mpk.toml";
const DB_FILE: &str = "mpk.db";

/// MPK Configuration
#[derive(Serialize, Deserialize)]
pub struct Config {
  fs: FsConfig,
  db: DbConfig,
  jack: JackConfig,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      fs: FsConfig::default(),
      db: DbConfig::default(),
      jack: JackConfig::default(),
    }
  }
}
impl Config {
  pub fn new(fs: FsConfig, db: DbConfig, jack: JackConfig) -> Result<Config, toml::de::Error> {
    Ok(
      Config {
	fs,
	db,
	jack,
      }
    )
  }

  pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
    let toml_string = toml::to_string_pretty(self)
      .expect("TOML serialization failed");
    let path = path.as_ref();

    if path.is_dir() {
      let path = &path.join(CONFIG_FILE);
      fs::write(path, toml_string)?;
    } else if path.is_file() {
      fs::write(path, toml_string)?;
    }
    Ok(())
  }

  pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, toml::de::Error> {
    let content = fs::read(path).expect("failed to read config file");
    let config: Config = toml::from_slice(&content)?;
    Ok(config)
  }

  pub fn build(&self) -> Result<(), io::Error> {
    fs::create_dir_all(self.fs.root())?;
    let paths = ["samples", "projects", "plugins", "patches", "tracks"];
    for i in paths.iter() {
      let p = self.fs.get_path(i)?;
      fs::create_dir(p)?;
    }
    Ok(())
  }
}

#[derive(Serialize, Deserialize)]
pub struct FsConfig {
  pub root: String,
  pub samples: String,
  pub projects: String,
  pub plugins: String,
  pub patches: String,
  pub tracks: String,
}

impl Default for FsConfig {
  fn default() -> Self {
    FsConfig {
      root: DEFAULT_PATH.into(),
      samples: "samples".into(),
      projects: "projects".into(),
      plugins: "plugins".into(),
      patches: "patches".into(),
      tracks: "tracks".into(),
    }
  }
}

impl FsConfig {
  pub fn new<P: AsRef<Path>>(root: P) -> Self {
    let root = root.as_ref().to_str().unwrap().to_string();
    FsConfig {
      root,
      ..Default::default()
    }
  }

  pub fn root(&self) -> PathBuf {
    PathBuf::from(&self.root)
  }

  pub fn get_path(&self, path: &str) -> Result<PathBuf, io::Error> {
    match path {
      "root" => Ok(PathBuf::from(&self.root)),
      "samples" => Ok([&self.root, &self.samples].iter().collect()),
      "projects" => Ok([&self.root, &self.projects].iter().collect()),
      "plugins" => Ok([&self.root, &self.plugins].iter().collect()),
      "patches" => Ok([&self.root, &self.patches].iter().collect()),
      "tracks" => Ok([&self.root, &self.patches].iter().collect()),
      e => Err(io::Error::new(io::ErrorKind::NotFound, e)),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct DbConfig {
  path: String,
  log_file: Option<String>,
  flags: Option<Vec<String>>,
  limits: Option<HashMap<String, usize>>,
}

impl Default for DbConfig {
  fn default() -> Self {
    DbConfig {
      path: DB_FILE.into(),
      log_file: None,
      flags: None,
      limits: None,
    }
  }
}

#[derive(Serialize, Deserialize)]
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
  pub fn new() -> Self {
    JackConfig {
      ..Default::default()
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct SetConfig {}

#[derive(Serialize, Deserialize)]
pub struct ProjectConfig {}

#[derive(Serialize, Deserialize)]
pub struct PatchConfig {}
