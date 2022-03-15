use id3::Tag;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::err::Result;

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