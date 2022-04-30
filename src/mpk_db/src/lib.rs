//! MPK_DB
#![feature(generic_associated_types)]
mod node;
mod edge;
mod id;
mod ser;
mod factory;
mod tree;
mod db;

pub use node::{Node, NodeKind, NodeVec};
pub use edge::{Edge, EdgeKind, EdgeVec};
pub use id::Id;
pub use ser::{NodeSerializer, EdgeSerializer};
pub use factory::{Factory, NodeFactory, EdgeFactory};
pub use tree::TREE_NAMES;
pub use db::{Db, DbRef};

#[cfg(test)]
mod tests {
  use super::*;
  use mpk_config::DbConfig;
  use mpk_hash::Djb2;
  use rkyv::ser::{serializers::AllocSerializer};
  use rkyv::archived_root;

  fn db() -> Db {
    Db::open(None).unwrap()
  }

  #[test]
  fn sled_test() {
    let tree = sled::open("/tmp/testdb").unwrap();
    assert!(tree.insert("test", "test").unwrap().is_some());
    assert!(tree.compare_and_swap("test", Some("test"), Some("test2")).unwrap().is_ok());
    assert!(tree.flush().is_ok());
    assert!(tree.open_tree("new_tree_1").is_ok());
    assert_eq!(tree.drop_tree("new_tree_1").unwrap(), true);
  }
  #[test]
  fn db_handle_test() {
    let db = db();
    db.inner().tree_names();
    assert!(db.flush().is_ok());
    let cfg = DbConfig {
      path: "/tmp/testdb1".into(),
      ..Default::default()
    };
    let db1 = Db::with_config(cfg).unwrap();
    assert!(db1.flush().is_ok());
    assert!(db.info().is_ok());

    let mut ser = NodeSerializer::<AllocSerializer<1024>>::default();
    let nodekind = NodeKind::new("track:/Users/ellis/dev/mpk/tests/ch1.wav").unwrap();
    let mut hasher = Djb2::default();
    let node = Node::new(nodekind, &mut hasher);
    node.serialize(&mut ser).unwrap();
    let node_val = ser.into_inner().into_serializer().into_inner();
    let node_key = node.key();
    db.inner().insert(node_key, node_val.as_slice()).unwrap();
    let node_val = db.inner().get(node_key).unwrap();
    assert!(node_val.is_some());
    let unwrapped = node_val.unwrap();
    let node = unsafe {archived_root::<NodeKind>(&unwrapped)};
    println!("{node_key:?}: {node:?}");
  }

  #[test]
  fn node_factory_test() {
    let mut factory = NodeFactory::<1024, Djb2>::new();
    let kind1 = NodeKind::new("track:/Users/ellis/dev/mpk/tests/ch1.wav").unwrap();
    let kind2 = NodeKind::new("sample:/Users/ellis/dev/mpk/tests/ch2.wav").unwrap();
    let kind3 = NodeKind::new("midi:/Users/ellis/dev/mpk/Cargo.toml").unwrap();
    let kind4 = NodeKind::new("patch:/Users/ellis/dev/mpk/config.nims").unwrap();
    let nodes = vec![vec![kind1, kind2, kind3, kind4]].into_iter()
      .flat_map(|n| std::iter::repeat(n).take(20)).flatten().collect();
    let bytes = factory.serialize_vec(nodes);
    for i in 0..bytes.0.len() {
      unsafe { archived_root::<NodeKind>(&bytes.1[i]) };
      dbg!(u64::from_be_bytes(bytes.0[i]));
    }
  }

  #[test]
  fn edge_factory_test() {
    let mut factory = EdgeFactory::<1024>;
    let edge = Edge::new(EdgeKind::Next(1234, 5678));
    let mut ser = factory.serializer();
    factory.serialize_val(&edge, &mut ser).unwrap();
    let bytes = factory.flush_bytes(ser);
    let edge= unsafe { archived_root::<u64>(&bytes) };
    assert_eq!(*edge, 5678);
  }
}
