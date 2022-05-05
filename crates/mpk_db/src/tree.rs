//! MPK_DB -- TREE
use crate::{
  DbRef, Edge, EdgeFactory, EdgeKey, EdgeProp, EdgePropFactory, Factory, Id, IdVec,
  Key, Meta, MetaFactory, MetaKind, Node, NodeFactory, NodeKind, NodeProp,
  NodePropFactory, Prop, Val,
};
use bincode::{deserialize, serialize};
use serde::Serialize;
use sled::{
  transaction::TransactionResult, Batch, CompareAndSwapError, Error, MergeOperator,
  Subscriber, Tree,
};

use std::ops::Deref;

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

pub trait TreeHandle<'de>: Sized {
  type Ty: Key + Val;
  type Key: Key;
  type Val: Val;
  type Factory: Factory;
  fn open(handle: DbRef, name: &str) -> Result<Self, Error>;
  fn insert(&mut self, val: &Self::Ty) -> Result<Option<Self::Val>, Error>;
  fn get<K: Serialize + Into<Self::Key>>(
    &mut self,
    key: &K,
  ) -> Result<Option<Self::Val>, Error>;
  fn exists<K: Serialize + Into<Self::Key>>(&self, key: &K) -> Result<bool, Error>;
  fn remove<K: Serialize + Into<Self::Key>>(
    &mut self,
    key: &K,
  ) -> Result<Option<Self::Val>, Error>;
  fn swap(
    &mut self,
    key: Self::Key,
    old: Option<Self::Val>,
    new: Option<Self::Val>,
  ) -> Result<Result<(), CompareAndSwapError>, Error>;
  fn batch(&mut self, batch: Batch) -> Result<(), Error>;
  fn transaction(&mut self, vals: &[Self::Ty]) -> TransactionResult<()>;
  fn watch_prefix<P: AsRef<[u8]>>(&self, prefix: P) -> Subscriber;
  fn set_merge_op<M: MergeOperator + 'static>(&self, op: M);
}

macro_rules! impl_deref_tree {
  ($($i:ident),+) => {
    $(impl Deref for $i {
      type Target = Tree;
      fn deref(&self) -> &Self::Target {
	&self.tree
      }
    })*
  }
}

macro_rules! impl_tree {
  ($i:ident, $k:ident, $v:ident, $t:ident, $f:ident) => {
    impl<'de> TreeHandle<'de> for $i {
      type Key = $k;
      type Val = $v;
      type Ty = $t;
      type Factory = $f;
      fn open(handle: DbRef, name: &str) -> Result<$i, Error> {
        let factory = $f;
        handle.open_tree(name).map(|tree| $i { tree, factory })
      }
      fn insert(&mut self, ty: &$t) -> Result<Option<$v>, Error> {
        let key = self.factory.serialize_key(&ty).unwrap();
        let val = self.factory.serialize_val(&ty).unwrap();
        match self.tree.insert(key, val) {
          Ok(o) => {
            if let Some(v) = o {
              Ok(deserialize(&v).unwrap())
            } else {
              Ok(None)
            }
          }
          Err(e) => Err(e),
        }
      }
      fn get<K: Serialize + Into<$k>>(&mut self, key: &K) -> Result<Option<$v>, Error> {
        let key: Vec<u8> = serialize(key).unwrap();
        match self.tree.get(&key) {
          Ok(v) => {
            if let Some(v) = v {
              let val = deserialize(&v).unwrap();
              Ok(Some(val))
            } else {
              Ok(None)
            }
          }
          Err(e) => Err(e),
        }
      }
      fn exists<K: Serialize + Into<$k>>(&self, key: &K) -> Result<bool, Error> {
        let key: Vec<u8> = serialize(key).unwrap();
        self.tree.contains_key(key)
      }
      fn remove<K: Serialize + Into<$k>>(
        &mut self,
        key: &K,
      ) -> Result<Option<$v>, Error> {
        let key: Vec<u8> = serialize(key).unwrap();
        match self.tree.remove(key) {
          Ok(v) => {
            if let Some(v) = v {
              let val = deserialize(&v).ok();
              Ok(val)
            } else {
              Ok(None)
            }
          }
          Err(e) => Err(e),
        }
      }
      fn swap(
        &mut self,
        key: Self::Key,
        old: Option<$v>,
        new: Option<$v>,
      ) -> Result<Result<(), CompareAndSwapError>, Error> {
        let key: Vec<u8> = key.into();
        let old = if let Some(o) = old {
          serialize(&o).ok()
        } else {
          None
        };
        let new = if let Some(n) = new {
          serialize(&n).ok()
        } else {
          None
        };
        self.tree.compare_and_swap(key, old, new)
      }
      fn batch(&mut self, batch: Batch) -> Result<(), Error> {
        self.tree.apply_batch(batch)
      }
      fn transaction(&mut self, vals: &[$t]) -> TransactionResult<()> {
        let (keys, vals) = self.factory.serialize_vec(vals.to_vec()).unwrap();
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
  };
}

pub struct NodeTree {
  pub tree: Tree,
  pub factory: NodeFactory,
}

impl_tree!(NodeTree, Id, NodeKind, Node, NodeFactory);

pub struct EdgeTree {
  pub tree: Tree,
  pub factory: EdgeFactory,
}

impl_tree!(EdgeTree, EdgeKey, u128, Edge, EdgeFactory);

pub struct MetaTree {
  pub tree: Tree,
  pub factory: MetaFactory,
}

impl_tree!(MetaTree, MetaKind, IdVec, Meta, MetaFactory);

pub struct NodePropTree {
  pub tree: Tree,
  pub factory: NodePropFactory,
}

impl_tree!(NodePropTree, Id, Prop, NodeProp, NodePropFactory);

pub struct EdgePropTree {
  pub tree: Tree,
  pub factory: EdgePropFactory,
}

impl_tree!(EdgePropTree, EdgeKey, Prop, EdgeProp, EdgePropFactory);

impl_deref_tree!(EdgeTree, NodeTree, MetaTree, NodePropTree, EdgePropTree);