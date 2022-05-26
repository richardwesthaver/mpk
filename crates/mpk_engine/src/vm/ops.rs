//! MPK_ENGINE -- VM/OPS
//!
//! Individual mk operators
use mpk_parser::ast::AstNode::{self, *};

use super::eval::*;

pub fn add(lhs: AstNode, rhs: AstNode) -> String {
  match (lhs, rhs) {
    (Int(x), Int(y)) => (x + y).to_string(),
    (Int(x), Float(y)) => (x as f64 + y).to_string(),
    (Int(x), List(y)) => y
      .iter()
      .map(|y| add(Int(x), y.clone()))
      .intersperse(" ".to_string())
      .collect(),
    (Float(x), Float(y)) => (x + y).to_string(),
    (Float(x), Int(y)) => (x + y as f64).to_string(),
    (Float(x), List(y)) => y
      .iter()
      .map(|y| add(Float(x), y.clone()))
      .intersperse(" ".to_string())
      .collect(),
    (List(x), List(y)) => {
      if x.len() == y.len() {
        y.iter()
          .zip(x)
          .map(|(x, y)| add(x.clone(), y))
          .intersperse(" ".to_string())
          .collect()
      } else {
        todo!()
      }
    }
    (List(x), Int(y)) => x
      .iter()
      .map(|x| add(x.clone(), Int(y)))
      .intersperse(" ".to_string())
      .collect(),
    (List(x), Float(y)) => x
      .iter()
      .map(|x| add(x.clone(), Float(y)))
      .intersperse(" ".to_string())
      .collect(),
    _ => todo!(),
  }
}

pub fn flip(expr: AstNode) -> String {
  match expr {
    Int(x) => x.to_string(),
    Float(x) => x.to_string(),
    List(x) => eval_list(x.into_iter().rev().collect()),
    _ => todo!(),
  }
}
