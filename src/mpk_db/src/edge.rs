//! MPK_DB -- EDGE
mod kind;
pub use kind::EdgeKind;

use rkyv::{Serialize, Deserialize, Archive, ser::Serializer};
use crate::ser::{SerializerError, EdgeSerializer};

pub type EdgeVec = Vec<EdgeKind>;

/// A single Edge.
#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[archive_attr(derive(Debug))]
pub struct Edge {
  kind: EdgeKind,
}

impl Edge {
  pub fn new(kind: EdgeKind) -> Edge {
    Edge { kind }
  }
  pub fn serialize<S: Serializer>(&self, ser: &mut EdgeSerializer<S>) -> Result<usize, SerializerError<S::Error>> {
    ser.serialize_value(&self.kind.outbound())
  }
  pub fn key(&self) -> [u8;8] {
    self.kind.inbound().to_be_bytes()
  }
}
