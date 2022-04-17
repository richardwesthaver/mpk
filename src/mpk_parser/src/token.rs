use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
  Name(String),
  Int(i64),
  Float(f32),
  Double(f64),
  String(String),
  Byte(u8),
  Newline,
  Eof,
  Lpar,
  Rpar,
  Lsqb,
  Rsqb,
  Lbr,
  Rbr,
  Colon,
  Comma,
  Semi,
  Plus,
  Minus,
  Star,
  Slash,
  Vbar,
  Amper,
  Lt,
  Gt,
  Eq,
  Dot,
  Percent,
  EqEq,
  NotEq,
  LtEq,
  GtEq,
  Tilde,
  Caret,
  LShift,
  RShift,
  StarStar,
  StarEq,
  PlusEq,
  MinusEq,
  SlashEq,
  LShiftEq,
  RShiftEq,
  SlashSlash,
  ColonEq,
  At,
  AtEq,
  RArrow,
  LArrow,
}

impl fmt::Display for Tok {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use Tok::*;
    match self {
      Name(s) => write!(f, "'{}'", s),
      Int(n) => write!(f, "'{}'", n),
      Float(n) => write!(f, "'{}'", n),
      _ => unimplemented!(),
    }
  }
}
