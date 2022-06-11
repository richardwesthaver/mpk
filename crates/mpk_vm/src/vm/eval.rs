//! MPK_VM -- EVAL
//!
//! Helper functions for evaluating mk expressions
use mpk_parser::ast::{
  AdVerb,
  AstNode::{self, *},
  Atom, DyadicVerb, MonadicVerb,
};

use super::ops::*;
use crate::EvalError;

pub fn eval_dyad(
  lhs: AstNode,
  verb: DyadicVerb,
  adverb: Option<AdVerb>,
  rhs: AstNode,
) -> Result<AstNode, EvalError> {
  let rhs = match rhs {
    Dyad {
      lhs,
      verb,
      adverb,
      rhs,
    } => eval_dyad(*lhs, verb, adverb, *rhs)?,
    Monad { verb, adverb, expr } => eval_monad(verb, adverb, *expr)?,
    rhs => rhs,
  };
  if let Some(ad) = adverb {
    todo!()
  } else {
    match verb {
      DyadicVerb::Plus => add(lhs, rhs),
      _ => todo!(),
    }
  }
}

pub fn eval_monad(
  verb: MonadicVerb,
  adverb: Option<AdVerb>,
  expr: AstNode,
) -> Result<AstNode, EvalError> {
  if let Some(ad) = adverb {
    todo!()
  } else {
    match verb {
      MonadicVerb::Flip => flip(expr),
      _ => todo!(),
    }
  }
}

pub fn eval_noun() {}

pub fn eval_verb() {}

pub fn eval_assign() {}

pub fn eval_sysfn() {}

pub fn eval_userfn() {}

pub fn eval_list(list: Vec<AstNode>) -> Result<AstNode, EvalError> {
  let mut res: Vec<AstNode> = Vec::new();
  for node in list {
    match node {
      Dyad {
        lhs,
        verb,
        adverb,
        rhs,
      } => res.push(eval_dyad(*lhs, verb, adverb, *rhs)?),
      Monad { verb, adverb, expr } => res.push(eval_monad(verb, adverb, *expr)?),
      List(x) => res.push(eval_list(x)?),
      x => res.push(x),
    }
  }
  Ok(List(res))
}
