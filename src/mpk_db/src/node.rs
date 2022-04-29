//! MPK_DB -- NODE
mod kind;
pub use kind::NodeKind;

use std::hash::Hasher;
use rkyv::{Archive, Serialize, Deserialize, ser::Serializer};
use crate::ser::{SerializerError, NodeSerializer};

pub type NodeVec = Vec<NodeKind>;

/// Errors that may occur while working with Node types.
#[derive(Debug)]
pub enum NodeError {
  BadNodeKind(String),
}

/// A single Node.
#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[archive_attr(derive(Debug))]
pub struct Node {
  key:  u64,
  val: NodeKind,
}

impl Node {
  pub fn new<H: Hasher>(val: NodeKind, hasher: &mut H) -> Node {
    hasher.write(val.path().as_bytes());
    let key = hasher.finish();
    Node { key, val }
  }
  pub fn serialize<S: Serializer>(&self, ser: &mut NodeSerializer<S>) -> Result<usize, SerializerError<S::Error>> {
    ser.serialize_value(&self.val)
  }
  pub fn key(&self) -> u64 {
    self.key
  }
}
