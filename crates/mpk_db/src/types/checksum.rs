//! MPK_DB/TYPES -- CHECKSUM
use std::fmt;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// B3 hash checksum - 256-bit value. We could adjust the OUTPUT_LEN
/// parameter if needed.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Checksum([u8; 32]);

impl Checksum {
  pub fn new<P: AsRef<Path>>(path: P) -> Checksum {
    mpk_hash::Checksum::from_path(&path).into()
  }
}

impl From<mpk_hash::Checksum> for Checksum {
  fn from(c: mpk_hash::Checksum) -> Checksum {
    Checksum(*c.0.as_bytes())
  }
}

impl fmt::Display for Checksum {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let cs: mpk_hash::Checksum = self.0.into();
    write!(f, "{}", cs.to_hex())
  }
}
