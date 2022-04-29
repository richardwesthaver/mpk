//! MPK_ANALYSIS
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use id3::{Tag, Error};
use mpk_util::walk_dir;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Id3 {
  pub path: PathBuf,
  pub tags: Option<HashMap<String, String>>,
}

impl Id3 {
  pub fn new<P: AsRef<Path>>(path: P) -> Result<Id3, Error> {
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
}

pub fn id3_walker<P: AsRef<Path>>(path: P) -> Option<HashMap<String, String>> {
  println!("parsing {:?}", path.as_ref());
  if let Ok(t) = Tag::read_from_path(&path) {
        let mut map = HashMap::new();
        for f in t.frames().into_iter() {
          map.insert(f.id().to_string(), f.content().to_string());
        }
    Some(map)
  } else {
    println!("no tags for {:?}", path.as_ref());
    None
  }
}

pub fn id3_walk<P: AsRef<Path>>(path: P) -> Vec<(PathBuf, HashMap<String, String>)> {
  let mut coll = Vec::new();
  walk_dir(path, id3_walker, &mut coll).unwrap();
  coll
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn id3_walk_test() {
    let tags = id3_walk("/Users/ellis/mpk/tracks/mp3");
    dbg!(&tags);
    dbg!(tags.len());
  }
}
