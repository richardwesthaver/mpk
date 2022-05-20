//! MPK_GC
//!
//! This crate provides a GC Arena for the mk language.
#![feature(allocator_api)]
use std::alloc::Allocator;
use slab::Slab;
use bumpalo::Bump;
use mpk_parser::ast::AstNode;

pub trait Objective: Sized {
  fn new(data: AstNode) -> Self;
  fn data(&self) -> AstNode;
  fn ty(&self) -> String;
  fn insert(&self, env: &mut Slab<AstNode>) -> usize {
    env.insert(self.data())
  }
  fn get(key: usize, env: &Slab<AstNode>) -> Self {
    let data = env.get(key);
    Self::new(data.unwrap().clone())
  }
  fn set(&self, key: usize, env: &mut Slab<AstNode>) {
    env[key] = self.data();
  }
}

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

pub struct Arena<A: Allocator> {
  alc: A,
  vars: Slab<AstNode>,
  functions: Slab<AstNode>,
  lists: Slab<AstNode>,
  dicts: Slab<AstNode>,
  tables: Slab<AstNode>,
}

impl<'arena> Arena<&'arena Bump> {
  pub fn new(alc: &'arena Bump) -> Arena<&'arena Bump> {
    let vars = Slab::new();
    let functions = Slab::new();
    let lists = Slab::new();
    let dicts = Slab::new();
    let tables = Slab::new();
    Arena {
      alc,
      vars,
      functions,
      lists,
      dicts,
      tables
    }
  }

  pub fn insert(&mut self, obj: impl Objective) -> usize{
    match obj.ty().as_str() {
      "<var>" => obj.insert(&mut self.vars),
      "<function>" => obj.insert(&mut self.vars),
      "<list>" => obj.insert(&mut self.vars),
      "<dict>" => obj.insert(&mut self.vars),
      "<table>" => obj.insert(&mut self.vars),
      _ => 0,
    }
  }
}
