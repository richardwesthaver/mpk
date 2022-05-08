//! MPK_DB -- ERR
use std::fmt;

#[derive(Debug)]
pub enum Error {
  BadValue(String),
  Db(sled::Error),
  BadUriScheme(String),
  BadUriPath(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::BadValue(s) => write!(f, "bad value: {}", s),
      Error::Db(ref e) => e.fmt(f),
      Error::BadUriScheme(s) => write!(f, "bad URI scheme: {}", s),
      Error::BadUriPath(s) => write!(f, "bad URI path: {}", s),
    }
  }
}

impl std::error::Error for Error {}

impl From<sled::Error> for Error {
  fn from(e: sled::Error) -> Error {
    Error::Db(e)
  }
}
