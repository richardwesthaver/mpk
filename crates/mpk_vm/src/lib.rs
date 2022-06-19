//! MPK_VM -- LIB
#![feature(allocator_api)]
#![feature(iter_intersperse)]
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
