pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  BadChainExt(String),
  Device(rodio::DevicesError),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::BadChainExt(_) => None,
      Error::Device(ref e) => Some(e),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::BadChainExt(ref s) => {
        f.write_str(&format!("Invalid Chain Extension: {}", s))
      }
      Error::Device(ref e) => e.fmt(f),
    }
  }
}

impl From<rodio::DevicesError> for Error {
  fn from(e: rodio::DevicesError) -> Error {
    Error::Device(e)
  }
}
