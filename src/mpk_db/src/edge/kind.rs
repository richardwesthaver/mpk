use super::{Serialize, Deserialize, Archive};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[archive_attr(derive(Debug))]
pub enum EdgeKind {
  Next(u64, u64),
  Similar(u64, f32, u64),
  Compliment(u64, u64),
  Compose(u64, u64),
 }

impl EdgeKind {
  pub fn inbound(&self) -> u64 {
    match *self {
      EdgeKind::Next(i, _) => i,
      EdgeKind::Similar(i, _, _) => i,
      EdgeKind::Compliment(i, _) => i,
      EdgeKind::Compose(i, _) => i,
    }
  }
  pub fn outbound(&self) -> u64 {
    match *self {
      EdgeKind::Next(_, o) => o,
      EdgeKind::Similar(_, _, o) => o,
      EdgeKind::Compliment(_, o) => o,
      EdgeKind::Compose(_, o) => o,
    }
  }
}
