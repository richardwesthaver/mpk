//! MPK_REPL PARSER
use crate::Result;

mod token;
pub use token::Token;

mod lexer;
pub use lexer::Lexer;

mod location;
pub use location::Location;

pub fn tokenize(line: &str) -> Result<Vec<Token>> {
  Ok(vec![])
}
