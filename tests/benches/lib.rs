#![feature(test)]
use mpk_db::{Db, Factory,
	     NodeFactory, NodeKind, NodeVec,
	     EdgeFactory, EdgeKind, EdgeVec};
use mpk_hash::Djb2;
use rand::prelude::*;

pub fn db_init() -> Db {
  Db::open(None).unwrap()
}

pub fn gen_keys(n: usize) -> Vec<[u8;8]> {
  let mut rng = rand::thread_rng();
  let mut keys: Vec<[u8;8]> = Vec::new();
  for _i in 0..n {
    let mut key: [u8;8] = [0;8];
    rng.fill_bytes(&mut key);
    keys.push(key)
  }
  keys
}

pub fn gen_vals(n: usize) -> Vec<[u8;64]> {
  let mut rng = rand::thread_rng();
  let mut vals: Vec<[u8;64]> = Vec::new();
  for _i in 0..n {
    let mut val: [u8;64] = [0;64];
    rng.fill_bytes(&mut val);
    vals.push(val)
  }
  vals
}

pub fn init_node_factory<const N: usize>() -> (NodeFactory<N, Djb2>, <NodeFactory<N, Djb2> as Factory>::Ser) {
  let factory = NodeFactory::<N, Djb2>::new();
  let serializer = factory.serializer();
  (factory, serializer)
}

pub fn init_edge_factory<const N: usize>() -> (EdgeFactory<N>, <EdgeFactory<N> as Factory>::Ser) {
  let factory = EdgeFactory::<N>;
  let serializer = factory.serializer();
  (factory, serializer)
}

pub fn serialize_node<F: Factory<Kind = NodeKind>>(factory: &mut F, mut ser: &mut F::Ser) {
  let node = factory.create(NodeKind::new("track:../../tests/ch1.wav").unwrap());
  factory.serialize_val(&node, &mut ser).unwrap();
}

pub fn serialize_nodevec<F: Factory<Kind = NodeKind>>(factory: &mut F, count: usize) -> (Vec<[u8;8]>, Vec<Vec<u8>>) {
  let kind1 = NodeKind::new("track:/Users/ellis/dev/mpk/tests/ch1.wav").unwrap();
  let kind2 = NodeKind::new("sample:/Users/ellis/dev/mpk/tests/ch2.wav").unwrap();
  let kind3 = NodeKind::new("midi:/Users/ellis/dev/mpk/Cargo.toml").unwrap();
  let kind4 = NodeKind::new("patch:/Users/ellis/dev/mpk/config.nims").unwrap();
  let nodes: NodeVec = vec![kind1, kind2, kind3, kind4]
    .into_iter().flat_map(|n| std::iter::repeat(n).take(count/4)).collect();
  factory.serialize_vec(nodes)
}

pub fn serialize_edge<F: Factory<Kind = EdgeKind>>(factory: &mut F, mut ser: &mut F::Ser) {
  let node = factory.create(EdgeKind::Next(100, 200));
  factory.serialize_val(&node, &mut ser).unwrap();
}

pub fn serialize_edgevec<F: Factory<Kind = EdgeKind>>(factory: &mut F, count: usize) -> (Vec<[u8;8]>, Vec<Vec<u8>>) {
  let kind1 = EdgeKind::Next(100, 200);
  let kind2 = EdgeKind::Similar(200, 0.5, 300);
  let kind3 = EdgeKind::Compliment(300, 400);
  let kind4 = EdgeKind::Compose(400, 500);
  let edges: EdgeVec = vec![kind1, kind2, kind3, kind4]
    .into_iter().flat_map(|n| std::iter::repeat(n).take(count/4)).collect();
  factory.serialize_vec(edges)
}
