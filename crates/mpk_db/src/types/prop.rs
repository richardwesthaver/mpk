//! MPK_DB/TYPES -- PROP
use serde::{Deserialize, Serialize};

use super::{Checksum, Edge, EdgeKey, Id, Key, Node, Uri, Val};

pub type PropVec = Vec<Prop>;

impl Val for PropVec {
  type Val = PropVec;
  fn val(&self) -> &PropVec {
    self
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Prop {
  Checksum(Checksum),
  Source(Uri),
  Duration(f64),
  Channels(u16),
  Samplerate(u32),
  Tags(Vec<String>),
  Notes(Vec<String>),
}

impl Val for Prop {
  type Val = Prop;
  fn val(&self) -> &Prop {
    &self
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NodeProp {
  pub id: Id,
  pub prop: Prop,
}

impl Key for NodeProp {
  type Key = Id;
  fn key(&self) -> &Id {
    &self.id
  }
}

impl Val for NodeProp {
  type Val = Prop;
  fn val(&self) -> &Prop {
    &self.prop
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NodeProps {
  pub id: Id,
  pub props: PropVec,
}

impl Key for NodeProps {
  type Key = Id;
  fn key(&self) -> &Id {
    &self.id
  }
}

impl Val for NodeProps {
  type Val = PropVec;
  fn val(&self) -> &PropVec {
    &self.props
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EdgeProp {
  id: EdgeKey,
  prop: Prop,
}

impl Key for EdgeProp {
  type Key = EdgeKey;
  fn key(&self) -> &EdgeKey {
    &self.id
  }
}

impl Val for EdgeProp {
  type Val = Prop;
  fn val(&self) -> &Prop {
    &self.prop
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EdgeProps {
  pub id: EdgeKey,
  pub props: PropVec,
}

impl Key for EdgeProps {
  type Key = EdgeKey;
  fn key(&self) -> &EdgeKey {
    &self.id
  }
}

impl Val for EdgeProps {
  type Val = PropVec;
  fn val(&self) -> &PropVec {
    &self.props
  }
}
