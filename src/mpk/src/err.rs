pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Db(mpk_db::Error),
  Cfg(mpk_config::Error),
  Midi(mpk_midi::Error),
  Io(std::io::Error),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Db(ref err) => Some(err),
      Error::Cfg(ref err) => Some(err),
      Error::Midi(ref err) => Some(err),
      Error::Io(ref err) => Some(err),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Db(ref err) => err.fmt(f),
      Error::Cfg(ref err) => err.fmt(f),
      Error::Midi(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    Error::Io(err)
  }
}

impl From<mpk_db::Error> for Error {
  fn from(err: mpk_db::Error) -> Error {
    Error::Db(err)
  }
}

impl From<mpk_config::Error> for Error {
  fn from(err: mpk_config::Error) -> Error {
    Error::Cfg(err)
  }
}

impl From<mpk_midi::Error> for Error {
  fn from(err: mpk_midi::Error) -> Error {
    Error::Midi(err)
  }
}
