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
pub use prop::{EdgeProp, EdgeProps, NodeProp, NodeProps, Prop};
pub use uri::Uri;

use serde::Serialize;

pub trait Key: Serialize {
  type Key;
  fn key(&self) -> &Self::Key;
}

pub trait Val: Serialize {
  type Val;
  fn val(&self) -> &Self::Val;
}
