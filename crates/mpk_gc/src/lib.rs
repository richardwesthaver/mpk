//! MPK_GC
//!
//! This crate provides a GC for the mk language.
pub use gc::{Gc, GcCell, GcCellRef, GcCellRefMut, custom_trace, force_collect, unsafe_empty_trace, finalizer_safe, Finalize, Trace};
