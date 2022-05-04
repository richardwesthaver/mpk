//! MPK_ANALYSIS
use mpk_codec::snd::decode;
use mpk_util::walk_dir;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub struct Tags {
  pub path: PathBuf,
  pub tags: Option<HashMap<String, String>>,
  pub sr: usize,
  pub channels: usize,
  pub duration: f64,
}

impl Tags {
  pub fn new<P: AsRef<Path>>(
    path: P,
    tags: Option<HashMap<String, String>>,
    sr: usize,
    channels: usize,
    duration: f64,
  ) -> Tags {
    Tags {
      path: path.as_ref().to_path_buf(),
      tags,
      sr,
      channels,
      duration,
    }
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

pub fn tag_walker<P: AsRef<Path>>(path: P) -> Option<Tags> {
  let path = path.as_ref();
  println!("parsing {:?}", path);
  if let Ok(mut snd) = decode(path) {
    let sr = snd.get_samplerate();
    println!("{}", sr);
    let n_frame = snd.len().unwrap();
    let channels = snd.get_channels();
    Some(Tags::new(
      &path,
      None,
      sr,
      channels,
      n_frame as f64 / sr as f64,
    ))
  } else {
    println!("failed to get tags for {:?}", path);
    None
  }
}

pub fn tag_walk<P: AsRef<Path>>(path: P) -> Vec<(PathBuf, Tags)> {
  let mut coll = Vec::new();
  walk_dir(path, tag_walker, &mut coll).unwrap();
  coll
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tag_walk_test() {
    let tags = tag_walk("/Users/ellis/mpk/tracks/mp3");
    dbg!(&tags);
    dbg!(tags.len());
  }
}
