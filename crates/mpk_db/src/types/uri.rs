//! MPK_DB/TYPES -- URI
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::Key;
use crate::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum UriScheme {
  File,
  Http,
  Https,
  Magnet,
  Yt,
  Sp,
  Fs,
}

impl FromStr for UriScheme {
  type Err = Error;
  fn from_str(str: &str) -> Result<UriScheme, Error> {
    match str {
      "file" => Ok(UriScheme::File),
      "http" => Ok(UriScheme::Http),
      "https" => Ok(UriScheme::Https),
      "magnet" => Ok(UriScheme::Magnet),
      "yt" => Ok(UriScheme::Yt),
      "sp" => Ok(UriScheme::Sp),
      "fs" => Ok(UriScheme::Fs),
      e => Err(Error::BadUriScheme(e.to_string())),
    }
  }
}

impl fmt::Display for UriScheme {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      UriScheme::File => write!(f, "file"),
      UriScheme::Http => write!(f, "http"),
      UriScheme::Https => write!(f, "https"),
      UriScheme::Magnet => write!(f, "magnet"),
      UriScheme::Yt => write!(f, "yt"),
      UriScheme::Sp => write!(f, "sp"),
      UriScheme::Fs => write!(f, "fs"),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Uri {
  scheme: UriScheme,
  path: String,
}

impl Uri {
  pub fn new(uri: &str) -> Result<Uri, Error> {
    Uri::from_str(uri)
  }
}

impl FromStr for Uri {
  type Err = Error;
  fn from_str(str: &str) -> Result<Uri, Error> {
    let mut split = str.splitn(2, ':');
    let scheme = UriScheme::from_str(split.next().unwrap())?;
    let path = split.next().unwrap().to_string();
    Ok(Uri { scheme, path })
  }
}

impl From<PathBuf> for Uri {
  fn from(path: PathBuf) -> Uri {
    let scheme = UriScheme::File;
    let path = path.to_string_lossy().to_string();
    Uri { scheme, path }
  }
}

impl Key for Uri {
  type Key = Uri;
  fn key(&self) -> &Uri {
    self
  }
}

impl fmt::Display for Uri {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", format!("{}:{}", self.scheme, self.path))
  }
}

pub trait UriExt {}
