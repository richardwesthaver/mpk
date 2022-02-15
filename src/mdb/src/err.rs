pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Id3(mpk_id3::Error),
  Sql(rusqlite::Error),
  Io(std::io::Error),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Id3(_) => None,
      Error::Sql(_) => None,
      Error::Io(_) => None,
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Id3(ref err) => err.fmt(f),
      Error::Sql(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    Error::Io(err)
  }
}

impl From<mpk_id3::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    Error::Id3(err)
  }
}

impl From<rusqlite::Error> for Error {
  fn from(err: rusqlite::Error) -> Error {
    Error::Sql(err)
  }
}
