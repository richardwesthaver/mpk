pub use blake3::{derive_key, hash, keyed_hash, Hash as B3Hash, Hasher as B3Hasher, OutputReader};
use hex;
use rand::Rng;

pub const KEY_LEN: usize = 32;
pub const OUT_LEN: usize = 32;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Default)]
pub struct Checksum([u8;OUT_LEN]);

impl Checksum {
  pub fn rand() -> Self {
    let mut rng = rand::thread_rng();
    let vals: [u8;KEY_LEN] = (0..KEY_LEN).map(|_| rng.gen_range(0..u8::MAX)).collect::<Vec<u8>>().as_slice().try_into().unwrap();
    Checksum(vals)
  }

  pub fn state_hash(&self, state: &mut B3Hasher) -> Self {
    let mut output = [0; OUT_LEN];
    state.update(&self.0);
    let mut res = state.finalize_xof();
    res.fill(&mut output);
    Checksum(output)
  }

  pub fn to_hex(&self) -> String {
    hex::encode(&self.0)
  }

  pub fn from_bytes(b: &[u8]) -> Self {
    Checksum(*blake3::hash(b).as_bytes())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use hex::decode;
  use std::convert::TryInto;
  #[test]
  fn id_state_hash() {
    let id = Checksum([0; KEY_LEN]);
    let hash = id.state_hash(&mut B3Hasher::new());
    assert_eq!(hash, id.state_hash(&mut B3Hasher::new()));
  }

  #[test]
  fn id_hex() {
    let id = Checksum([255; KEY_LEN]);

    assert_eq!(
      hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
      id.0
    );
  }

  #[test]
  fn rand_id() {
    let id = Checksum::rand();
    let hash = id.state_hash(&mut B3Hasher::new());
    assert_eq!(hash, id.state_hash(&mut B3Hasher::new()));
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
}
