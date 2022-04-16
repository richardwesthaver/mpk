pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Readline(rustyline::error::ReadlineError),
  Io(std::io::Error),
  Parser(ParserError),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Readline(ref err) => Some(err),
      Error::Io(ref err) => Some(err),
      Error::Parser(ref err) => Some(err),
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Readline(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
      Error::Parser(ref err) => err.fmt(f),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Error::Io(err)
  }
}

impl From<rustyline::error::ReadlineError> for Error {
  fn from(err: rustyline::error::ReadlineError) -> Self {
    Error::Readline(err)
  }
}

impl From<ParserError> for Error {
  fn from(err: ParserError) -> Self {
    Error::Parser(err)
  }
}

#[derive(Debug)]
pub enum ParserError {
  Unexpected {
    unexpected: String,
    expected: Option<String>,
  },
  EndOfStream {
    expected: Option<String>,
  },
  RecursionLimit,
  UnexpectedChar(u8),
  BadNumber,
}

impl std::fmt::Display for ParserError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let write_expected = |f: &mut std::fmt::Formatter, expected: &Option<String>| {
      match expected {
        Some(expected) => {
          write!(f, ", expected {}", expected)?;
        }
        None => {}
      }
      Ok(())
    };

    match self {
      ParserError::Unexpected {
        unexpected,
        expected,
      } => {
        write!(f, "found {}", unexpected)?;
        write_expected(f, expected)
      }
      ParserError::EndOfStream { expected } => {
        write!(f, "unexpected end of token stream")?;
        write_expected(f, expected)
      }
      ParserError::RecursionLimit => write!(f, "recursion limit reached"),
      ParserError::UnexpectedChar(c) => write!(
        f,
        "unexpected character: '{}'",
        std::char::from_u32(*c as u32).unwrap_or(std::char::REPLACEMENT_CHARACTER)
      ),
      ParserError::BadNumber => write!(f, "malformed number"),
    }
  }
}

impl std::error::Error for ParserError {}
