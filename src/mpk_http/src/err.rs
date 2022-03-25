pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Http(reqwest::Error),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Http(ref err) => Some(err),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Http(ref err) => err.fmt(f),
    }
  }
}

impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Error {
    Error::Http(err)
  }
}
