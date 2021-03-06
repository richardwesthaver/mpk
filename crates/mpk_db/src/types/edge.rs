//! MPK_DB/TYPES -- EDGE
use std::fmt;

use bincode::{deserialize, serialize};
use mpk_util::timestamp_nanos;
use serde::{Deserialize, Serialize};

use super::Id;
use super::{Key, Val};

pub type EdgeVec = Vec<Edge>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum EdgeKind {
  Next,
  Similar,
  Compliment,
  Compose,
}

impl fmt::Display for EdgeKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      EdgeKind::Next => write!(f, "next"),
      EdgeKind::Similar => write!(f, "similar"),
      EdgeKind::Compliment => write!(f, "compliment"),
      EdgeKind::Compose => write!(f, "compose"),
    }
  }
}

impl From<Vec<u8>> for EdgeKind {
  fn from(v: Vec<u8>) -> EdgeKind {
    deserialize(&v).unwrap()
  }
}

impl From<&[u8]> for EdgeKind {
  fn from(v: &[u8]) -> EdgeKind {
    deserialize(v).unwrap()
  }
}

impl From<EdgeKind> for Vec<u8> {
  fn from(key: EdgeKind) -> Vec<u8> {
    key.into()
  }
}

/// An EdgeKey.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct EdgeKey {
  inbound: Id,
  kind: EdgeKind,
  outbound: Id,
}

impl EdgeKey {
  pub fn new<I: Into<Id>>(kind: EdgeKind, inbound: I, outbound: I) -> EdgeKey {
    EdgeKey {
      inbound: inbound.into(),
      kind,
      outbound: outbound.into(),
    }
  }

  pub fn reverse(&self) -> EdgeKey {
    EdgeKey::new(self.kind, self.outbound.clone(), self.inbound.clone())
  }
}

impl fmt::Display for EdgeKey {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}:[{} -> {}]", self.kind, self.inbound, self.outbound)
  }
}

impl From<Vec<u8>> for EdgeKey {
  fn from(v: Vec<u8>) -> EdgeKey {
    deserialize(&v).unwrap()
  }
}

impl From<&[u8]> for EdgeKey {
  fn from(v: &[u8]) -> EdgeKey {
    deserialize(v).unwrap()
  }
}

impl From<EdgeKey> for Vec<u8> {
  fn from(key: EdgeKey) -> Vec<u8> {
    key.into()
  }
}

impl Key for EdgeKey {
  type Key = Self;
  fn key(&self) -> &Self {
    self
  }
}

impl Val for u128 {
  type Val = Self;
  fn val(&self) -> &Self {
    self
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Edge {
  key: EdgeKey,
  created: u128,
}

impl Edge {
  pub fn new(key: EdgeKey) -> Edge {
    Edge::with_time(key, timestamp_nanos())
  }

  pub fn with_time(key: EdgeKey, created: u128) -> Edge {
    Edge { key, created }
  }
}

impl Key for Edge {
  type Key = EdgeKey;
  fn key(&self) -> &EdgeKey {
    &self.key
  }
}

impl Val for Edge {
  type Val = u128;
  fn val(&self) -> &u128 {
    &self.created
  }
}
