//! MPK_DB -- TREE
use sled::{Tree, IVec, Error, CompareAndSwapError};
use mpk_hash::Djb2;
use rkyv::{archived_root, Deserialize, Infallible};
use crate::{DbRef, Factory, NodeFactory, NodeKind};

pub const TREE_NAMES: [&str; 20] = [
  "tracks",
  "samples",
  "midi",
  "patches",
  "track_meta",
  "sample_meta",
  "midi_meta",
  "patch_meta",
  "seq",
  "coll",
  "crate",
  "seq_edge",
  "seq_edge_rev",
  "coll_edge",
  "coll_edge_rev",
  "crate_edge",
  "crate_edge_rev",
  "sesh",
  "sesh_edge",
  "sesh_edge_rev",
];

pub trait TreeHandle {
  type Factory;
  type Kind;
  type Handle;
  fn open(handle: Self::Handle, name: &str) -> Result<Self, Error> where Self: Sized;
  fn insert(&mut self, kind: Self::Kind) -> Result<Option<Self::Kind>, Error>;
  fn get(&mut self, key: u64) -> Result<Option<Self::Kind>, Error>;
  fn remove(&mut self, key: u64) -> Result<Option<Self::Kind>, Error>;
  fn swap(&mut self, key: u64, old: Option<Self::Kind>, new: Option<Self::Kind>) -> Result<Result<(), CompareAndSwapError>, Error>;
}

pub struct NodeTree {
  tree: Tree,
  factory: NodeFactory<1024, Djb2>,
}

impl NodeTree {
  pub fn open(handle: DbRef, name: &str) -> Result<NodeTree, Error> {
    let factory = NodeFactory::<1024, Djb2>::new();
    handle.open_tree(name).map(|tree| NodeTree {tree, factory})
  }
  pub fn insert(&mut self, kind: NodeKind) -> Result<Option<NodeKind>, Error> {
    let mut ser = self.factory.serializer();
    let node = self.factory.create(kind);
    self.factory.serialize_val(&node, &mut ser).unwrap();
    match self.tree.insert(node.key().to_be_bytes(), self.factory.flush_bytes(ser)) {
      Ok(o) => if let Some(v) = o {
	let v = v.to_vec();
	let archived = unsafe { archived_root::<NodeKind>(v.as_slice()) };
	let val = archived.deserialize(&mut Infallible).ok();
	Ok(val)
      } else {
	Ok(None)
      }
      Err(e) => Err(e)
    }
  }
}
