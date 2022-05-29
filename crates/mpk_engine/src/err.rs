//! MPK_ENGINE ERR
pub use mpk_parser::EvalError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Osc(mpk_osc::Error),
  Parser(mpk_parser::Error),
  Vm(VmError),
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

impl From<VmError> for Error {
  fn from(e: VmError) -> Self {
    Error::Vm(e)
  }
}

#[derive(Debug)]
pub enum VmError {
  Eval(EvalError),
}

impl std::error::Error for VmError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      VmError::Eval(ref e) => Some(e),
    }
  }
}

impl std::fmt::Display for VmError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      VmError::Eval(err) => err.fmt(f),
    }
  }
}

impl From<EvalError> for VmError {
  fn from(e: EvalError) -> Self {
    VmError::Eval(e)
  }
}
