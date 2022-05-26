//! MPK_VM -- EVAL
//!
//! Helper functions for evaluating mk expressions
use mpk_parser::ast::{
  AdVerb,
  AstNode::{self, *},
  DyadicVerb, MonadicVerb,
};

use super::ops::*;

pub fn eval_dyad(
  lhs: AstNode,
  verb: DyadicVerb,
  adverb: Option<AdVerb>,
  rhs: AstNode,
) -> String {
  if let Some(ad) = adverb {
    todo!()
  } else {
    match verb {
      DyadicVerb::Plus => add(lhs, rhs),
      _ => todo!(),
    }
  }
}

pub fn eval_monad(verb: MonadicVerb, adverb: Option<AdVerb>, expr: AstNode) -> String {
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

pub fn eval_list(list: Vec<AstNode>) -> String {
  let mut res: Vec<String> = Vec::new();
  for node in list {
    match node {
      Dyad {
        lhs,
        verb,
        adverb,
        rhs,
      } => res.push(eval_dyad(*lhs, verb, adverb, *rhs)),
      Monad { verb, adverb, expr } => res.push(eval_monad(verb, adverb, *expr)),
      Int(x) => res.push(x.to_string()),
      Float(x) => res.push(x.to_string()),
      Name(x) => res.push(x),
      Str(x) => res.push(x),
      Symbol(x) => res.push(x),
      List(x) => res.push(eval_list(x).split("\n").intersperse(" ").collect()),
      _ => todo!(),
    }
  }
  let res = res.into_iter().intersperse("\n".to_string()).collect();
  res
}
