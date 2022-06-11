//! MPK_VM -- LIB
#![feature(allocator_api)]
#![feature(iter_intersperse)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

pub mod c;
pub mod e;
pub mod gc;
pub mod i;
pub mod m;
pub mod o;
pub mod vm;

pub use e::{EvalError, Result, VmError};
pub use gc::Gc;
pub use m::{Arena, Bump};
pub use o::Obj;
pub use vm::Vm;
