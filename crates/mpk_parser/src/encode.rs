//! MPK_PARSER -- ENCODE
use crate::ast::*;

pub fn encode_program(prog: Program) -> Vec<Vec<u8>> {
  let mut data = vec![];
  for node in prog {
    data.push(bincode::serialize(&node).unwrap());
  }
  data
}
