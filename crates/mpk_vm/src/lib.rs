//! MPK_ARENA
//!
//! This crate provides an Arena allocator for the MK language and any
//! other transient data types which need to be allocated by the
//! Engine.
#![feature(allocator_api, nonnull_slice_from_raw_parts)]
mod alloc;
mod arena;
mod block;
mod cons;
mod rawptr;

pub use arena::Arena;
#[cfg(test)]
mod tests {
  use std::alloc::{Allocator, Layout};

  use super::*;
  #[test]
  fn alloc_test() {
    let arena = Arena::default();
    assert_eq!(arena.allocated_bytes(), 0);
    arena
      .allocate_zeroed(unsafe { Layout::from_size_align_unchecked(100, 1) })
      .unwrap();
    assert_eq!(arena.allocated_bytes(), 100);
  }
}
