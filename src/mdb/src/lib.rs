use rusqlite::{Connection, OpenFlags, Params, Error as SqlError};
use std::path::Path;
use mpk_config::DbConfig;
mod id3;
pub use id3::Id3;

/// Media Database
pub struct Mdb {
  conn: Connection,
  cfg: DbConfig,
}

impl Mdb {
  pub fn new<P: AsRef<Path>>(path: Option<P>) -> Result<Mdb, SqlError> {
    let cfg = DbConfig::default();
    let conn = match path {
      Some(p) => Connection::open(p)?,
      None => Connection::open_in_memory()?,
    };

    Ok(
      Mdb {
	conn,
	cfg,
      }
    )
  }

  pub fn new_with_config(path: Option<&Path>, cfg: DbConfig) -> Result< Mdb, SqlError> {
    let flags: OpenFlags = OpenFlags::from_bits(cfg.c_flags().unwrap()).unwrap();
    let conn = match path {
      Some(p) => Connection::open_with_flags(p, flags)?,
      None => Connection::open_in_memory_with_flags(flags)?,
    };

    Ok(
      Mdb {
	conn,
	cfg,
      }
    )
  }

  pub fn exec<P: Params>(&self, sql: &str, params: P) -> Result<usize, SqlError> {
    self.conn.execute(sql, params)
  }
}
