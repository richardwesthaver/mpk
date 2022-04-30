//! MPK_DB -- TREE
use std::ops::Deref;

use sled::{Tree, IVec, Error, CompareAndSwapError, transaction::TransactionResult, Batch, Subscriber, Db, MergeOperator};
use mpk_hash::Djb2;
use rkyv::{archived_root, Deserialize, Infallible, Archive};

use crate::{DbRef, Factory, NodeFactory, NodeKind, Id, EdgeKind, EdgeFactory};

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

pub trait TreeHandle: Sized {
  type Kind: Archive;
  type Handle: AsRef<Db>;
  type Factory: Factory;
  fn open(handle: Self::Handle, name: &str) -> Result<Self, Error>;
  fn insert(&mut self, val: Self::Kind) -> Result<Option<Self::Kind>, Error>;
  fn get(&mut self, key: Id) -> Result<Option<Self::Kind>, Error>;
  fn exists(&self, key: Id) -> Result<bool, Error>;
  fn remove(&mut self, key: Id) -> Result<Option<Self::Kind>, Error>;
  fn swap(&mut self, key: Id, old: Option<Self::Kind>, new: Option<Self::Kind>) -> Result<Result<(), CompareAndSwapError>, Error>;
  fn batch(&mut self, batch: Batch) -> Result<(), Error>;
  fn transaction(&mut self, vals: &[Self::Kind]) -> TransactionResult<()>;
  fn watch_prefix<P: AsRef<[u8]>>(&self, prefix: P) -> Subscriber;
  fn set_merge_op<M: MergeOperator + 'static>(&self, op: M);
}

pub struct NodeTree {
  tree: Tree,
  factory: NodeFactory<1024, Djb2>,
}

impl TreeHandle for NodeTree {
  type Kind = NodeKind;
  type Handle = DbRef;
  type Factory = NodeFactory<1024, Djb2>;
  fn open(handle: DbRef, name: &str) -> Result<NodeTree, Error> {
    let factory = NodeFactory::<1024, Djb2>::new();
    handle.open_tree(name).map(|tree| NodeTree {tree, factory})
  }
  fn insert(&mut self, val: Self::Kind) -> Result<Option<Self::Kind>, Error> {
    let mut ser = self.factory.serializer();
    let node = self.factory.create(val);
    self.factory.serialize_val(&node, &mut ser).unwrap();
    match self.tree.insert(node.key(), self.factory.flush_bytes(ser)) {
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
  fn get(&mut self, key: Id) -> Result<Option<Self::Kind>, Error> {
    match self.tree.get(key) {
      Ok(v) => if let Some(v) = v {
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
  fn exists(&self, key: Id) -> Result<bool, Error> {
    self.tree.contains_key(key)
  }
  fn remove(&mut self, key: Id) -> Result<Option<Self::Kind>, Error> {
    match self.tree.remove(key) {
      Ok(v) => if let Some(v) = v {
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
  fn swap(&mut self, key: Id, old: Option<Self::Kind>, new: Option<Self::Kind>) -> Result<Result<(), CompareAndSwapError>, Error> {
    let old = if let Some(o) = old {
      let node = self.factory.create(o);
      let mut ser = self.factory.serializer();
      self.factory.serialize_val(&node, &mut ser).unwrap();
      Some(self.factory.flush_bytes(ser))
    } else {
      None
    };
    let new = if let Some(n) = new {
      let node = self.factory.create(n);
      let mut ser = self.factory.serializer();
      self.factory.serialize_val(&node, &mut ser).unwrap();
      Some(self.factory.flush_bytes(ser))
    } else {
      None
    };
    self.tree.compare_and_swap(key, old, new)
  }

  fn batch(&mut self, batch: Batch) -> Result<(), Error> {
    self.tree.apply_batch(batch)
  }
  fn transaction(&mut self, vals: &[Self::Kind]) -> TransactionResult<()> {
    let (keys, vals) = self.factory.serialize_vec(vals.to_vec());
    self.tree.transaction(|tx| {
      for i in 0..keys.len() {
	tx.insert(keys[i].as_slice(), vals[i].as_slice()).unwrap();
      }
      Ok(())
    })
  }
  fn watch_prefix<P: AsRef<[u8]>>(&self, prefix: P) -> Subscriber {
    self.tree.watch_prefix(prefix)
  }
  fn set_merge_op<M: MergeOperator + 'static>(&self, merge_op: M) {
    self.tree.set_merge_operator(merge_op)
  }
}

pub struct EdgeTree {
  tree: Tree,
  factory: EdgeFactory<1024>
}

impl TreeHandle for EdgeTree {
  type Kind = EdgeKind;
  type Handle = DbRef;
  type Factory = EdgeFactory<1024>;
  fn open(handle: DbRef, name: &str) -> Result<EdgeTree, Error> {
    let factory = EdgeFactory::<1024>;
    handle.open_tree(name).map(|tree| EdgeTree {tree, factory})
  }
  fn insert(&mut self, val: Self::Kind) -> Result<Option<Self::Kind>, Error> {
    let mut ser = self.factory.serializer();
    let node = self.factory.create(val);
    self.factory.serialize_val(&node, &mut ser).unwrap();
    match self.tree.insert(node.key(), self.factory.flush_bytes(ser)) {
      Ok(o) => if let Some(v) = o {
	let v = v.to_vec();
	let archived = unsafe { archived_root::<EdgeKind>(v.as_slice()) };
	let val = archived.deserialize(&mut Infallible).ok();
	Ok(val)
      } else {
	Ok(None)
      }
      Err(e) => Err(e)
    }
  }
  fn get(&mut self, key: Id) -> Result<Option<Self::Kind>, Error> {
    match self.tree.get(key) {
      Ok(v) => if let Some(v) = v {
	let v = v.to_vec();
	let archived = unsafe { archived_root::<EdgeKind>(v.as_slice()) };
	let val = archived.deserialize(&mut Infallible).ok();
	Ok(val)
      } else {
	Ok(None)
      }
      Err(e) => Err(e)
    }
  }
  fn exists(&self, key: Id) -> Result<bool, Error> {
    self.tree.contains_key(key)
  }
  fn remove(&mut self, key: Id) -> Result<Option<Self::Kind>, Error> {
    match self.tree.remove(key) {
      Ok(v) => if let Some(v) = v {
	let v = v.to_vec();
	let archived = unsafe { archived_root::<EdgeKind>(v.as_slice()) };
	let val = archived.deserialize(&mut Infallible).ok();
	Ok(val)
      } else {
	Ok(None)
      }
      Err(e) => Err(e)
    }
  }
  fn swap(&mut self, key: Id, old: Option<Self::Kind>, new: Option<Self::Kind>) -> Result<Result<(), CompareAndSwapError>, Error> {
    let old = if let Some(o) = old {
      let node = self.factory.create(o);
      let mut ser = self.factory.serializer();
      self.factory.serialize_val(&node, &mut ser).unwrap();
      Some(self.factory.flush_bytes(ser))
    } else {
      None
    };
    let new = if let Some(n) = new {
      let node = self.factory.create(n);
      let mut ser = self.factory.serializer();
      self.factory.serialize_val(&node, &mut ser).unwrap();
      Some(self.factory.flush_bytes(ser))
    } else {
      None
    };
    self.tree.compare_and_swap(key, old, new)
  }

  fn batch(&mut self, batch: Batch) -> Result<(), Error> {
    self.tree.apply_batch(batch)
  }
  fn transaction(&mut self, vals: &[Self::Kind]) -> TransactionResult<()> {
    let (keys, vals) = self.factory.serialize_vec(vals.to_vec());
    self.tree.transaction(|tx| {
      for i in 0..keys.len() {
	tx.insert(keys[i].as_slice(), vals[i].as_slice()).unwrap();
      }
      Ok(())
    })
  }
  fn watch_prefix<P: AsRef<[u8]>>(&self, prefix: P) -> Subscriber {
    self.tree.watch_prefix(prefix)
  }
  fn set_merge_op<M: MergeOperator + 'static>(&self, merge_op: M) {
    self.tree.set_merge_operator(merge_op)
  }
}
