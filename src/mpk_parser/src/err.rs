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
