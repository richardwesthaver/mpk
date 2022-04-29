#![feature(test)]
#![cfg(test)]
extern crate test;
use test::Bencher;

use benches::{init_edge_factory, serialize_edge, serialize_edgevec};

#[bench]
fn edge_serialize_val(b: &mut Bencher) {
  let (mut factory, mut ser) = init_edge_factory::<1024>();
  b.iter(|| {
    serialize_edge(&mut factory, &mut ser)
  })
}

#[bench]
fn edge_serialize_1k(b: &mut Bencher) {
  let (mut factory, _) = init_edge_factory::<1024>();
  b.iter(|| {
    serialize_edgevec(&mut factory, 1_000)
  })
}

#[bench]
fn edge_serialize_10k(b: &mut Bencher) {
  let (mut factory, _) = init_edge_factory::<1024>();
  b.iter(|| {
    serialize_edgevec(&mut factory, 10_000)
  })
}

#[bench]
fn edge_serialize_100k(b: &mut Bencher) {
  let (mut factory, _) = init_edge_factory::<1024>();
  b.iter(|| {
    serialize_edgevec(&mut factory, 100_000)
  })
}
