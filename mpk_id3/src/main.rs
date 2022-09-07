use id3::Tag;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

// ERRORS
#[derive(Debug)]
pub enum Error {
  Id3(id3::Error),
  Io(std::io::Error),
  Json(serde_json::Error),
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Error::Id3(_) => None,
      Error::Io(_) => None,
      Error::Json(_) => None,
    }
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Id3(ref err) => err.fmt(f),
      Error::Io(ref err) => err.fmt(f),
      Error::Json(ref err) => err.fmt(f),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    Error::Io(err)
  }
}

impl From<id3::Error> for Error {
  fn from(err: id3::Error) -> Error {
    Error::Id3(err)
  }
}

impl From<serde_json::Error> for Error {
  fn from(err: serde_json::Error) -> Error {
    Error::Json(err)
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Id3 {
  pub path: PathBuf,
  pub tags: Option<HashMap<String, String>>,
}

impl Id3 {
  pub fn new<P: AsRef<Path>>(path: P) -> Result<Id3> {
    let tags = match Tag::read_from_path(&path) {
      Ok(t) => {
        let mut map = HashMap::new();
        for f in t.frames().into_iter() {
          map.insert(f.id().to_string(), f.content().to_string());
        }
        Some(map)
      }
      Err(_) => None,
    };

    let path = path.as_ref().to_path_buf();
    Ok(Id3 { path, tags })
  }

  pub fn get_tag(&self, tag: &str) -> Option<String> {
    match &self.tags {
      Some(tags) => {
        if let Some(t) = tags.get(tag) {
          Some(t.to_owned())
        } else {
          None
        }
      }
      None => None,
    }
  }

  pub fn to_json_string(&self) -> Result<String> {
    let json = serde_json::to_string_pretty(self)?;
    Ok(json)
  }

  pub fn to_json_writer<P: AsRef<Path>>(&self, path: P) -> Result<()> {
    let out = fs::OpenOptions::new()
      .create_new(true)
      .append(true)
      .open(&path)?;
    serde_json::to_writer_pretty(out, self)?;
    Ok(())
  }
}

pub fn id3_walk<P: AsRef<Path>>(path: P, coll: &mut Vec<Id3>) -> Result<()> {
  let path = path.as_ref();
  if path.is_dir() {
    for elt in fs::read_dir(path)? {
      let elt = elt?;
      let p = elt.path();
      if p.is_dir() {
        id3_walk(&p, coll)?;
      } else if p.is_file() {
        println!("parsing {:?}", p);
        if let Ok(t) = Id3::new(&p) {
          coll.push(t);
        }
      }
    }
  } else if path.is_file() {
    if let Ok(t) = Id3::new(&path) {
      coll.push(t);
    }
  }
  Ok(())
}

fn main() -> Result<()> {
  let mut coll = Vec::new();
  id3_walk(&ts, &mut coll)?;
  for i in coll {
    let path = i.path.strip_prefix(&ts).unwrap().to_str().unwrap();
    let title = i.get_tag("TIT2");
    let artist = i.get_tag("TPE1");
    let album = i.get_tag("TALB");
    let genre = i.get_tag("TCON");
    let year = i.get_tag("TDRC").map(|y| y.parse::<i16>().unwrap());

    conn.insert_track(&path)?;
    let track_id = conn.last_insert_rowid();
    let tags = TrackTags {
      artist,
      title,
      album,
      genre,
      year,
    };
    conn.insert_track_tags(track_id, &tags)?;
  }
}
