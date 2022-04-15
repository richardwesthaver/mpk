//! MPK_REPL PARSER::TOKEN
pub enum Token {
  Array,
  Symbol(String),
  Int(i64),
  Float(f32),
  Double(f64),
  Op(Op),
  Fn(Fn),
}

pub enum Op {
  Less,
  Greater,
  Equal,
}

pub enum Fn {
  System,
  Shell,
}
