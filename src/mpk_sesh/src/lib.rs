//! MPK_SESH
//!
//! MPK Session Environment.
mod err;
pub use err::{Error, Result};
use mpk_osc::mpk;

pub struct SeshServer {}

#[cfg(test)]
mod tests {}
