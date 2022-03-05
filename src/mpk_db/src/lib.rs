use rusqlite::{Connection, OpenFlags, ToSql};
use std::path::Path;
use mpk_config::DbConfig;
pub use mpk_id3::Id3;

mod err;
pub use err::{Error, Result};

/// MPK Database
#[derive(Debug)]
pub struct Mdb {
  conn: Connection,
}

impl Mdb {
  pub fn new(path: Option<&Path>) -> Result<Mdb> {
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
    let flags: OpenFlags = OpenFlags::from_bits(cfg.flags().unwrap()).unwrap();
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

  pub fn exec_batch(&self, sql: &str) -> Result<()> {
    self.conn.execute_batch(sql)?;
    Ok(())
  }

  pub fn exec(&self, sql: &str, params: &[&dyn ToSql]) -> Result<usize> {
    let res = self.conn.execute(sql, params)?;
    Ok(res)
  }


  pub fn last_insert_rowid(&self) -> i64 {
    self.conn.last_insert_rowid()
  }

  pub fn init(&self) -> Result<()> {
    let sql = include_str!("init.sql");

    self.exec_batch(sql)
  }

  pub fn insert_track(&self, path: &str) -> Result<i64> {
    self.exec("insert into tracks (path) values (?)", &[&path])?;
    Ok(self.last_insert_rowid())
  }

  pub fn insert_track_tags(&self,
			   id: i64, artist: Option<String>,
			   title: Option<String>, album: Option<String>,
			   genre: Option<String>, year: Option<i16>) -> Result<()> {
    for t in [&artist, &title, &album, &genre, &year.map(|x| x.to_string())] {
      if t.is_some() {
	self.exec("insert into track_tags (track_id, artist, title, album, genre, year)
               values (?,?,?,?,?,?)", &[&id, &artist, &title, &album, &genre, &year])?;
	break;
      }
    }
    Ok(())
  }

  pub fn insert_track_tags_musicbrainz(&self, tags: MusicbrainzTags) -> Result<()> {
    Ok(())
  }

  pub fn insert_track_features_lowlevel(&self, features: LowlevelFeatures) -> Result<()> {
    Ok(())
  }

  pub fn insert_track_features_rhythm(&self, features: RhythmFeatures) -> Result<()> {
    Ok(())
  }

  pub fn insert_track_features_sfx(&self, features: SfxFeatures) -> Result<()> {
    Ok(())
  }

  pub fn insert_track_features_tonal(&self, features: TonalFeatures) -> Result<()> {
    Ok(())
  }

  pub fn insert_track_user_notes(&self, note: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_track_user_tags(&self, tag: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample(&self, path: &str) -> Result<()> {
    self.exec("insert into samples (path) values (?)", &[&path])?;
    Ok(())
  }

  pub fn insert_sample_features_lowlevel(&self, features: LowlevelFeatures) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample_features_rhythm(&self, features: RhythmFeatures) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample_features_sfx(&self, features: SfxFeatures) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample_features_tonal(&self, features: TonalFeatures) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample_user_notes(&self, note: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_sample_user_tags(&self, tag: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_project(&self, name: &str, path: &str, ty: &str) -> Result<()> {
    self.exec("insert into projects (name, path, type) values (?,?,?)", &[&name, &path, &ty])?;
    Ok(())
  }

  pub fn insert_project_user_notes(&self, note: &str, append: bool) -> Result<()> {
    Ok(())
  }

  pub fn insert_project_user_tags(&self, tag: &str, append: bool) -> Result<()> {
    Ok(())
  }
}
