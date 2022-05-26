//! MPK_PARSER -- DECODE
use crate::ast::*;

pub fn decode_program(prog: &[&[u8]]) -> Program {
  let mut data = vec![];
  for node in prog {
    data.push(bincode::deserialize(node).unwrap())
  }
  data
}
