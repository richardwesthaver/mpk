//! MPK_DB -- FACTORY
use std::hash::Hasher;
use rkyv::ser::{Serializer, serializers::AllocSerializer};
use rkyv::Archive;
use mpk_hash::Djb2;
use crate::{NodeSerializer, Node, NodeKind, NodeVec,
	    EdgeSerializer, Edge, EdgeKind, EdgeVec};

#[derive(Debug, Clone, Copy)]
pub enum FactoryError {
  SerializationFailed,
}

pub trait Factory {
  type Ser: Serializer;
  type Kind;
  type Val: Archive;
  fn serializer(&self) -> Self::Ser;
  fn create(&mut self, kind: Self::Kind) -> Self::Val;
  fn serialize_val(&mut self, val: &Self::Val, ser: &mut Self::Ser) -> Result<usize, FactoryError>;
  fn serialize_vec(&mut self, kinds: Vec<Self::Kind>) -> (Vec<[u8; 8]>, Vec<Vec<u8>>);
  fn flush_bytes(&self, ser: Self::Ser) -> Vec<u8>;
}

#[derive(Debug)]
pub struct NodeFactory<const N: usize, H: Hasher + Default> {
  hasher: H,
}

impl<const N: usize , H: Hasher + Default> NodeFactory<N,H> {
  pub fn new() -> NodeFactory<N, Djb2> {
    NodeFactory {
      hasher: Djb2::default(),
    }
  }
}

impl<const N: usize,H: Hasher + Default> Factory for NodeFactory<N,H> {
  type Ser = NodeSerializer<AllocSerializer<N>>;
  type Kind = NodeKind;
  type Val = Node;
  fn serializer(&self) -> NodeSerializer<AllocSerializer<N>> {
    NodeSerializer::<AllocSerializer<N>>::default()
  }
  fn create(&mut self, kind: Self::Kind) -> Self::Val {
    Node::new(kind, &mut self.hasher)
  }
  fn serialize_val(&mut self, val: &Self::Val, ser: &mut Self::Ser) -> Result<usize, FactoryError> {
    match val.serialize(ser) {
      Ok(n) => Ok(n),
      Err(_) => Err(FactoryError::SerializationFailed)
    }
  }
  fn serialize_vec(&mut self, kinds: NodeVec) -> (Vec<[u8; 8]>, Vec<Vec<u8>>) {
    let mut keys = Vec::<[u8; 8]>::new();
    let mut vals = Vec::new();
    for n in kinds {
      let mut ser = self.serializer();
      let n = self.create(n);
      self.serialize_val(&n, &mut ser).unwrap();
      let key = n.key();
      let val = self.flush_bytes(ser);
      keys.push(key);
      vals.push(val);
    }
    (keys, vals)
  }
  fn flush_bytes(&self, ser: Self::Ser) -> Vec<u8> {
    ser.into_inner().into_serializer().into_inner().to_vec()
  }
}

#[derive(Debug)]
pub struct EdgeFactory<const N: usize>;

impl<const N: usize> Factory for EdgeFactory<N> {
  type Ser = EdgeSerializer<AllocSerializer<N>>;
  type Kind = EdgeKind;
  type Val = Edge;
  fn serializer(&self) -> EdgeSerializer<AllocSerializer<N>> {
    EdgeSerializer::<AllocSerializer<N>>::default()
  }
  fn create(&mut self, kind: Self::Kind) -> Self::Val {
    Edge::new(kind)
  }
  fn serialize_val(&mut self, val: &Self::Val, ser: &mut Self::Ser) -> Result<usize, FactoryError> {
    match val.serialize(ser) {
      Ok(n) => Ok(n),
      Err(_) => Err(FactoryError::SerializationFailed)
    }
  }
  fn serialize_vec(&mut self, kinds: EdgeVec) -> (Vec<[u8; 8]>, Vec<Vec<u8>>) {
    let mut keys = Vec::<[u8; 8]>::new();
    let mut vals = Vec::new();
    for n in kinds {
      let mut ser = self.serializer();
      let n = self.create(n);
      self.serialize_val(&n, &mut ser).unwrap();
      let key = n.key();
      let val = self.flush_bytes(ser);
      keys.push(key);
      vals.push(val);
    }
    (keys, vals)
  }
  fn flush_bytes(&self, ser: Self::Ser) -> Vec<u8> {
    ser.into_inner().into_serializer().into_inner().to_vec()
  }
}
