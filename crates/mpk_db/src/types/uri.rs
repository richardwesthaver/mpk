//! MPK_DB/TYPES -- URI
use super::Key;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

pub enum UriError {
  BadScheme(String),
  BadPath(String),
}

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
  type Err = UriError;
  fn from_str(str: &str) -> Result<UriScheme, UriError> {
    match str {
      "file" => Ok(UriScheme::File),
      "http" => Ok(UriScheme::Http),
      "https" => Ok(UriScheme::Https),
      "magnet" => Ok(UriScheme::Magnet),
      "yt" => Ok(UriScheme::Yt),
      "sp" => Ok(UriScheme::Sp),
      "fs" => Ok(UriScheme::Fs),
      e => Err(UriError::BadScheme(e.to_string())),
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
  pub fn new(uri: &str) -> Result<Uri, UriError> {
    Uri::from_str(uri)
  }
}

impl FromStr for Uri {
  type Err = UriError;
  fn from_str(str: &str) -> Result<Uri, UriError> {
    let mut split = str.splitn(2, ':');
    let scheme = UriScheme::from_str(split.next().unwrap())?;
    let path = split.next().unwrap().to_string();
    Ok(Uri { scheme, path })
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
