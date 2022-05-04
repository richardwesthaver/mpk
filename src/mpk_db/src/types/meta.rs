//! MPK_DB/TYPES -- META
use super::{IdVec, Key, Uri, Val};
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum MetaKind {
  Path(Uri),
  Artist(String),
  Album(String),
  Playlist(String),
  Crate(String),
  Genre(String),
}

impl Key for MetaKind {
  type Key = MetaKind;
  fn key(&self) -> &MetaKind {
    &self
  }
}

impl From<Vec<u8>> for MetaKind {
  fn from(vec: Vec<u8>) -> MetaKind {
    deserialize(&vec).unwrap()
  }
}

impl From<&[u8]> for MetaKind {
  fn from(v: &[u8]) -> MetaKind {
    deserialize(v).unwrap()
  }
}

impl<'a> From<&MetaKind> for &'a [u8] {
  fn from(kind: &MetaKind) -> &'a [u8] {
    kind.into()
  }
}

impl From<MetaKind> for Vec<u8> {
  fn from(kind: MetaKind) -> Vec<u8> {
    serialize(&kind).unwrap()
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Meta {
  id: MetaKind,
  nodes: IdVec,
}

impl Key for Meta {
  type Key = MetaKind;
  fn key(&self) -> &MetaKind {
    &self.id
  }
}

impl Val for Meta {
  type Val = IdVec;
  fn val(&self) -> &IdVec {
    &self.nodes
  }
}
