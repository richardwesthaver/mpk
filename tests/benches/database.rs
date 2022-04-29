#![feature(test)]
#![cfg(test)]
extern crate test;
use test::Bencher;

use mpk_db::Factory;

use benches::{db_init, gen_keys, gen_vals,
	      init_edge_factory, init_node_factory,
	      serialize_node, serialize_nodevec,
	      serialize_edge, serialize_edgevec};

#[bench]
fn insert_8x64_1k(b: &mut Bencher) {
  let db = db_init();
  let keys = gen_keys(1_000);
  let vals = gen_vals(1_000);
  b.iter(|| {
    for i in 0..keys.len() {
      db.insert(&keys[i], vals[i].as_slice()).unwrap();
    }
  })
}

#[bench]
fn insert_8x64_10k(b: &mut Bencher) {
  let db = db_init();
  let keys = gen_keys(10_000);
  let vals = gen_vals(10_000);
  b.iter(|| {
    for i in 0..keys.len() {
      db.insert(&keys[i], vals[i].as_slice()).unwrap();
    }
  })
}

#[bench]
fn insert_8x64_100k(b: &mut Bencher) {
  let db = db_init();
  let keys = gen_keys(100_000);
  let vals = gen_vals(100_000);
  b.iter(|| {
    for i in 0..keys.len() {
      db.insert(&keys[i], vals[i].as_slice()).unwrap();
    }
  })
}

#[bench]
fn insert_node_1k(b: &mut Bencher) {
  let (mut factory, _) = init_node_factory::<1024>();
  let db = db_init();
  let (keys, vals) = serialize_nodevec(&mut factory, 1_000);
  b.iter(|| {
    for i in 0..1_000 {
      db.insert(&keys[i], vals[i].as_slice()).unwrap();
    }
  })
}

#[bench]
fn insert_node_10k(b: &mut Bencher) {
  let (mut factory, _) = init_node_factory::<1024>();
  let db = db_init();
  let (keys, vals) = serialize_nodevec(&mut factory, 10_000);
  b.iter(|| {
    for i in 0..10_000 {
      db.insert(&keys[i], vals[i].as_slice()).unwrap();
    }
  })
}

#[bench]
fn insert_node_100k(b: &mut Bencher) {
  let (mut factory, _) = init_node_factory::<1024>();
  let db = db_init();
  let (keys, vals) = serialize_nodevec(&mut factory, 100_000);
  b.iter(|| {
    for i in 0..100_000 {
      db.insert(&keys[i], vals[i].as_slice()).unwrap();
    }
  })
}

#[bench]
fn insert_edge_1k(b: &mut Bencher) {
  let (mut factory, _) = init_edge_factory::<1024>();
  let db = db_init();
  let (keys, vals) = serialize_edgevec(&mut factory, 1_000);
  b.iter(|| {
    for i in 0..1_000 {
      db.insert(&keys[i], vals[i].as_slice()).unwrap();
    }
  })
}

#[bench]
fn insert_edge_10k(b: &mut Bencher) {
  let (mut factory, _) = init_edge_factory::<1024>();
  let db = db_init();
  let (keys, vals) = serialize_edgevec(&mut factory, 10_000);
  b.iter(|| {
    for i in 0..10_000 {
      db.insert(&keys[i], vals[i].as_slice()).unwrap();
    }
  })
}

#[bench]
fn insert_edge_100k(b: &mut Bencher) {
  let (mut factory, _) = init_edge_factory::<1024>();
  let db = db_init();
  let (keys, vals) = serialize_edgevec(&mut factory, 100_000);
  b.iter(|| {
    for i in 0..100_000 {
      db.insert(&keys[i], vals[i].as_slice()).unwrap();
    }
  })
}
