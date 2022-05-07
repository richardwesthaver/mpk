#![feature(test)]
use std::hash::Hasher;

use mpk_db::{
  Db, Edge, EdgeFactory, EdgeKey, EdgeKind, EdgeVec, Factory, Node, NodeFactory,
  NodeKind, NodeVec,
};
use mpk_hash::{B3Hasher, Djb2, FxHasher};
use rand::prelude::*;

pub fn db_init() -> Db {
  Db::open(None::<String>).unwrap()
  //  Db::open(Some("test.db")).unwrap()
}

pub fn gen_keys(n: usize) -> Vec<[u8; 8]> {
  let mut rng = rand::thread_rng();
  let mut keys: Vec<[u8; 8]> = Vec::new();
  for _i in 0..n {
    let mut key: [u8; 8] = [0; 8];
    rng.fill_bytes(&mut key);
    keys.push(key)
  }
  keys
}

pub fn gen_vals(n: usize) -> Vec<[u8; 64]> {
  let mut rng = rand::thread_rng();
  let mut vals: Vec<[u8; 64]> = Vec::new();
  for _i in 0..n {
    let mut val: [u8; 64] = [0; 64];
    rng.fill_bytes(&mut val);
    vals.push(val)
  }
  vals
}

pub fn init_node_factory() -> NodeFactory {
  NodeFactory
}

pub fn init_edge_factory() -> EdgeFactory {
  EdgeFactory
}

pub fn serialize_node(factory: &NodeFactory) {
  let node = Node::new(NodeKind::Track);
  factory.serialize_val(&node).unwrap();
}

pub fn serialize_nodevec(
  factory: &NodeFactory,
  count: usize,
) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
  let kind1 = Node::new(NodeKind::Track);
  let kind2 = Node::new(NodeKind::Sample);
  let kind3 = Node::new(NodeKind::Midi);
  let kind4 = Node::new(NodeKind::Patch);
  let nodes: NodeVec = vec![kind1, kind2, kind3, kind4]
    .into_iter()
    .flat_map(|n| std::iter::repeat(n).take(count / 4))
    .collect();
  factory.serialize_vec(nodes).unwrap()
}

pub fn serialize_edge(factory: &EdgeFactory) {
  let node = Edge::new(EdgeKey::new(EdgeKind::Next, 100, 200));
  factory.serialize_val(&node).unwrap();
}

pub fn serialize_edgevec(
  factory: &EdgeFactory,
  count: usize,
) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
  let kind1 = Edge::new(EdgeKey::new(EdgeKind::Next, 100, 200));
  let kind2 = Edge::new(EdgeKey::new(EdgeKind::Similar, 200, 300));
  let kind3 = Edge::new(EdgeKey::new(EdgeKind::Compliment, 300, 400));
  let kind4 = Edge::new(EdgeKey::new(EdgeKind::Compose, 400, 500));
  let edges: EdgeVec = vec![kind1, kind2, kind3, kind4]
    .into_iter()
    .flat_map(|n| std::iter::repeat(n).take(count / 4))
    .collect();
  factory.serialize_vec(edges).unwrap()
}

pub fn b3_range(n: usize) {
  let input = gen_vals(n);
  let mut hasher = B3Hasher::default();
  for i in input {
    hasher.update(&i);
    hasher.finalize();
  }
}

pub fn djb2_range(n: usize) {
  let input = gen_vals(n);
  let mut hasher = Djb2::default();
  for i in input {
    hasher.write(&i);
    hasher.finish();
  }
}

pub fn fx_range(n: usize) {
  let input = gen_vals(n);
  let mut hasher = FxHasher::default();
  for i in input {
    hasher.write(&i);
    hasher.finish();
  }
}
