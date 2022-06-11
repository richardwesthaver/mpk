//! MPK_PARSER -- ENCODE
use crate::parser::Prog;

pub fn encode_program(prog: Prog) -> Vec<Vec<u8>> {
  let mut data = vec![];
  for node in prog {
    data.push(bincode::serialize(&node).unwrap());
  }
  data
}
