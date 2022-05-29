//! MPK_ENGINE -- VM
use core::alloc::Allocator;

use mpk_arena::{Arena, Bump};
use mpk_osc::mpk::server::ResultMessageKind;
use mpk_parser::ast::{AdVerb, AstNode, DyadicVerb, Program};

use crate::err::VmError;
mod eval;
mod ops;
use eval::*;

pub struct Vm<'vm, A: Allocator> {
  arena: Arena<'vm, A>,
}

impl<'vm, A: Allocator> Vm<'_, A> {
  pub fn new(alc: &'vm Bump) -> Vm<&'vm Bump> {
    let arena = Arena::<&'vm Bump>::new(alc);
    Vm { arena }
  }
  pub fn eval(&self, program: Program) -> Result<ResultMessageKind, VmError> {
    let mut res: Vec<String> = Vec::new();
    for node in program {
      match node {
        AstNode::Dyad {
          lhs,
          verb,
          adverb,
          rhs,
        } => res.push(
          eval_dyad(*lhs, verb, adverb, *rhs)
            .map_err(|e| VmError::from(e))?
            .to_string(),
        ),
        AstNode::Monad { verb, adverb, expr } => {
          res.push(eval_monad(verb, adverb, *expr)?.to_string())
        }
        AstNode::Atom(x) => res.push(x.to_string()),
        AstNode::Str(x) => {
          res.push(String::from_utf8(x.into_iter().map(|c| *c).collect()).unwrap())
        }
        AstNode::Symbol(x) => res.push(x.to_string()),
        AstNode::List(x) => res.push(eval_list(x)?.to_string()),
        x => res.push(x.to_string()),
      }
    }
    let res = res.into_iter().intersperse("\n".to_string()).collect();

    Ok(ResultMessageKind::Ok(res))
  }
}
