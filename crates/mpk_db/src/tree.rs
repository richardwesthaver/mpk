//! MPK_DB -- TREE
use std::ops::Deref;

// use std::ops::RangeBounds;
use bincode::{deserialize, serialize};
use serde::Serialize;
use sled::{
  transaction::TransactionResult, Batch, CompareAndSwapError, MergeOperator,
  Subscriber, Tree,
};

use crate::{
  DbRef, Edge, EdgeFactory, EdgeKey, EdgePropFactory, EdgeProps, Error, Factory, Id,
  IdVec, Key, Meta, MetaFactory, MetaKind, Node, NodeFactory, NodeKind,
  NodePropFactory, NodeProps, Prop, PropVec, Val,
};

pub const TREE_NAMES: [&str; 11] = [
  "media",
  "media_props",
  "path",
  "artist",
  "album",
  "genre",
  "coll",
  "playlist",
  "source",
  "edge",
  "edge_props",
];

pub trait TreeHandle: Sized {
  type Ty: Key + Val;
  type Key: Key;
  type Val: Val;
  type Factory: Factory;
  fn open(handle: DbRef, name: &str) -> Result<Self, Error>;
  fn insert(&mut self, val: &Self::Ty) -> Result<Option<Self::Val>, Error>;
  fn get<K: Serialize + Into<Self::Key>>(
    &self,
    key: &K,
  ) -> Result<Option<Self::Val>, Error>;
  fn get_lt<K: Serialize + Into<Self::Key>>(
    &self,
    key: &K,
  ) -> Result<Option<(Self::Key, Self::Val)>, Error>;
  fn get_gt<K: Serialize + Into<Self::Key>>(
    &self,
    key: &K,
  ) -> Result<Option<(Self::Key, Self::Val)>, Error>;
  fn exists<K: Serialize + Into<Self::Key>>(&self, key: &K) -> Result<bool, Error>;
  fn remove<K: Serialize + Into<Self::Key>>(
    &mut self,
    key: &K,
  ) -> Result<Option<Self::Val>, Error>;
  fn merge(&self, key: Self::Key, val: Self::Val) -> Result<Option<Self::Val>, Error>;
  fn swap(
    &mut self,
    key: Self::Key,
    old: Option<Self::Val>,
    new: Option<Self::Val>,
  ) -> Result<Result<(), CompareAndSwapError>, Error>;
  //  fn range<R: RangeBounds<Self::Key>>(&self, range: R) -> Iter;
  //  fn iter(&self) -> Iter;
  //  fn scan_prefix<P>(&self, prefix: P) -> Iter
  fn batch(&mut self, batch: Batch) -> Result<(), Error>;
  fn transaction(&mut self, vals: &[Self::Ty]) -> TransactionResult<()>;
  fn watch_prefix<P: AsRef<[u8]>>(&self, prefix: P) -> Subscriber;
  fn set_merge_op<M: MergeOperator + 'static>(&self, op: M);
}

macro_rules! make_tree {
  ($(#[$m:meta])*
  $v:vis $i:ident, $f:ident) => {
    $(#[$m])*
    $v struct $i {
      $v tree: Tree,
      $v factory: $f,
    }
  }
}

macro_rules! impl_deref_tree {
  ($i:ident) => {
    impl Deref for $i {
      type Target = Tree;
      fn deref(&self) -> &Self::Target {
        &self.tree
      }
    }
  };
}

macro_rules! impl_tree_handle {
  ($i:ident, $k:ident, $v:ident, $t:ident, $f:ident) => {
    impl TreeHandle for $i {
      type Key = $k;
      type Val = $v;
      type Ty = $t;
      type Factory = $f;
      fn open(handle: DbRef, name: &str) -> Result<$i, Error> {
        let factory = $f;
        handle
          .open_tree(name)
          .map_err(|e| e.into())
          .map(|tree| $i { tree, factory })
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
          Err(e) => Err(e.into()),
        }
      }
      fn get<K: Serialize + Into<$k>>(&self, key: &K) -> Result<Option<$v>, Error> {
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
          Err(e) => Err(e.into()),
        }
      }
      fn get_lt<K: Serialize + Into<$k>>(
        &self,
        key: &K,
      ) -> Result<Option<($k, $v)>, Error> {
        let key: Vec<u8> = serialize(key).unwrap();
        match self.tree.get_lt(&key) {
          Ok(v) => {
            if let Some((k, v)) = v {
              let key = deserialize(&k).unwrap();
              let val = deserialize(&v).unwrap();
              Ok(Some((key, val)))
            } else {
              Ok(None)
            }
          }
          Err(e) => Err(e.into()),
        }
      }
      fn get_gt<K: Serialize + Into<$k>>(
        &self,
        key: &K,
      ) -> Result<Option<($k, $v)>, Error> {
        let key: Vec<u8> = serialize(key).unwrap();
        match self.tree.get_gt(&key) {
          Ok(v) => {
            if let Some((k, v)) = v {
              let key = deserialize(&k).unwrap();
              let val = deserialize(&v).unwrap();
              Ok(Some((key, val)))
            } else {
              Ok(None)
            }
          }
          Err(e) => Err(e.into()),
        }
      }
      fn exists<K: Serialize + Into<$k>>(&self, key: &K) -> Result<bool, Error> {
        let key: Vec<u8> = serialize(key).unwrap();
        self.tree.contains_key(key).map_err(|e| e.into())
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
          Err(e) => Err(e.into()),
        }
      }
      fn merge(&self, key: $k, val: $v) -> Result<Option<Self::Val>, Error> {
        let key: Vec<u8> = key.into();
        let val = serialize(&val).unwrap();
        match self.tree.merge(key, val) {
          Ok(v) => {
            if let Some(v) = v {
              let val = deserialize(&v).ok();
              Ok(val)
            } else {
              Ok(None)
            }
          }
          Err(e) => Err(e.into()),
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
        self
          .tree
          .compare_and_swap(key, old, new)
          .map_err(|e| e.into())
      }
      fn batch(&mut self, batch: Batch) -> Result<(), Error> {
        self.tree.apply_batch(batch).map_err(|e| e.into())
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

macro_rules! trees {
  ($(($i:ident, $k:ident, $v:ident, $t:ident, $f:ident)),+) => {
    $(
      make_tree!(#[derive(Debug)] pub $i, $f);
      impl_deref_tree!($i);
      impl_tree_handle!($i, $k, $v, $t, $f);
    )*
  }
}

trees!(
  (NodeTree, Id, NodeKind, Node, NodeFactory),
  (EdgeTree, EdgeKey, u128, Edge, EdgeFactory),
  (MetaTree, MetaKind, IdVec, Meta, MetaFactory),
  (NodePropTree, Id, PropVec, NodeProps, NodePropFactory),
  (EdgePropTree, EdgeKey, PropVec, EdgeProps, EdgePropFactory)
);

pub fn meta_merge_op(_key: &[u8], old: Option<&[u8]>, new: &[u8]) -> Option<Vec<u8>> {
  let mut ret = old
    .map(|v| deserialize::<IdVec>(v).unwrap())
    .unwrap_or_else(|| vec![]);
  ret.push(deserialize::<Id>(new).unwrap());
  Some(serialize::<IdVec>(&ret).unwrap())
}

pub fn prop_merge_op(_key: &[u8], old: Option<&[u8]>, new: &[u8]) -> Option<Vec<u8>> {
  let mut ret = old
    .map(|v| deserialize::<PropVec>(v).unwrap())
    .unwrap_or_else(|| vec![]);
  ret.push(deserialize::<Prop>(new).unwrap());
  Some(serialize::<PropVec>(&ret).unwrap())
}
