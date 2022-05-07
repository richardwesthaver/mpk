#![feature(test)]
#![cfg(test)]
extern crate test;
use benches::{b3_range, djb2_range, fx_range};
use test::Bencher;

#[bench]
fn b3_hash_1k(b: &mut Bencher) {
  b.iter(|| b3_range(1_000))
}

#[bench]
fn b3_hash_10k(b: &mut Bencher) {
  b.iter(|| b3_range(10_000))
}

#[bench]
fn b3_hash_100k(b: &mut Bencher) {
  b.iter(|| b3_range(100_000))
}

#[bench]
fn djb2_hash_1k(b: &mut Bencher) {
  b.iter(|| djb2_range(1_000))
}

#[bench]
fn djb2_hash_10k(b: &mut Bencher) {
  b.iter(|| djb2_range(10_000))
}

#[bench]
fn djb2_hash_100k(b: &mut Bencher) {
  b.iter(|| djb2_range(100_000))
}

#[bench]
fn fx_hash_1k(b: &mut Bencher) {
  b.iter(|| fx_range(1_000))
}

#[bench]
fn fx_hash_10k(b: &mut Bencher) {
  b.iter(|| fx_range(10_000))
}

#[bench]
fn fx_hash_100k(b: &mut Bencher) {
  b.iter(|| fx_range(100_000))
}
