//! MPK_DB -- NODE_KIND
use std::path::PathBuf;
use std::str::FromStr;
use rkyv::with::AsString;
use super::{NodeError, Serialize, Deserialize, Archive};

/// B3 hash checksum - 256-bit value. We could adjust the OUTPUT_LEN
/// parameter if needed.
#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[archive_attr(derive(Debug))]
pub struct Checksum([u8; 32]);

impl From<mpk_hash::Checksum> for Checksum {
  fn from(c: mpk_hash::Checksum) -> Checksum {
    Checksum(*c.0.as_bytes())
  }
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[archive_attr(derive(Debug))]
pub enum NodeKind {
  Track(#[with(AsString)] PathBuf, Checksum),
  Sample(#[with(AsString)] PathBuf, Checksum),
  Midi(#[with(AsString)] PathBuf, Checksum),
  Patch(#[with(AsString)] PathBuf, Checksum),
}

impl NodeKind {
  pub fn new(node: &str) -> Result<NodeKind, NodeError> {
    NodeKind::from_str(node)
  }

  pub fn path(&self) -> &str {
    match self {
      NodeKind::Track(p,_) => p.to_str().unwrap(),
      NodeKind::Sample(p,_) => p.to_str().unwrap(),
      NodeKind::Midi(p,_) => p.to_str().unwrap(),
      NodeKind::Patch(p,_) => p.to_str().unwrap(),
    }
  }
}

impl FromStr for NodeKind {
  type Err = NodeError;
  fn from_str(str: &str) -> Result<NodeKind, NodeError> {
    let mut split = str.split(":");
    let prefix = split.next().unwrap();
    let path = PathBuf::from(split.next().unwrap());
    let checksum = mpk_hash::Checksum::from_path(&path).into();
    match prefix {
      "track" => Ok(
	NodeKind::Track(
	  path,
	  checksum,
	)
      ),
      "sample"=> Ok(
	NodeKind::Sample(
	  path,
	  checksum,
	)
      ),
      "midi"=> Ok(
	NodeKind::Midi(
	  path,
	  checksum,
	)
      ),
      "patch"=> Ok(
	NodeKind::Patch(
	  path,
	  checksum,
	)
      ),
      e => Err(NodeError::BadNodeKind(e.to_string())),
    }
  }
}
