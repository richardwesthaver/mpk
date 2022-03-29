pub use blake3::{
  derive_key, hash, keyed_hash, Hash as B3Hash, Hasher as B3Hasher, OutputReader,
};
use rand::Rng;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
pub const KEY_LEN: usize = 32;
pub const OUT_LEN: usize = 32;
pub const HEX_LEN: usize = KEY_LEN * 2;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Checksum(pub B3Hash);

impl Checksum {
  pub fn rand() -> Self {
    let mut rng = rand::thread_rng();
    let vals: [u8; KEY_LEN] = (0..KEY_LEN)
      .map(|_| rng.gen_range(0..u8::MAX))
      .collect::<Vec<u8>>()
      .as_slice()
      .try_into()
      .unwrap();
    Checksum(B3Hash::from(vals))
  }
  pub fn hash(input: &[u8]) -> Self {
    let output = blake3::hash(input);
    Checksum(output)
  }
  pub fn to_hex(&self) -> String {
    self.0.to_hex().to_string()
  }
  pub fn from_hex(h: &str) -> Self {
    Checksum(B3Hash::from_hex(h).unwrap())
  }
  pub fn from_file(f: File) -> Self {
    let mut buf_reader = BufReader::new(f);
    let mut buf = vec![];
    buf_reader.read_to_end(&mut buf).unwrap();
    Checksum::hash(buf.as_slice())
  }
  pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
    let file = File::open(path).unwrap();
    Checksum::from_file(file)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use hex::decode;
  use std::convert::TryInto;
  #[test]
  fn rand_id() {
    let id = Checksum::rand();
    assert_eq!(id.0.as_bytes().len(), OUT_LEN);
  }

  #[test]
  fn hex_hash() {
    let mut hasher1 = B3Hasher::new();
    hasher1.update(b"foo");
    hasher1.update(b"bar");
    hasher1.update(b"baz");
    let out1 = hasher1.finalize();
    let mut xof1 = [0; 301];
    hasher1.finalize_xof().fill(&mut xof1);
    assert_eq!(out1.as_bytes(), &xof1[..32]);

    let hash_hex = "d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24";
    let hash_bytes = decode(hash_hex).unwrap();
    let hash_array: [u8; blake3::OUT_LEN] = hash_bytes[..].try_into().unwrap();
    let _hash: B3Hash = hash_array.into();
  }

  #[test]
  fn path_checksum() {
    let checksum = Checksum::from_path("Cargo.toml");
    dbg!(checksum);
    let checksum = Checksum::from_file(File::open("src/lib.rs").unwrap());
    dbg!(checksum);
  }
}
