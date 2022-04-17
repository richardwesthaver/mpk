//! MPK_AST
use lalrpop_util::lalrpop_mod;

mod err;
pub use err::ParserError;

mod token;
pub use token::Tok;

mod location;
pub use location::Location;

mod lexer;
pub use lexer::Lexer;

mod parser;

pub mod ast;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn grammar_test() {
    assert!(grammar::ProgParser::new().parse("2287823824738").is_ok());
    assert!(grammar::ProgParser::new()
      .parse("22878238247389999999999999")
      .is_err());
  }
}
