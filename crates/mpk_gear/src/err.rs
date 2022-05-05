pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Audio(mpk_audio::Error),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Audio(ref err) => Some(err),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Audio(ref err) => err.fmt(f),
    }
  }
}

impl From<mpk_audio::Error> for Error {
  fn from(err: mpk_audio::Error) -> Error {
    Error::Audio(err)
  }
}
