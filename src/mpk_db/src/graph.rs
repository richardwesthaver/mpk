//! MPK_DB -- GRAPH
use crate::db::DbRef;
use crate::db::Db;

pub struct Graph<'a> {
  handle: DbRef<'a>,
}

impl<'a> Graph<'a> {
  pub fn new(handle: DbRef) -> Graph {
    Graph {
      handle
    }
  }
}
