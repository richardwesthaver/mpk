//! MPK_ANALYSIS
use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus, Output};

use chromaprint::{Chromaprint, ChromaprintAlgorithm};
#[cfg(feature = "ffmpeg")]
use mpk_codec::ffmpeg;
#[cfg(feature = "snd")]
use mpk_codec::snd;
use mpk_codec::AudioMetadata;
use mpk_util::walk_dir;

#[derive(Debug)]
pub enum Error {
  Chromaprint(chromaprint::Error),
  Codec(mpk_codec::Error),
}

impl From<chromaprint::Error> for Error {
  fn from(e: chromaprint::Error) -> Error {
    Error::Chromaprint(e)
  }
}

impl From<mpk_codec::Error> for Error {
  fn from(e: mpk_codec::Error) -> Error {
    Error::Codec(e)
  }
}

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
pub fn chromaprint_extract<P: AsRef<Path>>(path: P) -> Result<Vec<u32>, Error> {
  let (data, sr, ch) = ffmpeg::get_audio_resample(
    path,
    ffmpeg::format::Sample::I16(ffmpeg::format::sample::Type::Planar),
  )
  .map_err(|e| Error::Codec(e.into()))?;
  let mut cp = Chromaprint::new();
  cp.start(sr, ch.into())?;
  for i in data {
    cp.feed(&i.plane(0));
  }
  cp.finish();
  cp.raw_fingerprint().map_err(|e| Error::Chromaprint(e))
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
    let fp = fpcalc_extract("../../tests/slayer-Flesh_Storm.mp3");
    assert!(fp.is_ok());
    println!("{:?}", fp.unwrap());
  }

  #[cfg(feature = "chromaprint")]
  #[test]
  fn cp_extract_test() {
    let fp = chromaprint_extract("../../tests/slayer-Flesh_Storm.mp3").unwrap();
    //    dbg!(fp);
    let encoded = chromaprint::encode_fingerprint(
      &fp,
      chromaprint::ChromaprintAlgorithm::default(),
      true,
    )
    .unwrap();
    dbg!(encoded);
  }
}
