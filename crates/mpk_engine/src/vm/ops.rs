//! MPK_ENGINE -- VM/OPS
//!
//! Individual mk operators
use mpk_parser::ast::{
  AstNode::{self, *},
  *,
};

use super::eval::*;
use crate::err::EvalError;

pub fn add(lhs: AstNode, rhs: AstNode) -> Result<String, EvalError> {
  match (lhs, rhs) {
    (Int(x), Int(y)) => Ok((x + y).to_string()),
    (Int(x), Float(y)) => Ok((x as f64 + y).to_string()),
    (Int(x), List(y)) => Ok(
      y.iter()
        .map(|y| add(Int(x), y.clone()).unwrap())
        .intersperse(" ".to_string())
        .collect(),
    ),
    (
      Int(x),
      Dyad {
        lhs,
        verb,
        adverb,
        rhs,
      },
    ) => eval_dyad(
      Int(x),
      DyadicVerb::Plus,
      None,
      Dyad {
        lhs,
        verb,
        adverb,
        rhs,
      },
    ),
    (Float(x), Float(y)) => Ok((x + y).to_string()),
    (Float(x), Int(y)) => Ok((x + y as f64).to_string()),
    (Float(x), List(y)) => Ok(
      y.iter()
        .map(|y| add(Float(x), y.clone()).unwrap())
        .intersperse(" ".to_string())
        .collect(),
    ),
    (List(x), List(y)) => {
      if x.len() == y.len() {
        Ok(
          y.iter()
            .zip(x)
            .map(|(x, y)| add(x.clone(), y).unwrap())
            .intersperse(" ".to_string())
            .collect(),
        )
      } else {
        Err(EvalError::Length)
      }
    }
    (List(x), Int(y)) => Ok(
      x.iter()
        .map(|x| add(x.clone(), Int(y)).unwrap())
        .intersperse(" ".to_string())
        .collect(),
    ),
    (List(x), Float(y)) => Ok(
      x.iter()
        .map(|x| add(x.clone(), Float(y)).unwrap())
        .intersperse(" ".to_string())
        .collect(),
    ),
    _ => todo!(),
  }
}

pub fn flip(expr: AstNode) -> Result<String, EvalError> {
  match expr {
    Int(x) => Ok(x.to_string()),
    Float(x) => Ok(x.to_string()),
    List(x) => Ok(eval_list(x.into_iter().rev().collect())?),
    _ => Err(EvalError::Rank),
  }
}
