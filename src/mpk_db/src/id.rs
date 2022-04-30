//! MPK_DB -- ID
use rkyv::{Serialize, Deserialize, Archive};
use sled::IVec;
#[derive(Eq, PartialEq, Clone, Debug, Hash, Ord, PartialOrd, Serialize, Deserialize, Archive)]
pub struct Id([u8;8]);

impl AsRef<[u8]> for Id {
  fn as_ref(&self) -> &[u8] {
    &self.0
  }
}

impl From<IVec> for Id {
  fn from(v: IVec) -> Id {
    let mut id = [0u8; 8];
    id.copy_from_slice(v.to_vec().as_slice());
    Id(id)
  }
}


impl From<&[u8]> for Id {
  fn from(v: &[u8]) -> Id {
    let mut id = [0u8; 8];
    id.copy_from_slice(v);
    Id(id)
  }
}
