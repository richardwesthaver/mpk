//! MPK_VM -- MAP
//!
//! Mapping of symbol names to unique pointers.
use std::cell::RefCell;
use mpk_hash::FxHashMap as HashMap;
use crate::Arena;

pub struct GlobalMap {
  map: RefCell<HashMap<String, RawPtr<Name>>>,
  arena: Arena,
}

impl GlobalMap {
  pub fn new() -> GlobalMap {
    GlobalMap {
      map: RefCell::new(HashMap::new()),
      arena: Arena::new(),
    }
  }

  pub fn lookup(&self, name: &str) -> RawPtr<Symbol> {
    {
      if let Some(ptr) = self.map.borrow().get(name) {
        return *ptr;
      }
    }
    let name = String::from(name);
    let ptr = self.arena.alloc(Symbol::new(&name)).unwrap();
    self.map.borrow_mut().insert(name, ptr);
    ptr
  }
}

