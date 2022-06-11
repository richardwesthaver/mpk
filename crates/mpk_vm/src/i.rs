//! MPK_VM -- i
//!
//! vm instruction set
use mpk_parser::span::Span;
use mpk_parser::Node;
use serde::{Deserialize, Serialize};
#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum Op {
  VOID = 0,
  PUSH = 1,
  LOOKUP = 2,
  IF = 3,
  JMP = 4,
  FUNC = 5,
  SCLOSURE = 6,
  ECLOSURE = 7,
  STRUCT = 8,
  POP = 9,
  BIND = 10,
  SDEF = 11,
  EDEF = 12,
  PASS = 13,
  PUSHCONST = 14,
  NDEFS = 15,
  EVAL = 16,
  PANIC = 17,
  CLEAR = 18,
  TAILCALL = 19,
  SET,
  TCOJMP,
}

/// Instruction loaded with lots of information prior to being condensed
/// Includes the opcode and the payload size, plus some information
/// used for locating spans and pretty error messages
#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
  pub op: Op,
  pub payload_size: usize,
  pub contents: Option<Node>,
  pub constant: bool,
}

impl Instruction {
  pub fn new(
    op: Op,
    payload_size: usize,
    contents: Node,
    constant: bool,
  ) -> Instruction {
    Instruction {
      op,
      payload_size,
      contents: Some(contents),
      constant,
    }
  }

  pub fn new_panic(span: Node) -> Instruction {
    Instruction {
      op: Op::PANIC,
      payload_size: 0,
      contents: Some(span),
      constant: false,
    }
  }
  pub fn new_pop() -> Instruction {
    Instruction {
      op: Op::POP,
      payload_size: 0,
      contents: None,
      constant: false,
    }
  }
  pub fn new_if(true_jump: usize) -> Instruction {
    Instruction {
      op: Op::IF,
      payload_size: true_jump,
      contents: None,
      constant: false,
    }
  }

  pub fn new_jmp(jump: usize) -> Instruction {
    Instruction {
      op: Op::JMP,
      payload_size: jump,
      contents: None,
      constant: false,
    }
  }

  pub fn new_tco_jmp() -> Instruction {
    Instruction {
      op: Op::TCOJMP,
      payload_size: 0,
      contents: None,
      constant: false,
    }
  }
  pub fn new_sclosure() -> Instruction {
    Instruction {
      op: Op::SCLOSURE,
      payload_size: 0,
      contents: None,
      constant: false,
    }
  }

  pub fn new_eclosure(arity: usize) -> Instruction {
    Instruction {
      op: Op::ECLOSURE,
      payload_size: arity,
      contents: None,
      constant: false,
    }
  }

  pub fn new_bind(contents: Node) -> Instruction {
    Instruction {
      op: Op::BIND,
      payload_size: 0,
      contents: Some(contents),
      constant: false,
    }
  }

  pub fn new_sdef() -> Instruction {
    Instruction {
      op: Op::SDEF,
      payload_size: 0,
      contents: None,
      constant: false,
    }
  }

  pub fn new_edef() -> Instruction {
    Instruction {
      op: Op::EDEF,
      payload_size: 0,
      contents: None,
      constant: false,
    }
  }

  pub fn new_void() -> Instruction {
    Instruction {
      op: Op::VOID,
      payload_size: 0,
      contents: None,
      constant: false,
    }
  }

  pub fn new_pass(arity: usize) -> Instruction {
    Instruction {
      op: Op::PASS,
      payload_size: arity,
      contents: None,
      constant: false,
    }
  }

  pub fn new_set() -> Instruction {
    Instruction {
      op: Op::SET,
      payload_size: 0,
      contents: None,
      constant: false,
    }
  }
  pub fn new_struct(idx: usize) -> Instruction {
    Instruction {
      op: Op::STRUCT,
      payload_size: idx,
      contents: None,
      constant: true,
    }
  }
}

pub fn compact(instructions: Vec<Instruction>) -> Vec<Ins> {
  instructions.into_iter().map(|x| x.into()).collect()
}

pub fn pretty_print_ins(instrs: &[Ins]) {
  for (i, instruction) in instrs.iter().enumerate() {
    println!(
      "{}    {:?} : {}",
      i, instruction.op, instruction.payload_size
    );
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct Ins {
  pub op: Op,
  pub payload_size: u32,
  pub span: Span,
}

impl Ins {
  pub fn new(op: Op, payload_size: u32, span: Span) -> Ins {
    Ins {
      op,
      payload_size,
      span,
    }
  }
}

impl From<Instruction> for Ins {
  fn from(val: Instruction) -> Ins {
    Ins::new(
      val.op,
      val.payload_size.try_into().unwrap(),
      if let Some(syn) = val.contents {
        syn.1
      } else {
        Span::new(0, 0)
      },
    )
  }
}
