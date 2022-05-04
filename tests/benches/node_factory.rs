#![feature(test)]
#![cfg(test)]
extern crate test;
use test::Bencher;

use benches::{init_node_factory, serialize_node, serialize_nodevec};

#[bench]
fn node_serialize_val(b: &mut Bencher) {
  let factory = init_node_factory();
  b.iter(|| serialize_node(&factory))
}

#[bench]
fn node_serialize_1k(b: &mut Bencher) {
  let factory = init_node_factory();
  b.iter(|| serialize_nodevec(&factory, 1_000))
}

#[bench]
fn node_serialize_10k(b: &mut Bencher) {
  let factory = init_node_factory();
  b.iter(|| serialize_nodevec(&factory, 10_000))
}

#[bench]
fn node_serialize_100k(b: &mut Bencher) {
  let factory = init_node_factory();
  b.iter(|| serialize_nodevec(&factory, 100_000))
}
