//! MPK_ENGINE ERR

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

#[derive(Debug)]
pub enum EvalError {
  Class,
  Rank,
  Length,
  Type,
  Domain,
  Limit,
  Nyi,
  Parse,
  Value,
}

impl std::error::Error for EvalError {}

impl std::fmt::Display for EvalError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      EvalError::Class => f.write_str(":class"),
      EvalError::Rank => f.write_str(":rank"),
      EvalError::Length => f.write_str(":length"),
      EvalError::Type => f.write_str(":type"),
      EvalError::Domain => f.write_str(":domain"),
      EvalError::Limit => f.write_str(":limit"),
      EvalError::Nyi => f.write_str(":nyi"),
      EvalError::Parse => f.write_str(":parse"),
      EvalError::Value => f.write_str(":value"),
    }
  }
}
