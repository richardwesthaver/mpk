pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Toml(toml::de::Error),
  Io(std::io::Error),
  BadDbMode(String),
  NotFound(String),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Toml(ref err) => Some(err),
      Error::Io(ref err) => Some(err),
      Error::BadDbMode(_) => None,
      Error::NotFound(_) => None,
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Toml(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
      Error::BadDbMode(ref s) => f.write_str(&format!("invalid DB mode: {}", s)),
      Error::NotFound(ref s) => f.write_str(&format!("path not found: {}", s)),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    Error::Io(err)
  }
}

impl From<toml::de::Error> for Error {
  fn from(err: toml::de::Error) -> Error {
    Error::Toml(err)
  }
}
