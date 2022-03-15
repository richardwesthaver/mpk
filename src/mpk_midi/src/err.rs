pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  MidiInit(midir::InitError),
  MidiConnect(midir::ConnectError<midir::ConnectErrorKind>),
  MidiPortInfo(midir::PortInfoError),
  MidiSend(midir::SendError),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::MidiInit(ref err) => Some(err),
      Error::MidiConnect(ref err) => Some(err),
      Error::MidiPortInfo(ref err) => Some(err),
      Error::MidiSend(ref err) => Some(err),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::MidiInit(ref err) => err.fmt(f),
      Error::MidiConnect(ref err) => err.fmt(f),
      Error::MidiPortInfo(ref err) => err.fmt(f),
      Error::MidiSend(ref err) => err.fmt(f),
    }
  }
}

impl From<midir::InitError> for Error {
  fn from(e: midir::InitError) -> Error {
    Error::MidiInit(e)
  }
}

impl From<midir::ConnectError<midir::ConnectErrorKind>> for Error {
  fn from(e: midir::ConnectError<midir::ConnectErrorKind>) -> Error {
    Error::MidiConnect(e)
  }
}

impl From<midir::PortInfoError> for Error {
  fn from(e: midir::PortInfoError) -> Error {
    Error::MidiPortInfo(e)
  }
}

impl From<midir::SendError> for Error {
  fn from(e: midir::SendError) -> Error {
    Error::MidiSend(e)
  }
}
