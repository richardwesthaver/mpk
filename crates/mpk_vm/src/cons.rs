//! MPK_VM -- CONS
//!
//! Constant values for VM allocators
use std::mem::size_of;

pub const BLOCK_SIZE: usize = 32 * 1024;
pub const LINE_SIZE: usize = 128;
pub const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;
pub const MAX_ALLOC_SIZE: usize = std::u32::MAX as usize;

/// The first object in a block is not at offset 0 - that location is reserved
/// for a pointer to the BlockMeta struct for the Block - but at the next
/// double-word offset.
pub const FIRST_OBJECT_OFFSET: usize = size_of::<usize>() * 2;
pub const BLOCK_CAPACITY: usize = BLOCK_SIZE - FIRST_OBJECT_OFFSET;

/// Object size
pub const MEDIUM_OBJECT: usize = LINE_SIZE;
pub const LARGE_OBJECT: usize = 8 * 1024;

/// Whether evacuation should be used or not.
pub const USE_EVACUATION: bool = true;

/// The number of blocks stored into the `EvacAllocator` for evacuation.
pub const EVAC_HEADROOM: usize = 5;

/// Ratio when to trigger evacuation collection.
pub const EVAC_TRIGGER_THRESHHOLD: f64 = 0.25;
