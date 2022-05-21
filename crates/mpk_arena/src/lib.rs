//! MPK_ARENA
//!
//! This crate provides Arenas for heap allocation in various contexts
//! such as the mk language runtime.
#![feature(allocator_api)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

mod slab;
use alloc::alloc::Allocator;
use alloc::string::{String, ToString};

use bumpalo::Bump;
use mpk_parser::ast::AstNode;
pub use slab::Slab;

pub trait Objective: Sized {
  fn new(data: AstNode) -> Self;
  fn data(&self) -> AstNode;
  fn ty(&self) -> String;
  fn insert<A: Allocator>(&self, env: &mut Slab<AstNode, A>) -> usize {
    env.insert(self.data())
  }
  fn get<A: Allocator>(key: usize, env: &Slab<AstNode, A>) -> Option<Self> {
    let data = env.get(key);
    if let Some(d) = data {
      Some(Self::new(d.clone()))
    } else {
      None
    }
  }
  fn set<A: Allocator>(&self, key: usize, env: &mut Slab<AstNode, A>) {
    env[key] = self.data();
  }
}

#[derive(Debug)]
pub struct Var {
  data: AstNode,
}

impl Objective for Var {
  fn new(data: AstNode) -> Self {
    Var { data }
  }
  fn data(&self) -> AstNode {
    self.data.clone()
  }
  fn ty(&self) -> String {
    "<var>".to_string()
  }
}

#[derive(Debug)]
pub struct Function {
  data: AstNode,
}

impl Objective for Function {
  fn new(data: AstNode) -> Self {
    Function { data }
  }
  fn data(&self) -> AstNode {
    self.data.clone()
  }
  fn ty(&self) -> String {
    "<function>".to_string()
  }
}

#[derive(Debug)]
pub struct List {
  data: AstNode,
}

impl Objective for List {
  fn new(data: AstNode) -> Self {
    List { data }
  }
  fn data(&self) -> AstNode {
    self.data.clone()
  }
  fn ty(&self) -> String {
    "<list>".to_string()
  }
}

#[derive(Debug)]
pub struct Dict {
  data: AstNode,
}

impl Objective for Dict {
  fn new(data: AstNode) -> Self {
    Dict { data }
  }
  fn data(&self) -> AstNode {
    self.data.clone()
  }
  fn ty(&self) -> String {
    "<dict>".to_string()
  }
}

#[derive(Debug)]
pub struct Table {
  data: AstNode,
}

impl Objective for Table {
  fn new(data: AstNode) -> Self {
    Table { data }
  }
  fn data(&self) -> AstNode {
    self.data.clone()
  }
  fn ty(&self) -> String {
    "<table>".to_string()
  }
}

pub struct Arena<'arena, A: Allocator> {
  vars: Slab<'arena, AstNode, A>,
  functions: Slab<'arena, AstNode, A>,
  lists: Slab<'arena, AstNode, A>,
  dicts: Slab<'arena, AstNode, A>,
  tables: Slab<'arena, AstNode, A>,
}

impl<'arena, A: Allocator> Arena<'_, A> {
  pub fn new(alc: &'arena Bump) -> Arena<&'arena Bump> {
    let vars = Slab::<'arena, AstNode, &Bump>::new_in(alc);
    let functions = Slab::<'arena, AstNode, &Bump>::new_in(alc);
    let lists = Slab::<'arena, AstNode, &Bump>::new_in(alc);
    let dicts = Slab::<'arena, AstNode, &Bump>::new_in(alc);
    let tables = Slab::<'arena, AstNode, &Bump>::new_in(alc);
    Arena {
      vars,
      functions,
      lists,
      dicts,
      tables,
    }
  }

  pub fn insert(&mut self, obj: impl Objective) -> usize {
    match obj.ty().as_str() {
      "<var>" => obj.insert(&mut self.vars),
      "<function>" => obj.insert(&mut self.functions),
      "<list>" => obj.insert(&mut self.lists),
      "<dict>" => obj.insert(&mut self.dicts),
      "<table>" => obj.insert(&mut self.tables),
      _ => 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use mpk_parser::ast;

  use super::*;
  #[test]
  fn arena_test() {
    let bump = Bump::new();
    let mut arena = Arena::<&Bump>::new(&bump);
    let var = |x: i64| Var::new(AstNode::Int(x));
    let fun = |x: i64| {
      Function::new(AstNode::Dyad {
        lhs: Box::new(AstNode::Int(x)),
        verb: ast::DyadicVerb::Plus,
        adverb: Some(ast::AdVerb::Over),
        rhs: Box::new(AstNode::Nouns(vec![AstNode::Int(x), AstNode::Int(x + 1)])),
      })
    };
    let lst =
      |x: i64| List::new(AstNode::Nouns(vec![AstNode::Int(x), AstNode::Int(x - 1)]));
    let dct = |x: i64| {};
    let tbl = |x: i64| {};

    for i in 0..1_000 {
      let var_key = arena.insert(var(i));
      let fun_key = arena.insert(fun(i));
      let lst_key = arena.insert(lst(i));
      let var = Var::get(var_key, &arena.vars);
      let fun = Function::get(fun_key, &arena.functions);
      let lst = List::get(lst_key, &arena.lists);
      dbg!(var, fun, lst);
    }
  }
}
