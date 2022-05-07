pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Osc(rosc::OscError),
  Io(std::io::Error),
  BadType(String),
  BadMessage(String),
  BadPacket(rosc::OscPacket),
  BadArg(&'static str),
  BadReplyAddr(String),
  BadAddr(String),
  BadCode(i32),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Osc(ref err) => Some(err),
      Error::Io(ref err) => Some(err),
      _ => None,
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match &self {
      Error::Osc(ref err) => write!(f, "OSC error: {}", err.to_string()),
      Error::Io(ref err) => write!(f, "IO error: {}", err.to_string()),
      Error::BadType(s) => write!(f, "bad type: {}", s),
      Error::BadMessage(s) => write!(f, "bad OSC message: {}", s),
      Error::BadPacket(s) => write!(f, "bad OSC packet: {:?}", s),
      Error::BadArg(s) => write!(f, "bad OSC arg: {}", s),
      Error::BadReplyAddr(s) => write!(f, "bad OSC reply address: {}", s),
      Error::BadAddr(s) => write!(f, "bad OSC address: {}", s),
      Error::BadCode(n) => write!(f, "bad error code: {}", n),
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
