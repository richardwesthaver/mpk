pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Id3(id3::Error),
  Sql(rusqlite::Error),
  Json(serde_json::Error),
  Io(std::io::Error),
  BadQType(String),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Id3(ref err) => Some(err),
      Error::Sql(ref err) => Some(err),
      Error::Json(ref err) => Some(err),
      Error::Io(ref err) => Some(err),
      Error::BadQType(_) => None,
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Id3(ref err) => err.fmt(f),
      Error::Sql(ref err) => err.fmt(f),
      Error::Json(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
      Error::BadQType(ref s) => f.write_str(&format!("Invalid Query Type: {}", s)),
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

impl From<id3::Error> for Error {
  fn from(err: id3::Error) -> Error {
    Error::Id3(err)
  }
}

impl From<serde_json::Error> for Error {
  fn from(err: serde_json::Error) -> Error {
    Error::Json(err)
  }
}
