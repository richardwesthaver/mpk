use std::path::{PathBuf, Path, MAIN_SEPARATOR};
use std::fs;
use std::io;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
pub const DEFAULT_PATH: &str = "~/mpk";
pub const CONFIG_FILE: &str = "mpk.toml";
pub const DB_FILE: &str = "mpk.db";

fn expand_tilde<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
  let p = path.as_ref();
  if !p.starts_with("~") {
    return Some(p.to_path_buf());
  }
  if p == Path::new("~") {
    return dirs::home_dir();
  }
  dirs::home_dir().map(|mut h| {
    if h == Path::new("/") {
      p.strip_prefix("~").unwrap().to_path_buf()
    } else {
      h.push(p.strip_prefix("~/").unwrap());
      h
    }
  })
}

/// MPK Configuration
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
  pub fs: FsConfig,
  pub db: DbConfig,
  pub jack: JackConfig,
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

  pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, toml::de::Error> {
    let content = fs::read(expand_tilde(path).unwrap()).unwrap();
    let config: Config = toml::from_slice(&content)?;
    Ok(config)
  }

  pub fn build(&self) -> Result<(), io::Error> {
    let root = expand_tilde(&self.fs.root()).unwrap();
    if !root.exists() {
      fs::create_dir(&root)?;
    }
    for i in ["samples", "projects", "plugins", "patches", "tracks"].map(|x| root.join(x)) {
      if !i.exists() {
	fs::create_dir(root.join(i))?;
      }
    }
    Ok(())
  }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FsConfig {
  pub root: String,
}

impl Default for FsConfig {
  fn default() -> Self {
    FsConfig {
      root: DEFAULT_PATH.into(),
    }
  }
}

impl FsConfig {
  pub fn new<P: AsRef<Path>>(root: P) -> Result<Self, io::Error> {
    let root = root.as_ref().to_str().unwrap().to_string();
    Ok(
      FsConfig {
	root
      }
    )
  }

  pub fn root(&self) -> PathBuf {
    PathBuf::from(&self.root)
  }

  pub fn get_path(&self, path: &str) -> Result<PathBuf, io::Error> {
    match path {
      "root" => Ok(expand_tilde(PathBuf::from(&self.root)).unwrap()),
      "samples" => Ok(expand_tilde([&self.root, "samples"].iter().collect::<PathBuf>()).unwrap()),
      "projects" => Ok(expand_tilde([&self.root, "projects"].iter().collect::<PathBuf>()).unwrap()),
      "plugins" => Ok(expand_tilde([&self.root, "plugins"].iter().collect::<PathBuf>()).unwrap()),
      "patches" => Ok(expand_tilde([&self.root, "patches"].iter().collect::<PathBuf>()).unwrap()),
      "tracks" => Ok(expand_tilde([&self.root, "tracks"].iter().collect::<PathBuf>()).unwrap()),
      e => Err(io::Error::new(io::ErrorKind::NotFound, e)),
    }
  }
}

#[allow(non_snake_case, non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[repr(C)]
pub enum Flags {
  READ_ONLY = 0x00000001,
  READ_WRITE = 0x00000002,
  CREATE = 0x00000004,
  DELETE_ON_CLOSE = 0x00000008,
  EXCLUSIVE = 0x00000010,
  AUTOPROXY = 0x00000020,
  URI = 0x00000040,
  MEMORY = 0x00000080,
  MAIN_DB = 0x00000100,
  TEMP_DB = 0x00000200,
  TRANSIENT_DB = 0x00000400,
  MAIN_JOURNAL = 0x00000800,
  TEMP_JOURNAL = 0x00001000,
  SUBJOURNAL = 0x00002000,
  SUPER_JOURNAL = 0x00004000,
  NOMUTEX = 0x00008000,
  FULLMUTEX = 0x00010000,
  SHAREDCACHE = 0x00020000,
  PRIVATECACHE = 0x00040000,
  WAL = 0x00080000,
  NOFOLLOW = 0x01000000,
  EXRESCODE = 0x02000000,
}

impl Flags {
  pub fn to_int(&self) -> std::os::raw::c_int {
    match &self {
      Flags::READ_ONLY => 0x00000001,
      Flags::READ_WRITE => 0x00000002,
      Flags::CREATE => 0x00000004,
      Flags::DELETE_ON_CLOSE => 0x00000008,
      Flags::EXCLUSIVE => 0x00000010,
      Flags::AUTOPROXY => 0x00000020,
      Flags::URI => 0x00000040,
      Flags::MEMORY => 0x00000080,
      Flags::MAIN_DB => 0x00000100,
      Flags::TEMP_DB => 0x00000200,
      Flags::TRANSIENT_DB => 0x00000400,
      Flags::MAIN_JOURNAL => 0x00000800,
      Flags::TEMP_JOURNAL => 0x00001000,
      Flags::SUBJOURNAL => 0x00002000,
      Flags::SUPER_JOURNAL => 0x00004000,
      Flags::NOMUTEX => 0x00008000,
      Flags::FULLMUTEX => 0x00010000,
      Flags::SHAREDCACHE => 0x00020000,
      Flags::PRIVATECACHE => 0x00040000,
      Flags::WAL => 0x00080000,
      Flags::NOFOLLOW => 0x01000000,
      Flags::EXRESCODE => 0x02000000,
    }
  }
}
impl FromStr for Flags {
  type Err = ();

  fn from_str(input: &str) -> Result<Flags, Self::Err> {
    match input {
      "readonly" => Ok(Flags::READ_ONLY),
      "readwrite" => Ok(Flags::READ_WRITE),
      "create" => Ok(Flags::CREATE),
      "deleteonclose" => Ok(Flags::DELETE_ON_CLOSE),
      "exclusive" => Ok(Flags::EXCLUSIVE),
      "autoproxy" => Ok(Flags::AUTOPROXY),
      "uri" => Ok(Flags::URI),
      "memory" => Ok(Flags::MEMORY),
      "maindb" => Ok(Flags::MAIN_DB),
      "tempdb" => Ok(Flags::TEMP_DB),
      "transientdb" => Ok(Flags::TRANSIENT_DB),
      "mainjournal" => Ok(Flags::MAIN_JOURNAL),
      "tempjournal" => Ok(Flags::TEMP_JOURNAL),
      "subjournal" => Ok(Flags::SUBJOURNAL),
      "superjournal" => Ok(Flags::SUPER_JOURNAL),
      "nomutex" => Ok(Flags::NOMUTEX),
      "fullmutex" => Ok(Flags::FULLMUTEX),
      "sharedcache" => Ok(Flags::SHAREDCACHE),
      "privatecache" => Ok(Flags::PRIVATECACHE),
      "wal" => Ok(Flags::WAL),
      "nofollow" => Ok(Flags::NOFOLLOW),
      "exrescode" => Ok(Flags::EXRESCODE),
      _ => Err(()),
    }
  }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct DbConfig {
  pub path: Option<String>,
  pub log_file: Option<String>,
  pub flags: Option<Vec<String>>,
  pub limits: Option<HashMap<String, usize>>,
}

impl DbConfig {
  pub fn flags(&self) -> Option<std::os::raw::c_int> {
    match &self.flags {
      Some(fs) => {
	Some(
	  fs.into_iter()
	    .map(|f| Flags::from_str(&f).expect("invalid flag").to_int()).sum()
	)
      },
      None => None,
    }
  }

  pub fn path(&self) -> Option<PathBuf> {
    match &self.path {
      Some(p) => expand_tilde(p),
      None => None,
    }
  }
}

impl Default for DbConfig {
  fn default() -> Self {
    DbConfig {
      path: Some([DEFAULT_PATH, &MAIN_SEPARATOR.to_string(), DB_FILE].concat()),
      log_file: None,
      flags: Some(vec!["readwrite", "create", "nomutex", "uri"].iter().map(|x| x.to_string()).collect()),
      limits: None,
    }
  }
}

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
  pub fn new() -> Result<Self, io::Error> {
    Ok(
      JackConfig {
	..Default::default()
      }
    )
  }
}

#[derive(Serialize, Deserialize)]
pub struct SetConfig {}

#[derive(Serialize, Deserialize)]
pub struct ProjectConfig {}

#[derive(Serialize, Deserialize)]
pub struct PatchConfig {}
