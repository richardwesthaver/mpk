#![feature(test)]
#![cfg(test)]
extern crate test;
use benches::{init_edge_factory, serialize_edge, serialize_edgevec};
use test::Bencher;

#[bench]
fn edge_serialize_val(b: &mut Bencher) {
  let factory = init_edge_factory();
  b.iter(|| serialize_edge(&factory))
}

#[bench]
fn edge_serialize_1k(b: &mut Bencher) {
  let factory = init_edge_factory();
  b.iter(|| serialize_edgevec(&factory, 1_000))
}

#[bench]
fn edge_serialize_10k(b: &mut Bencher) {
  let factory = init_edge_factory();
  b.iter(|| serialize_edgevec(&factory, 10_000))
}

#[bench]
fn edge_serialize_100k(b: &mut Bencher) {
  let factory = init_edge_factory();
  b.iter(|| serialize_edgevec(&factory, 100_000))
}
