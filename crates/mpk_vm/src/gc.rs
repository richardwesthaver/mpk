//! mpk_vm -- gc.rs
//! garbage collection
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::{ffi::OsStr, fmt};

use crate::{stop, Obj, VmError};
pub static OBJECT_COUNT: AtomicU32 = AtomicU32::new(0);
pub(crate) static MAXIMUM_OBJECTS: u32 = 65_535;

// use rust_gc?
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Gc<T: Clone>(Rc<T>);

impl fmt::Display for Gc<Obj> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl fmt::Display for Gc<String> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

pub fn get_object_count() -> usize {
  OBJECT_COUNT.fetch_add(0, Ordering::SeqCst) as usize
}

impl<T: Clone> Gc<T> {
  // in order to fully sandbox, I have to check the memory limit
  pub fn new(val: T) -> Gc<T> {
    // OBJECT_COUNT.fetch_add(1, Ordering::SeqCst);
    Gc(Rc::new(val))
  }

  pub fn try_new(val: T) -> Result<Gc<T>, VmError> {
    let mem = OBJECT_COUNT.fetch_add(1, Ordering::SeqCst);
    if mem > MAXIMUM_OBJECTS {
      stop!(Generic => "ran out of memory!")
    }
    Ok(Gc(Rc::new(val)))
  }

  pub fn checked_allocate(allocations: usize) -> Result<(), VmError> {
    let mem = OBJECT_COUNT.fetch_add(0, Ordering::SeqCst);
    if mem as usize + allocations > MAXIMUM_OBJECTS as usize {
      stop!(Generic => "allocation would exceed maximum allowed memory")
    }
    Ok(())
  }

  pub fn get_mut(&mut self) -> Option<&mut T> {
    Rc::get_mut(&mut self.0)
  }

  pub fn make_mut(&mut self) -> &mut T {
    Rc::make_mut(&mut self.0)
  }

  pub fn ptr_eq(this: &Self, other: &Self) -> bool {
    Rc::ptr_eq(&this.0, &other.0)
  }

  /// Deep clone the object to remove it from the GC
  pub fn unwrap(&self) -> T {
    (*self.0).clone()
  }

  pub fn as_ptr(&self) -> *const T {
    Rc::as_ptr(&self.0)
  }

  // this does not match the original semantics of Rc::try_unwrap
  // in order to match this, we would need some unsafe rust
  // instead, I take a _slight_ performance hit in order to
  // match the original functionality, and the specific use case
  // for me, which is unwinding lists in the drop for SteelVal
  pub fn try_unwrap(this: Self) -> Result<T, VmError> {
    let inner = Rc::clone(&this.0);
    drop(this);
    Rc::try_unwrap(inner)
      .map_err(|_| VmError::Generic("value still has reference".to_string()))
      .map(|x| {
        OBJECT_COUNT.fetch_sub(1, Ordering::SeqCst);
        x
      })
  }

  pub fn check_memory() -> Result<usize, VmError> {
    let mem = OBJECT_COUNT.fetch_add(0, Ordering::SeqCst);
    if mem > MAXIMUM_OBJECTS {
      stop!(Generic => "out of memory!")
    }
    Ok(mem as usize)
  }
}

impl<T: Clone> AsRef<T> for Gc<T> {
  fn as_ref(&self) -> &T {
    self.0.as_ref()
  }
}

impl<T: Clone> Deref for Gc<T> {
  type Target = T;
  fn deref(&self) -> &T {
    self.0.deref()
  }
}

impl<T: Clone> Drop for Gc<T> {
  fn drop(&mut self) {
    // if Rc::strong_count(&self.0) == 1 {
    //     OBJECT_COUNT.fetch_sub(1, Ordering::SeqCst);
    // }
  }
}

impl<T: Clone> Clone for Gc<T> {
  fn clone(&self) -> Self {
    Gc(Rc::clone(&self.0))
  }
}

impl AsRef<OsStr> for Gc<String> {
  fn as_ref(&self) -> &OsStr {
    self.0.as_ref().as_ref()
  }
}

impl From<&str> for Gc<String> {
  fn from(val: &str) -> Self {
    Gc::new(val.to_string())
  }
}

impl From<String> for Gc<String> {
  fn from(val: String) -> Self {
    Gc::new(val)
  }
}

impl From<&String> for Gc<String> {
  fn from(val: &String) -> Self {
    Gc::new(val.clone())
  }
}

impl AsRef<str> for Gc<String> {
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}
