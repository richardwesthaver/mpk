//! MPK_ENGINE ERR
pub use mpk_parser::EvalError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Osc(mpk_osc::Error),
  Parser(mpk_parser::Error),
  Vm(mpk_vm::VmError),
  //  Arena(mpk_arena::Error),
  Io(std::io::Error),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Osc(ref err) => Some(err),
      Error::Parser(ref err) => Some(err),
      Error::Io(ref err) => Some(err),
      Error::Vm(ref err) => Some(err),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Osc(ref err) => err.fmt(f),
      Error::Parser(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
      Error::Vm(ref err) => err.fmt(f),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Error::Io(err)
  }
}

impl From<mpk_osc::Error> for Error {
  fn from(err: mpk_osc::Error) -> Self {
    Error::Osc(err)
  }
}

impl From<mpk_parser::Error> for Error {
  fn from(err: mpk_parser::Error) -> Self {
    Error::Parser(err)
  }
}

impl From<mpk_vm::VmError> for Error {
  fn from(e: mpk_vm::VmError) -> Self {
    Error::Vm(e)
  }
}
