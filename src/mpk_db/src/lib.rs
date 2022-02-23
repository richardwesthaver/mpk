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
    let sql = r"
pragma foreign_keys = on;

create table if not exists tracks (
id integer primary key,
path text not null,
format text,
channels integer,
filesize integer,
bitrate integer,
bitdepth integer,
duration integer,
samplerate integer,
updated datetime default current_timestamp not null);

create table if not exists track_tags (
track_id integer,
artist text,
title text,
album text,
genre text,
year text,
foreign key(track_id) references tracks(id));

create table if not exists track_stats (
track_id integer,
foreign key(track_id) references tracks(id));

create table if not exists track_images (
track_id integer,
mel_spectra blob,
lin_spectra blob,
waveform blob,
foreign key(track_id) references tracks(id));

create table if not exists track_user_data (
track_id integer,
user_tags text,
notes text,
foreign key(track_id) references tracks(id));

create table if not exists samples (
id integer primary key,
path text not null,
format text,
channels integer,
filesize integer,
bitrate integer,
bitdepth integer,
duration integer,
samplerate integer,
updated datetime default current_timestamp not null);

create table if not exists sample_stats (
sample_id integer,
foreign key(sample_id) references samples(id));

create table if not exists sample_images (
sample_id integer,
mel_spectra blob,
lin_spectra blob,
waveform blob,
foreign key(sample_id) references samples(id));

create table if not exists sample_user_data (
sample_id integer,
user_tags text,
notes text,
foreign key(sample_id) references samples(id));

create table if not exists projects (
id integer primary key,
name text not null,
path text not null,
type text not null,
updated datetime default current_timestamp not null);

create table if not exists project_user_data (
project_id integer,
user_tags text,
notes text,
foreign key(project_id) references projects(id));";

    self.exec_batch(sql)
  }

  pub fn insert_track(&self, path: &str) -> Result<()> {
    self.exec("insert into tracks (path) values (?)", &[&path])?;
    Ok(())
  }

  pub fn insert_track_tags(&self,
			   id: i64, artist: Option<String>,
			   title: Option<String>, album: Option<String>,
			   genre: Option<String>, year: Option<String>) -> Result<()> {
    for t in [&artist, &title, &album, &genre, &year] {
      if t.is_some() {
	self.exec("insert into track_tags (track_id, artist, title, album, genre, year)
               values (?,?,?,?,?,?)", &[&id, &artist, &title, &album, &genre, &year])?;
	break;
      }
    }
    Ok(())
  }

  pub fn insert_sample(&self, path: &str) -> Result<()> {
    self.exec("insert into samples (path) values (?)", &[&path])?;
    Ok(())
  }

  pub fn insert_project(&self, name: &str, path: &str, ty: &str) -> Result<()> {
    self.exec("insert into projects (name, path, type) values (?,?,?)", &[&name, &path, &ty])?;
    Ok(())
  }
}
