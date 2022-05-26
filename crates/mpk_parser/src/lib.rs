//! MPK_AST
#![feature(is_some_with, iter_intersperse)]
#[macro_use]
extern crate pest_derive;

pub mod ast;
mod parser;
pub use parser::*;
mod encode;
pub use encode::encode_program;
mod decode;
pub use decode::decode_program;
mod err;
pub use err::*;

#[cfg(test)]
mod tests {}
