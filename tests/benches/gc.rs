#![feature(test)]
#![cfg(test)]
extern crate test;
use mpk_gc::Gc;
use test::Bencher;

#[bench]
fn collect_u64(b: &mut Bencher) {
  b.iter(|| {
    let i = Gc::new(0u64);
  })
}
