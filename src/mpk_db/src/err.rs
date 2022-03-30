pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Sql(rusqlite::Error),
  Io(std::io::Error),
  BadType(String),
  BadNote(String),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Sql(ref err) => Some(err),
      Error::Io(ref err) => Some(err),
      _ => None,
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Sql(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
      Error::BadType(ref s) => f.write_str(&format!("Invalid Type: {}", s)),
      Error::BadNote(ref s) => f.write_str(&format!("Invalid Note: {}", s)),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    Error::Io(err)
  }
}

impl From<rusqlite::Error> for Error {
  fn from(err: rusqlite::Error) -> Error {
    Error::Sql(err)
  }
}
