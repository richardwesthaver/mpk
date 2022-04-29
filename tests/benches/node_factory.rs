#![feature(test)]
#![cfg(test)]
extern crate test;
use test::Bencher;

use benches::{init_node_factory, serialize_node, serialize_nodevec};

#[bench]
fn node_serialize_val(b: &mut Bencher) {
  let (mut factory, mut ser) = init_node_factory::<1024>();
  b.iter(|| {
    serialize_node(&mut factory, &mut ser)
  })
}

#[bench]
fn node_serialize_1k(b: &mut Bencher) {
  let (mut factory, _) = init_node_factory::<1024>();
  b.iter(|| {
    serialize_nodevec(&mut factory, 1_000)
  })
}

#[bench]
fn node_serialize_10k(b: &mut Bencher) {
  let (mut factory, _) = init_node_factory::<1024>();
  b.iter(|| {
    serialize_nodevec(&mut factory, 10_000)
  })
}

#[bench]
fn node_serialize_100k(b: &mut Bencher) {
  let (mut factory, _) = init_node_factory::<1024>();
  b.iter(|| {
    serialize_nodevec(&mut factory, 100_000)
  })
}
