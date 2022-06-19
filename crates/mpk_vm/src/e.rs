//! MPK_VM -- e
//! error flavors
pub use mpk_parser::EvalError;
pub type Result<T> = std::result::Result<T, VmError>;

#[derive(Debug)]
pub enum VmError {
  Eval(EvalError),
  Generic(String),
  Conversion(String),
  UnexpectedToken(String),
}

impl std::error::Error for VmError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      VmError::Eval(ref e) => Some(e),
      VmError::Generic(_) => None,
      VmError::Conversion(_) => None,
      VmError::UnexpectedToken(_) => None,
    }
  }
}

impl std::fmt::Display for VmError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      VmError::Eval(err) => err.fmt(f),
      VmError::Generic(m) => m.fmt(f),
      VmError::Conversion(m) => m.fmt(f),
      VmError::UnexpectedToken(m) => m.fmt(f),
    }
  }
}

impl From<EvalError> for VmError {
  fn from(e: EvalError) -> Self {
    VmError::Eval(e)
  }
}

#[macro_export]
macro_rules! stop {
    ($type:ident => $fmt:expr, $($arg:tt)+) => {
      return Err(VmError::$type(format!($fmt, $($arg)+)))
    };
    ($type:ident => $thing:expr) => {
        return Err(VmError::$type(($thing).to_string()))
    };
}

#[macro_export]
macro_rules! throw {
    ($type:ident => $fmt:expr, $($arg:tt)+) => {
        || VmError::$type(format!($fmt, $($arg)+))
    };
    ($type:ident => $thing:expr) => {
        || VmError::$type(($thing).to_string())
    };
}
