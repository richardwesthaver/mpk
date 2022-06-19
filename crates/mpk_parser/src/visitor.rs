//! mpk_parser -- visitor
use crate::ast::*;
use mpk_hash::FxHashMap as HashMap;

pub trait VisitorMut {
  type Output;
  fn visit(&mut self, expr: &AstNode) -> Self::Output {
    match expr {
      AstNode::Atom(x) => self.visit_atom(x),
      AstNode::Str(x) => self.visit_str(x),
      AstNode::List(x) => self.visit_list(x),
      AstNode::Map(x) => self.visit_map(x),
      AstNode::Table(x) => self.visit_table(x),
      AstNode::Monad{verb,adverb,expr} => self.visit_monad(verb,adverb,expr),
      AstNode::Dyad{lhs,verb,adverb,rhs} => self.visit_dyad(lhs,verb,adverb,rhs),
      AstNode::SysFn{verb,args} => self.visit_sysfn(verb,args),
      AstNode::UserFn{args,expr} => self.visit_userfn(args,expr),
      AstNode::FnCall{name,args} => self.visit_fncall(name,args),
      AstNode::Var{name,expr} => self.visit_var(name,expr),
    }
  }
  fn visit_atom(&mut self, x: &Atom) -> Self::Output;
  fn visit_str(&self, x: &str) -> Self::Output;
  fn visit_list(&mut self, x: &[AstNode]) -> Self::Output;
  fn visit_map(&mut self, x: &HashMap<String, AstNode>) -> Self::Output;
  fn visit_table(&mut self, x: &HashMap<String, AstNode>) -> Self::Output;
  fn visit_monad(&mut self, x: &MonadicVerb, y: &Option<AdVerb>, z: &Box<AstNode>) -> Self::Output;
  fn visit_dyad(&mut self, w: &Box<AstNode>, x: &DyadicVerb, y: &Option<AdVerb>, z: &Box<AstNode>) -> Self::Output;
  fn visit_sysfn(&mut self, x: &SysVerb, y: &Option<Box<AstNode>>) -> Self::Output;
  fn visit_userfn(&mut self, x: &Option<Vec<String>>, y: &Box<AstNode>) -> Self::Output;
  fn visit_fncall(&mut self, x: &String, y: &Option<Vec<AstNode>>) -> Self::Output;
  fn visit_var(&mut self, x: &String, y: &Box<AstNode>) -> Self::Output;
}

// TODO
pub trait Visitor {
  type Output;
  fn visit(&self, expr: &AstNode) -> Self::Output {
    match expr {
      AstNode::Atom(x) => self.visit_atom(x),
      AstNode::Str(x) => self.visit_str(x),
      AstNode::List(x) => self.visit_list(x),
      AstNode::Map(x) => self.visit_map(x),
      AstNode::Table(x) => self.visit_table(x),
      AstNode::Monad{verb,adverb,expr} => self.visit_monad(verb,adverb,expr),
      AstNode::Dyad{lhs,verb,adverb,rhs} => self.visit_dyad(lhs,verb,adverb,rhs),
      AstNode::SysFn{verb,args} => self.visit_sysfn(verb,args),
      AstNode::UserFn{args,expr} => self.visit_userfn(args,expr),
      AstNode::FnCall{name,args} => self.visit_fncall(name,args),
      AstNode::Var{name,expr} => self.visit_var(name,expr),
    }
  }
  fn visit_atom(&self, x: &Atom) -> Self::Output;
  fn visit_str(&self, x: &str) -> Self::Output;
  fn visit_list(&self, x: &[AstNode]) -> Self::Output;
  fn visit_map(&self, x: &HashMap<String, AstNode>) -> Self::Output;
  fn visit_table(&self, x: &HashMap<String, AstNode>) -> Self::Output;
  fn visit_monad(&self, x: &MonadicVerb, y: &Option<AdVerb>, z: &Box<AstNode>) -> Self::Output;
  fn visit_dyad(&self, w: &Box<AstNode>, x: &DyadicVerb, y: &Option<AdVerb>, z: &Box<AstNode>) -> Self::Output;
  fn visit_sysfn(&self, x: &SysVerb, y: &Option<Box<AstNode>>) -> Self::Output;
  fn visit_userfn(&self, x: &Option<Vec<String>>, y: &Box<AstNode>) -> Self::Output;
  fn visit_fncall(&self, x: &String, y: &Option<Vec<AstNode>>) -> Self::Output;
  fn visit_var(&self, x: &String, y: &Box<AstNode>) -> Self::Output;
}

pub trait ConsumingVisitor {
  type Output;
  fn visit(&mut self, expr: AstNode) -> Self::Output {
    match expr {
      AstNode::Atom(x) => self.visit_atom(x),
      AstNode::Str(x) => self.visit_str(x),
      AstNode::List(x) => self.visit_list(x),
      AstNode::Map(x) => self.visit_map(x),
      AstNode::Table(x) => self.visit_table(x),
      AstNode::Monad{verb,adverb,expr} => self.visit_monad(verb,adverb,expr),
      AstNode::Dyad{lhs,verb,adverb,rhs} => self.visit_dyad(lhs,verb,adverb,rhs),
      AstNode::SysFn{verb,args} => self.visit_sysfn(verb,args),
      AstNode::UserFn{args,expr} => self.visit_userfn(args,expr),
      AstNode::FnCall{name,args} => self.visit_fncall(name,args),
      AstNode::Var{name,expr} => self.visit_var(name,expr),
    }
  }
  fn visit_atom(&mut self, x: Atom) -> Self::Output;
  fn visit_str(&self, x: String) -> Self::Output;
  fn visit_list(&mut self, x: Vec<AstNode>) -> Self::Output;
  fn visit_map(&mut self, x: HashMap<String, AstNode>) -> Self::Output;
  fn visit_table(&mut self, x: HashMap<String, AstNode>) -> Self::Output;
  fn visit_monad(&mut self, x: MonadicVerb, y: Option<AdVerb>, z: Box<AstNode>) -> Self::Output;
  fn visit_dyad(&mut self, w: Box<AstNode>, x: DyadicVerb, y: Option<AdVerb>, z: Box<AstNode>) -> Self::Output;
  fn visit_sysfn(&mut self, x: SysVerb, y: Option<Box<AstNode>>) -> Self::Output;
  fn visit_userfn(&mut self, x: Option<Vec<String>>, y: Box<AstNode>) -> Self::Output;
  fn visit_fncall(&mut self, x: String, y: Option<Vec<AstNode>>) -> Self::Output;
  fn visit_var(&mut self, x: String, y: Box<AstNode>) -> Self::Output;
}
