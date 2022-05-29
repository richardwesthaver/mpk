//! MPK -- ERR
#[derive(Debug)]
pub enum Error {
  #[cfg(feature = "db")]
  Db(mpk_db::Error),
  #[cfg(feature = "config")]
  Cfg(mpk_config::Error),
  #[cfg(feature = "midi")]
  Midi(mpk_midi::Error),
  #[cfg(feature = "audio")]
  Audio(mpk_audio::Error),
  #[cfg(feature = "gear")]
  Gear(mpk_gear::Error),
  #[cfg(feature = "repl")]
  Repl(mpk_repl::Error),
  Io(std::io::Error),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      #[cfg(feature = "db")]
      Error::Db(ref err) => Some(err),
      #[cfg(feature = "config")]
      Error::Cfg(ref err) => Some(err),
      #[cfg(feature = "midi")]
      Error::Midi(ref err) => Some(err),
      #[cfg(feature = "audio")]
      Error::Audio(ref err) => Some(err),
      #[cfg(feature = "gear")]
      Error::Gear(ref err) => Some(err),
      #[cfg(feature = "repl")]
      Error::Repl(ref err) => Some(err),
      Error::Io(ref err) => Some(err),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      #[cfg(feature = "db")]
      Error::Db(ref err) => err.fmt(f),
      #[cfg(feature = "config")]
      Error::Cfg(ref err) => err.fmt(f),
      #[cfg(feature = "midi")]
      Error::Midi(ref err) => err.fmt(f),
      #[cfg(feature = "audio")]
      Error::Audio(ref err) => err.fmt(f),
      #[cfg(feature = "gear")]
      Error::Gear(ref err) => err.fmt(f),
      #[cfg(feature = "repl")]
      Error::Repl(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    Error::Io(err)
  }
}

#[cfg(feature = "db")]
impl From<mpk_db::Error> for Error {
  fn from(err: mpk_db::Error) -> Error {
    Error::Db(err)
  }
}

#[cfg(feature = "config")]
impl From<mpk_config::Error> for Error {
  fn from(err: mpk_config::Error) -> Error {
    Error::Cfg(err)
  }
}

#[cfg(feature = "midi")]
impl From<mpk_midi::Error> for Error {
  fn from(err: mpk_midi::Error) -> Error {
    Error::Midi(err)
  }
}

#[cfg(feature = "audio")]
impl From<mpk_audio::Error> for Error {
  fn from(err: mpk_audio::Error) -> Error {
    Error::Audio(err)
  }
}

#[cfg(feature = "gear")]
impl From<mpk_gear::Error> for Error {
  fn from(err: mpk_gear::Error) -> Error {
    Error::Gear(err)
  }
}

#[cfg(feature = "repl")]
impl From<mpk_repl::Error> for Error {
  fn from(err: mpk_repl::Error) -> Error {
    Error::Repl(err)
  }
}
