//! MPK_DB -- FACTORY
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

use crate::{
  Edge, EdgeKind, EdgeProps, Key, Meta, MetaKind, Node, NodeKind, NodeProps, Prop,
  PropVec, Val,
};

#[derive(Debug, Clone, Copy)]
pub enum FactoryError {
  SerializationFailed,
  DeserializationFailed,
}

pub trait Factory {
  type Kind;
  type Ty: Key + Val + Serialize;
  fn serialize_key(&self, ty: &Self::Ty) -> Result<Vec<u8>, FactoryError>
  where
    <<Self as Factory>::Ty as Key>::Key: Serialize,
  {
    match serialize(&ty.key()) {
      Ok(n) => Ok(n),
      Err(_) => Err(FactoryError::SerializationFailed),
    }
  }
  fn serialize_val(&self, ty: &Self::Ty) -> Result<Vec<u8>, FactoryError>
  where
    <<Self as Factory>::Ty as Val>::Val: Serialize,
  {
    match serialize(&ty.val()) {
      Ok(n) => Ok(n),
      Err(_) => Err(FactoryError::SerializationFailed),
    }
  }
  fn serialize(&self, ty: &Self::Ty) -> Result<(Vec<u8>, Vec<u8>), FactoryError>
  where
    <<Self as Factory>::Ty as Key>::Key: Serialize,
    <<Self as Factory>::Ty as Val>::Val: Serialize,
  {
    let key = self.serialize_key(ty)?;
    let val = self.serialize_val(ty)?;
    Ok((key, val))
  }
  fn serialize_vec(
    &self,
    vec: Vec<Self::Ty>,
  ) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), FactoryError>
  where
    <<Self as Factory>::Ty as Key>::Key: Serialize,
    <<Self as Factory>::Ty as Val>::Val: Serialize,
  {
    let mut keys = Vec::new();
    let mut vals = Vec::new();
    for n in vec {
      let key = self.serialize_key(&n)?;
      let val = self.serialize_val(&n)?;
      keys.push(key);
      vals.push(val);
    }
    Ok((keys, vals))
  }
  fn deserialize_key<'de, K: Key + Deserialize<'de>>(
    &self,
    bytes: &'de [u8],
  ) -> Result<K, FactoryError> {
    deserialize(bytes).map_err(|_| FactoryError::DeserializationFailed)
  }
  fn deserialize_val<'de, V: Val + Deserialize<'de>>(
    &self,
    bytes: &'de [u8],
  ) -> Result<V, FactoryError> {
    deserialize(bytes).map_err(|_| FactoryError::DeserializationFailed)
  }
  fn deserialize<'de, K: Key + Deserialize<'de>, V: Val + Deserialize<'de>>(
    &self,
    key: &'de [u8],
    val: &'de [u8],
  ) -> Result<(K, V), FactoryError> {
    let key = self.deserialize_key(key)?;
    let val = self.deserialize_val(val)?;
    Ok((key, val))
  }
}

#[derive(Debug)]
pub struct NodeFactory;

impl Factory for NodeFactory {
  type Kind = NodeKind;
  type Ty = Node;
}

#[derive(Debug)]
pub struct EdgeFactory;

impl Factory for EdgeFactory {
  type Kind = EdgeKind;
  type Ty = Edge;
}

#[derive(Debug)]
pub struct MetaFactory;

impl Factory for MetaFactory {
  type Kind = MetaKind;
  type Ty = Meta;
}

#[derive(Debug)]
pub struct EdgePropFactory;

impl Factory for EdgePropFactory {
  type Kind = PropVec;
  type Ty = EdgeProps;
}

#[derive(Debug)]
pub struct NodePropFactory;

impl Factory for NodePropFactory {
  type Kind = PropVec;
  type Ty = NodeProps;
}
