//! mpk_vm/vm -- env.rs
//! environment table
use crate::{Obj, Result};
#[derive(Debug)]
pub struct Env {
  pub(crate) bindings_vec: Vec<Obj>,
}

pub trait MacroEnv {
  fn validate_identifier(&self, name: &str) -> bool;
}

impl Env {
  pub fn extract(&self, idx: usize) -> Option<Obj> {
    self.bindings_vec.get(idx).cloned()
  }

  /// top level global env has no parent
  pub fn root() -> Self {
    Env {
      bindings_vec: Vec::new(),
    }
  }

  /// Search starting from the current environment
  /// for `idx`, looking through the parent chain in order.
  ///
  /// if found, return that value
  ///
  /// Otherwise, error with `FreeIdentifier`
  // #[inline]
  pub fn repl_lookup_idx(&self, idx: usize) -> Result<Obj> {
    Ok(self.bindings_vec[idx].clone())
  }

  #[inline]
  pub fn repl_define_idx(&mut self, idx: usize, val: Obj) {
    if idx < self.bindings_vec.len() {
      self.bindings_vec[idx] = val;
    } else {
      self.bindings_vec.push(val);
      assert_eq!(self.bindings_vec.len() - 1, idx);
    }
  }

  pub fn repl_set_idx(&mut self, idx: usize, val: Obj) -> Result<Obj> {
    let output = self.bindings_vec[idx].clone();
    self.bindings_vec[idx] = val;
    Ok(output)
  }

  #[inline]
  pub fn add_root_value(&mut self, idx: usize, val: Obj) {
    // self.bindings_map.insert(idx, val);
    self.repl_define_idx(idx, val);
  }
}
