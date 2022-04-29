//! MPK_DB -- DB
use sled::{Db as SledDb, Error, IVec, CompareAndSwapError, Tree};
use std::path::Path;
use mpk_config::{DbConfig, DbMode};

pub type DbRef<'a> = &'a SledDb;

fn into_db_mode(mode: DbMode) -> sled::Mode {
  match mode {
    DbMode::Small => sled::Mode::LowSpace,
    DbMode::Fast => sled::Mode::HighThroughput,
  }
}

#[derive(Debug)]
pub struct Db {
  db: SledDb,
}

impl Db {
  /// Open a database. If path is Some, open or create database at
  /// that path else create a temporary database.
  pub fn open(path: Option<&Path>) -> Result<Db, Error> {
    let db = if let Some(p) = path {
      sled::open(p)?
    } else {
      sled::Config::new().temporary(true).open()?
    };
    Ok(Db { db })
  }

  /// Open a database with DbConfig CFG.
  pub fn with_config(cfg: DbConfig) -> Result<Db, Error> {
    let db = sled::Config::new()
      .path(cfg.path)
      .mode(into_db_mode(cfg.mode))
      .use_compression(cfg.use_compression)
      .compression_factor(cfg.compression_factor)
      .open()?;

      Ok(Db { db })
  }

  /// Return a ref to the inner Database.
  pub fn inner<'a>(&'a self) -> DbRef<'a> {
    &self.db
  }

  /// Flush the database to disk.
  pub fn flush(&self) -> Result<usize, Error> {
    self.db.flush()
  }

  pub async fn flush_async(&self) -> Result<usize, Error> {
    self.db.flush_async().await
  }

  /// Print info about the current database.
  pub fn info(&self) -> Result<(), Error> {
    println!("trees:");
    for i in self.db.tree_names() {
      println!("{}", std::str::from_utf8(&i).unwrap());
    }
    println!("CRC32: {}", self.db.checksum()?);
    println!("size: {}", self.db.size_on_disk()?);
    Ok(())
  }

  pub fn open_tree<N: AsRef<[u8]>>(&self, name: N) -> Result<Tree, Error> {
    self.db.open_tree(name)
  }

  pub fn drop_tree<N: AsRef<[u8]>>(&self, name: N) -> Result<bool, Error> {
    self.db.drop_tree(name)
  }

  pub fn insert<K: AsRef<[u8]>, V: Into<IVec>>(&self, key: K, val: V) -> Result<Option<IVec>, Error> {
    self.db.insert(key, val)
  }

  pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<IVec>, Error> {
    self.db.get(key)
  }

  pub fn remove<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<IVec>, Error> {
    self.db.remove(key)
  }

  pub fn swap<A: AsRef<[u8]>, B: Into<IVec>>(&self, key: A, old: Option<A>, new: Option<B>) -> Result<Result<(), CompareAndSwapError>, Error> {
    self.db.compare_and_swap(key, old, new)
  }
}