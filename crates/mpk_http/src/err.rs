pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Http(reqwest::Error),
  TokenExpired,
  TokenRefreshFailed,
  Value(String),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Http(ref err) => Some(err),
      Error::TokenExpired => None,
      Error::TokenRefreshFailed => None,
      Error::Value(_) => None,
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Http(ref err) => err.fmt(f),
      Error::TokenExpired => f.write_str("Refresh token has expired"),
      Error::TokenRefreshFailed => {
        f.write_str("Failed to renew auth with refresh token")
      }
      Error::Value(ref s) => write!(f, "Invalid value: {}", s),
    }
  }
}

impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Error {
    Error::Http(err)
  }
}
