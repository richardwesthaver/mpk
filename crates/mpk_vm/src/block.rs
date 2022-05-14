//! MPK_VM -- BLOCK
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

pub type BlockPtr = NonNull<u8>;
pub type BlockSize = usize;

#[derive(Debug, PartialEq)]
pub enum BlockError {
  /// Usually means requested block size, and therefore alignment, wasn't a
  /// power of two
  BadRequest,
  /// Insufficient memory, couldn't allocate a block
  OOM,
}

pub struct Block {
  ptr: BlockPtr,
  size: BlockSize,
}

impl Block {
  /// Instantiate a new block of the given size. Size must be a power of two.
  pub fn new(size: BlockSize) -> Result<Block, BlockError> {
    if !size.is_power_of_two() {
      return Err(BlockError::BadRequest);
    }

    Ok(Block {
      ptr: alloc_block(size)?,
      size,
    })
  }

  /// Consume and return the pointer only
  pub fn into_mut_ptr(self) -> BlockPtr {
    self.ptr
  }

  /// Return the size in bytes of the block
  pub fn size(&self) -> BlockSize {
    self.size
  }

  /// Unsafely reassemble from pointer and size
  pub unsafe fn from_raw_parts(ptr: BlockPtr, size: BlockSize) -> Block {
    Block { ptr, size }
  }

  /// Return a bare pointer to the base of the block
  pub fn as_ptr(&self) -> *const u8 {
    self.ptr.as_ptr()
  }
}

impl Drop for Block {
  fn drop(&mut self) {
    dealloc_block(self.ptr, self.size);
  }
}

pub fn alloc_block(size: BlockSize) -> Result<BlockPtr, BlockError> {
  unsafe {
    let layout = Layout::from_size_align_unchecked(size, size);

    let ptr = alloc(layout);
    if ptr.is_null() {
      Err(BlockError::OOM)
    } else {
      Ok(NonNull::new_unchecked(ptr))
    }
  }
}

pub fn dealloc_block(ptr: BlockPtr, size: BlockSize) {
  unsafe {
    let layout = Layout::from_size_align_unchecked(size, size);

    dealloc(ptr.as_ptr(), layout);
  }
}

#[cfg(test)]
mod tests {
  use super::{Block, BlockError, BlockSize};

  fn alloc_dealloc(size: BlockSize) -> Result<(), BlockError> {
    let block = Block::new(size)?;
    // the block address bitwise AND the alignment bits (size - 1) should
    // be a mutually exclusive set of bits
    let mask = size - 1;
    assert!((block.ptr.as_ptr() as usize & mask) ^ mask == mask);
    drop(block);
    Ok(())
  }

  #[test]
  fn test_bad_sizealign() {
    assert!(alloc_dealloc(999) == Err(BlockError::BadRequest))
  }

  #[test]
  fn test_4k() {
    assert!(alloc_dealloc(4096).is_ok())
  }

  #[test]
  fn test_32k() {
    assert!(alloc_dealloc(32768).is_ok())
  }

  #[test]
  fn test_16m() {
    assert!(alloc_dealloc(16 * 1024 * 1024).is_ok())
  }
}
