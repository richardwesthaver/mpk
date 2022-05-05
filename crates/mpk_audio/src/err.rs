pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  BadChainExt(String),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::BadChainExt(_) => None,
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::BadChainExt(ref s) => {
        f.write_str(&format!("Invalid Chain Extension: {}", s))
      }
    }
  }
}
