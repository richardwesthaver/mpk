//! MPK_PARSER -- ERR
use std::fmt;

use mpk_hash::FxHashMap as HashMap;
pub use pest::error::{Error as PestError, ErrorVariant, InputLocation};

use crate::parser::Rule;

#[derive(Debug)]
pub enum Error {
  PestErr(PestError<Rule>),
  InvalidNoun(String, String),
  Length(usize, usize),
  Num(String),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Error::PestErr(ref e) => Some(e),
      Error::InvalidNoun(..) => None,
      Error::Length(..) => None,
      Error::Num(..) => None,
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::PestErr(ref e) => e.fmt(f),
      Error::InvalidNoun(expected, found) => {
        f.write_fmt(format_args!("expected {}, found {}", expected, found))
      }
      Error::Length(expected, found) => {
        f.write_fmt(format_args!("expected {}, found {}", expected, found))
      }
      Error::Num(i) => f.write_fmt(format_args!("failed to parse number {}", i)),
    }
  }
}

impl From<PestError<Rule>> for Error {
  fn from(e: PestError<Rule>) -> Error {
    Error::PestErr(e)
  }
}

pub fn convert_error(error: PestError<Rule>, grammar: &str) -> HashMap<String, String> {
  let message = match error.variant {
    ErrorVariant::CustomError { message } => message,
    _ => unreachable!(),
  };

  match error.location {
    InputLocation::Pos(pos) => {
      let mut map = HashMap::default();

      map.insert("from".to_owned(), line_col(pos, grammar));
      map.insert("to".to_owned(), line_col(pos, grammar));
      map.insert("message".to_owned(), format!("{}", message));

      map
    }
    InputLocation::Span((start, end)) => {
      let mut map = HashMap::default();

      map.insert("from".to_owned(), line_col(start, grammar));
      map.insert("to".to_owned(), line_col(end, grammar));
      map.insert("message".to_owned(), format!("{}", message));

      map
    }
  }
}

pub fn line_col(pos: usize, input: &str) -> String {
  let (line, col) = {
    let mut pos = pos;
    // Position's pos is always a UTF-8 border.
    let slice = &input[..pos];
    let mut chars = slice.chars().peekable();

    let mut line_col = (1, 1);

    while pos != 0 {
      match chars.next() {
        Some('\r') => {
          if let Some(&'\n') = chars.peek() {
            chars.next();

            if pos == 1 {
              pos -= 1;
            } else {
              pos -= 2;
            }

            line_col = (line_col.0 + 1, 1);
          } else {
            pos -= 1;
            line_col = (line_col.0, line_col.1 + 1);
          }
        }
        Some('\n') => {
          pos -= 1;
          line_col = (line_col.0 + 1, 1);
        }
        Some(c) => {
          pos -= c.len_utf8();
          line_col = (line_col.0, line_col.1 + 1);
        }
        None => unreachable!(),
      }
    }

    line_col
  };

  format!("({}, {})", line - 1, col - 1)
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
