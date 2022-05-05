//! MPK_DB
#![feature(generic_associated_types)]
mod db;
mod err;
mod factory;
mod query;
mod tree;
mod types;

pub use db::{Db, DbRef};
pub use err::ValidationError;
pub use factory::{
  EdgeFactory, EdgePropFactory, Factory, MetaFactory, NodeFactory, NodePropFactory,
};
pub use query::{EdgeQuery, EdgeQueryExt, NodeQuery, NodeQueryExt};
pub use tree::{EdgeTree, NodeTree, TreeHandle, TREE_NAMES};
pub use types::*;

pub use sled::Batch;

#[cfg(test)]
mod tests {
  use super::*;
  use mpk_config::DbConfig;
  use ulid::Ulid;

  fn db() -> Db {
    Db::open(None).unwrap()
  }

  #[test]
  fn sled_test() {
    let tree = sled::open("/tmp/testdb").unwrap();
    assert!(tree.insert("test", "test").unwrap().is_some());
    assert!(tree
      .compare_and_swap("test", Some("test"), Some("test2"))
      .unwrap()
      .is_ok());
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

    let nodekind = NodeKind::Track;
    let node = Node::new(nodekind);
    let node_val: Vec<u8> = bincode::serialize(node.val()).unwrap();
    let node_key: Vec<u8> = bincode::serialize(node.key()).unwrap();
    db.inner().insert(&node_key, node_val).unwrap();
    let node_val = db.inner().get(node_key.as_slice()).unwrap();
    assert!(node_val.is_some());
    let unwrapped = node_val.unwrap();
    let node: NodeKind = bincode::deserialize(&unwrapped).unwrap();
    println!("{node_key:?}: {node:?}");
  }

  #[test]
  fn node_factory_test() {
    let factory = NodeFactory;
    let kind1 = Node::new(NodeKind::Track);
    let kind2 = Node::new(NodeKind::Sample);
    let kind3 = Node::new(NodeKind::Midi);
    let kind4 = Node::new(NodeKind::Patch);
    let nodes = vec![vec![kind1, kind2, kind3, kind4]]
      .into_iter()
      .flat_map(|n| std::iter::repeat(n).take(20))
      .flatten()
      .collect();
    let bytes = factory.serialize_vec(nodes).unwrap();
    for i in 0..bytes.0.len() {
      assert_eq!(Id::from(bytes.0[i].as_slice()).to_string().len(), 26)
    }
  }

  #[test]
  fn edge_factory_test() {
    let factory = EdgeFactory;
    let edge = Edge::new(EdgeKey::new(
      EdgeKind::Next,
      Id::from(Ulid::new()),
      Id::from(Ulid::new()),
    ));
    let key = factory.serialize_key(&edge);
    let val = factory.serialize_val(&edge);
    assert!(key.is_ok());
    assert!(val.is_ok());
  }

  #[test]
  fn node_tree_test() {
    let db = db();
    let mut tree = NodeTree::open(db.inner(), "test").unwrap();
    let node = Node::with_id(1000, NodeKind::Track);
    let insert = &tree.insert(&node).unwrap();
    assert_eq!(insert, &None);
    let get = &tree.get(&1000).unwrap();
    assert!(get.is_some());
    assert!(&tree.exists(&1000).unwrap());
    println!(
      "{}",
      &tree
        .factory
        .deserialize_key::<Id>(tree.first().unwrap().unwrap().0.to_vec().as_slice())
        .unwrap()
    );
    dbg!(&tree
      .factory
      .deserialize_val::<NodeKind>(tree.first().unwrap().unwrap().1.to_vec().as_slice())
      .unwrap());
    let mut batch = sled::Batch::default();
    let (k, v) = tree.factory.serialize(&node).unwrap();
    batch.insert(k, v);
    assert!(tree.batch(batch).is_ok());
  }
}