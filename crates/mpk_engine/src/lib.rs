//! MPK_ENGINE
#![feature(allocator_api, iter_intersperse)]
mod err;
pub use err::{Error, Result};

mod engine;
pub mod proxy;
pub use engine::Engine;

#[cfg(test)]
mod tests {}
