pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Toml(toml::de::Error),
  Io(std::io::Error),
  BadFlag(String),
  NotFound(String),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Toml(ref err) => Some(err),
      Error::Io(ref err) => Some(err),
      Error::BadFlag(_) => None,
      Error::NotFound(_) => None,
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Toml(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
      Error::BadFlag(ref s) => f.write_str(&format!("Invalid Flag: {}", s)),
      Error::NotFound(ref s) => f.write_str(&format!("Path not found: {}", s)),
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
