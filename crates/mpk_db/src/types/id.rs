//! MPK_DB/TYPES -- ID
use std::fmt;

use serde::{Deserialize, Serialize};
use sled::IVec;
use ulid::Ulid;

use super::{Key, Val};

pub type IdVec = Vec<Id>;

impl Val for IdVec {
  type Val = IdVec;
  fn val(&self) -> &IdVec {
    &self
  }
}

#[derive(
  Eq, PartialEq, Clone, Copy, Debug, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct Id(u128);

impl fmt::Display for Id {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", Ulid::from(self.0))
  }
}

impl From<[u8; 16]> for Id {
  fn from(bytes: [u8; 16]) -> Id {
    Id(u128::from_be_bytes(bytes))
  }
}

impl From<u128> for Id {
  fn from(n: u128) -> Id {
    Id(n)
  }
}
impl From<IVec> for Id {
  fn from(v: IVec) -> Id {
    let mut buf = [0u8; 16];
    buf.copy_from_slice(&v);
    Id(u128::from_be_bytes(buf))
  }
}

impl From<Vec<u8>> for Id {
  fn from(v: Vec<u8>) -> Id {
    Id::from(v.as_slice())
  }
}

impl From<&[u8]> for Id {
  fn from(v: &[u8]) -> Id {
    let mut buf = [0u8; 16];
    buf.copy_from_slice(v);
    Id(u128::from_be_bytes(buf))
  }
}

impl From<Ulid> for Id {
  fn from(id: Ulid) -> Id {
    Id(id.0)
  }
}

impl Key for Id {
  type Key = u128;
  fn key(&self) -> &u128 {
    &self.0
  }
}

impl From<Id> for Vec<u8> {
  fn from(id: Id) -> Vec<u8> {
    id.into()
  }
}
