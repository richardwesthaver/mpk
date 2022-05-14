//! MPK_ANALYSIS
use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus, Output};

use chromaprint::{Chromaprint, CHROMAPRINT_ALGORITHM_DEFAULT};
#[cfg(feature = "ffmpeg")]
use mpk_codec::ffmpeg;
#[cfg(feature = "snd")]
use mpk_codec::snd;
use mpk_codec::AudioMetadata;
use mpk_util::walk_dir;

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

pub fn freesound_extract<P: AsRef<Path>>(
  input: P,
  output: P,
  exe: Option<P>,
) -> Result<ExitStatus, io::Error> {
  let mut cmd = if let Some(x) = exe {
    Command::new(x.as_ref())
  } else {
    Command::new("essentia_streaming_extractor_freesound")
  };
  cmd.args([input.as_ref(), output.as_ref()]).status()
}

pub fn musicbrainz_extract<P: AsRef<Path>>(
  input: P,
  output: P,
  exe: Option<P>,
) -> Result<ExitStatus, io::Error> {
  let mut cmd = if let Some(x) = exe {
    Command::new(x.as_ref())
  } else {
    Command::new("essentia_streaming_extractor_music")
  };
  cmd.args([input.as_ref(), output.as_ref()]).status()
}

pub fn fpcalc_extract<P: AsRef<Path>>(input: P) -> Result<Output, io::Error> {
  let mut cmd = Command::new("fpcalc");
  cmd.arg(input.as_ref());
  cmd.arg("-raw").output()
}

#[cfg(feature = "ffmpeg")]
pub fn chromaprint_extract<P: AsRef<Path>>(
  path: P,
  base64: bool,
) -> Result<Option<Vec<u8>>, io::Error> {
  let (data, ty, sr, ch) = ffmpeg::get_audio_data(path)?;
  let mut cp = Chromaprint::new();
  cp.start(sr as i32, ch as i32);
  for i in data {
    cp.feed(&i);
  }
  cp.finish();
  let raw = cp.raw_fingerprint();
  if let Some(r) = raw {
    Ok(Chromaprint::encode(
      &r,
      CHROMAPRINT_ALGORITHM_DEFAULT,
      base64,
    ))
  } else {
    Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      "failed to get raw fingerprint",
    ))
  }
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
    assert!(freesound_extract(
      "../../tests/luke_vibert-bongo_beats.mp3",
      "../../tests/luke_vibert-bongo_beats-fs.json",
      None
    )
    .is_ok())
  }

  #[test]
  fn mb_extract_test() {
    assert!(musicbrainz_extract(
      "../../tests/luke_vibert-bongo_beats.mp3",
      "../../tests/luke_vibert-bongo_beats-mb.json",
      None
    )
    .is_ok())
  }

  #[test]
  fn fp_extract_test() {
    let fp = fpcalc_extract("../../tests/luke_vibert-bongo_beats.mp3");
    assert!(fp.is_ok());
    println!("{:?}", fp.unwrap());
  }

  #[cfg(feature = "chromaprint")]
  #[test]
  fn cp_extract_test() {
    let fp = chromaprint_extract("../../tests/luke_vibert-bongo_beats.mp3", false)
      .unwrap()
      .unwrap();
    //    let fp: &str = std::str::from_utf8(&fp).unwrap();
    println!("{:?}", fp.as_slice());
  }
}
