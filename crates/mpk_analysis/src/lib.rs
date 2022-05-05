//! MPK_ANALYSIS
use mpk_codec::AudioMetadata;
use mpk_util::walk_dir;
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::io;

pub fn tag_walker<P: AsRef<Path>>(path: P) -> Option<AudioMetadata> {
  let path = path.as_ref();
  println!("parsing {:?}", path);
  let metadata = if cfg!(feature = "ffmpeg") {
    AudioMetadata::from_ffmpeg(path)
  } else if cfg!(feature = "snd") {
    AudioMetadata::from_snd(path)
  } else {
    panic!("compile with ffmpeg or snd to use this function")
  };
  if let Ok(data) = metadata {
    Some(data)
  } else {
    println!("failed to get tags for {:?}", path);
    None
  }
}

pub fn tag_walk<P: AsRef<Path>>(path: P) -> Vec<AudioMetadata> {
  let mut coll = Vec::new();
  walk_dir(path, tag_walker, &mut coll).unwrap();
  coll
}

pub fn freesound_extract<P: AsRef<Path>>(input: P, output: P, exe: Option<P>) -> Result<ExitStatus, io::Error> {
  let mut cmd = if let Some(x) = exe {
    Command::new(x.as_ref())
  } else {
    Command::new("essentia_streaming_extractor_freesound")
  };
  cmd.args([input.as_ref(), output.as_ref()]).status()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(feature = "ffmpeg")]
  #[test]
  fn ffmpeg_tag_walk_test() {
    let tags = tag_walk("../../tests");
    dbg!(&tags);
    dbg!(tags.len());
  }

  #[cfg(feature = "snd")]
  #[test]
  fn snd_tag_walk_test() {
    let tags = tag_walk("../../tests");
    dbg!(&tags);
    dbg!(tags.len());
  }

  #[test]
  fn fs_extract_test() {
    assert!(freesound_extract("../../tests/luke_vibert-funkyacidstuff.mp3", "../../tests/luke_vibert-funkyacidstuff.json").is_ok())
  }
}