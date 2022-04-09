mod err;
pub use err::{Error, Result};

pub mod octatrack;

pub trait DeviceHandle {
  fn info(&self) -> Result<()>;
  fn upload(&self) -> Result<()>;
  fn mount(&self) -> Result<()>;
  fn umount(&self) -> Result<()>;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mount() {
  }
}
