pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Osc(rosc::OscError),
  Io(std::io::Error),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Osc(ref err) => Some(err),
      Error::Io(ref err) => Some(err),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Osc(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
    }
  }
}

impl From<rosc::OscError> for Error {
  fn from(err: rosc::OscError) -> Error {
    Error::Osc(err)
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    Error::Io(err)
  }
}