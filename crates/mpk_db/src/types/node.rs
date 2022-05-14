//! MPK_DB/TYPES -- NODE
use std::hash::{Hash, Hasher};

use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use sled::IVec;
use ulid::Ulid;

use super::{Id, Key, Val};

pub type NodeVec = Vec<Node>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum NodeKind {
  Track,
  Sample,
  Midi,
  Patch,
}

impl Val for NodeKind {
  type Val = Self;
  fn val(&self) -> &Self {
    self
  }
}

impl From<&[u8]> for NodeKind {
  fn from(v: &[u8]) -> NodeKind {
    deserialize(v).unwrap()
  }
}

impl<'a> From<&NodeKind> for &'a [u8] {
  fn from(kind: &NodeKind) -> &'a [u8] {
    kind.into()
  }
}

impl From<&NodeKind> for Vec<u8> {
  fn from(kind: &NodeKind) -> Vec<u8> {
    serialize(kind).unwrap()
  }
}

impl From<Vec<u8>> for NodeKind {
  fn from(vec: Vec<u8>) -> NodeKind {
    deserialize(&vec).unwrap()
  }
}

impl From<IVec> for NodeKind {
  fn from(iv: IVec) -> NodeKind {
    deserialize(&iv).unwrap()
  }
}

impl From<&IVec> for NodeKind {
  fn from(iv: &IVec) -> NodeKind {
    deserialize(iv).unwrap()
  }
}

/// A single Node.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Node {
  id: Id,
  ty: NodeKind,
}

impl Node {
  pub fn new(ty: NodeKind) -> Node {
    Node::with_id(Id::from(Ulid::new()), ty)
  }
  pub fn with_id<I: Into<Id>>(id: I, ty: NodeKind) -> Node {
    Node { id: id.into(), ty }
  }
}

impl Hash for Node {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.key().hash(state);
  }
}

impl Key for Node {
  type Key = Id;
  fn key(&self) -> &Id {
    &self.id
  }
}

impl Val for Node {
  type Val = NodeKind;
  fn val(&self) -> &NodeKind {
    &self.ty
  }
}
