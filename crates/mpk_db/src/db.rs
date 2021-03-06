//! MPK_DB -- DB
use std::path::Path;
use std::sync::Arc;

use mpk_config::{DbConfig, DbMode};
use mpk_util::{expand_tilde, format_byte_size};
use sled::{CompareAndSwapError, Db as SledDb, IVec, Tree};

use crate::Error;

pub type DbRef = Arc<SledDb>;

/// according to discord these options are noop in current sled version
fn into_db_mode(mode: DbMode) -> sled::Mode {
  match mode {
    DbMode::Small => sled::Mode::LowSpace,
    DbMode::Fast => sled::Mode::HighThroughput,
  }
}

/// The MPK Database. This struct wraps the internal Sled key-value
/// store in an Arc that is cloned with the `inner` method and used to
/// initialize types that implement TreeHandle.
#[derive(Debug)]
pub struct Db {
  db: Arc<SledDb>,
}

impl Db {
  /// Open a database. If path is Some, open or create database at
  /// that path else create a temporary database.
  pub fn open<P: AsRef<Path>>(path: Option<P>) -> Result<Db, Error> {
    let db = if let Some(p) = path {
      sled::open(expand_tilde(p).unwrap())?
    } else {
      sled::Config::new().temporary(true).open()?
    };
    Ok(Db { db: Arc::new(db) })
  }

  /// Open a database with DbConfig CFG.
  pub fn with_config(cfg: DbConfig) -> Result<Db, Error> {
    let db = sled::Config::new()
      .path(expand_tilde(cfg.path).unwrap())
      .mode(into_db_mode(cfg.mode))
      .use_compression(cfg.use_compression)
      .compression_factor(cfg.compression_factor)
      .open()?;

    Ok(Db { db: Arc::new(db) })
  }

  /// Return a ref to the inner Database.
  pub fn inner(&self) -> DbRef {
    self.db.clone()
  }

  /// Flush the database to disk.
  pub fn flush(&self) -> Result<usize, Error> {
    self.db.flush().map_err(|e| e.into())
  }

  /// Flush the database to disk (async).
  pub async fn flush_async(&self) -> Result<usize, Error> {
    self.db.flush_async().await.map_err(|e| e.into())
  }

  /// Print info about the current database.
  pub fn info(&self) -> Result<(), Error> {
    println!("trees:");
    for i in self.db.tree_names() {
      let str = std::str::from_utf8(&i).unwrap();
      print!("{}: ", str);
      println!("{}", self.open_tree(str)?.len())
    }
    println!("CRC32: {}", self.db.checksum()?);
    println!("size: {}", format_byte_size(self.db.size_on_disk()?));
    Ok(())
  }

  /// Open a tree by NAME. Returns the default handle from Sled.
  pub fn open_tree<N: AsRef<[u8]>>(&self, name: N) -> Result<Tree, Error> {
    self.db.open_tree(name).map_err(|e| e.into())
  }

  /// Drop a tree by NAME.
  pub fn drop_tree<N: AsRef<[u8]>>(&self, name: N) -> Result<bool, Error> {
    self.db.drop_tree(name).map_err(|e| e.into())
  }

  /// Insert KEY with value VAL into the default database tree. This
  /// should only be used as a worker queue - separate trees are used
  /// for nodes, edges, etc.
  pub fn insert<K: AsRef<[u8]>, V: Into<IVec>>(
    &self,
    key: K,
    val: V,
  ) -> Result<Option<IVec>, Error> {
    self.db.insert(key, val).map_err(|e| e.into())
  }

  /// Get a value from the default tree by KEY.
  pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<IVec>, Error> {
    self.db.get(key).map_err(|e| e.into())
  }

  /// Remove a value from the default tree by KEY.
  pub fn remove<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<IVec>, Error> {
    self.db.remove(key).map_err(|e| e.into())
  }

  /// Compare and swap a value at KEY.
  pub fn swap<A: AsRef<[u8]>, B: Into<IVec>>(
    &self,
    key: A,
    old: Option<A>,
    new: Option<B>,
  ) -> Result<Result<(), CompareAndSwapError>, Error> {
    self
      .db
      .compare_and_swap(key, old, new)
      .map_err(|e| e.into())
  }
}
