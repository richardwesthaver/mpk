//! MPK_VM -- ARENA
use std::alloc::{dealloc, AllocError, Allocator, Layout};
use std::ptr::NonNull;

use bumpalo::Bump;

#[derive(Default)]
pub struct Arena {
  heap: Bump,
}

unsafe impl Sync for Arena {}

impl Arena {
  pub fn allocated_bytes(&self) -> usize {
    self.heap.allocated_bytes()
  }
  pub fn available_bytes(&self) -> usize {
    self.heap.chunk_capacity()
  }
  pub fn alloc<T>(&self, val: T) -> &mut T {
    self.heap.alloc(val)
  }
}

unsafe impl Allocator for Arena {
  fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    self
      .heap
      .try_alloc_layout(layout)
      .map(|p| NonNull::slice_from_raw_parts(p, layout.size()))
      .map_err(|_| AllocError)
  }
  unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {}
}
