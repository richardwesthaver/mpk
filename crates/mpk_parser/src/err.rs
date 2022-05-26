//! MPK_PARSER -- ERR
use std::fmt;

use mpk_hash::FxHashMap as HashMap;
pub use pest::error::{Error as PestError, ErrorVariant, InputLocation};

use crate::parser::Rule;

#[derive(Debug)]
pub enum Error {
  PestErr(PestError<Rule>),
  InvalidNoun(String, String),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Error::PestErr(ref e) => Some(e),
      Error::InvalidNoun(..) => None,
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::PestErr(ref e) => e.fmt(f),
      Error::InvalidNoun(expected, found) => {
        f.write_str(format!("expected {}, found {}", expected, found).as_str())
      }
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
