use rusqlite::{Connection, OpenFlags, Params, Error as SqlError};
use std::path::Path;

mod id3;
pub use id3::Id3;

/// Media Database
pub struct Mdb {
  conn: Connection
}

impl Mdb {
  pub fn new<P: AsRef<Path>>(path: Option<P>) -> Result<Mdb, SqlError> {
    let conn = match path {
      Some(p) => Connection::open(p)?,
      None => Connection::open_in_memory()?,
    };

    Ok(Mdb{conn})
  }

  pub fn new_with_flags(path: Option<&Path>, flags: OpenFlags) -> Result< Mdb, SqlError> {
    let conn = match path {
      Some(p) => Connection::open_with_flags(p, flags)?,
      None => Connection::open_in_memory_with_flags(flags)?,
    };

    Ok(Mdb{conn})
  }

  pub fn exec<P: Params>(&self, sql: &str, params: P) -> Result<usize, SqlError> {
    self.conn.execute(sql, params)
  }
}
