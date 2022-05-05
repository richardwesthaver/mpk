//! MPK_DB -- ERR
use std::fmt;

#[derive(Debug)]
pub enum ValidationError {
  BadValue(String),
}

impl fmt::Display for ValidationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ValidationError::BadValue(s) => write!(f, "bad value: {}", s),
    }
  }
}

impl std::error::Error for ValidationError {}
