//! MPK_DB -- TYPES
mod checksum;
mod edge;
mod id;
mod meta;
mod node;
mod prop;
mod uri;

pub use checksum::Checksum;
pub use edge::{Edge, EdgeKey, EdgeKind, EdgeVec};
pub use id::{Id, IdVec};
pub use meta::{Meta, MetaKind};
pub use node::{Node, NodeKind, NodeVec};
pub use prop::{EdgeProp, EdgeProps, NodeProp, NodeProps, Prop, PropVec};
use serde::Serialize;
pub use uri::Uri;

pub trait Key {
  type Key: Serialize;
  fn key(&self) -> &Self::Key;
}

pub trait Val: Serialize {
  type Val;
  fn val(&self) -> &Self::Val;
}
