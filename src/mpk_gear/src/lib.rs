mod err;
pub use err::{Error, Result};
pub mod octatrack;

pub trait DeviceHandle {
  fn info(&self) -> Result<()>;
  fn upload(&self) -> Result<()>;
}
