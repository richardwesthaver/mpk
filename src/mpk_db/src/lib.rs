use rusqlite::{Connection, OpenFlags, ToSql};
use std::path::Path;
use mpk_config::DbConfig;
pub use mpk_id3::Id3;

mod err;
pub use err::{Error, Result};

/// Media Database
pub struct Mdb {
  conn: Connection,
}

impl Mdb {
  pub fn new<P: AsRef<Path>>(path: Option<P>) -> Result<Mdb> {
    let conn = match path {
      Some(p) => Connection::open(p)?,
      None => Connection::open_in_memory()?,
    };

    Ok(
      Mdb {
	conn
      }
    )
  }

  pub fn new_with_config(cfg: DbConfig) -> Result<Mdb> {
    let flags: OpenFlags = OpenFlags::from_bits(cfg.c_flags().unwrap()).unwrap();
    let conn = match cfg.path() {
      Some(p) => Connection::open_with_flags(p, flags)?,
      None => Connection::open_in_memory_with_flags(flags)?,
    };

    Ok(
      Mdb {
	conn
      }
    )
  }

  pub fn init(&self) -> Result<()> {
    let sql = r"
pragma foreign_keys = on;

create table if not exists tracks (
id integer primary key,
path text not null,
updated datetime default current_timestamp not null);

create table if not exists track_tags (
track_id integer,
artist text,
title text,
album text,
genre text,
year text,
foreign key(track_id) references tracks(id));

create table if not exists samples (
id integer primary key,
name text not null,
path text not null,
ext text not null,
updated datetime default current_timestamp not null);

create table if not exists projects (
id integer primary key,
name text not null,
path text not null,
type text not null,
updated datetime default current_timestamp not null);";

    self.exec_batch(sql)
  }
  pub fn exec_batch(&self, sql: &str) -> Result<()> {
    self.conn.execute_batch(sql)?;
    Ok(())
  }

  pub fn exec(&self, sql: &str, params: &[&dyn ToSql]) -> Result<usize> {
    let res = self.conn.execute(sql, params)?;
    Ok(res)
  }
}
