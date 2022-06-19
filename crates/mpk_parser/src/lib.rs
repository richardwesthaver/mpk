//! MPK_AST
#![feature(
  is_some_with,
  iter_intersperse,
  iterator_try_collect,
  slice_flatten,
  byte_slice_trim_ascii
)]
#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod visitor;
mod parser;
pub mod span;
pub use parser::*;
mod encode;
pub use encode::encode_program;
mod decode;
pub use decode::decode_program;
mod err;
pub use err::*;

#[cfg(test)]
mod tests {}
