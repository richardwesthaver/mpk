//! MPK_PARSER -- DECODE
use crate::parser::Prog;

pub fn decode_program(prog: &[&[u8]]) -> Prog {
  let mut data = vec![];
  for node in prog {
    data.push(bincode::deserialize(node).unwrap())
  }
  data
}
