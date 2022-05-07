//! MPK_DB/TYPES -- PROP
use serde::{Deserialize, Serialize};

use super::{Checksum, Edge, EdgeKey, Id, Key, Node, Uri, Val};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Prop {
  Checksum(Checksum),
  Source(Uri),
  Genre(String),
  Tags(Vec<String>),
  Notes(Vec<String>),
}

impl Val for Prop {
  type Val = Prop;
  fn val(&self) -> &Prop {
    &self
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct NodeProp {
  id: Id,
  prop: Prop,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct NodeProps {
  id: Node,
  props: Vec<Prop>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EdgeProps {
  id: Edge,
  props: Vec<Prop>,
}
