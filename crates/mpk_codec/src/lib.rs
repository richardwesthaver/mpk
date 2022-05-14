//! MPK_CODEC
#[cfg(feature = "ffmpeg")]
pub mod ffmpeg;
#[cfg(feature = "snd")]
pub mod snd;

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
  Snd(snd::SndFileError),
  Ffmpeg(ffmpeg::Error),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::Snd(ref e) => write!(f, "{:?}", e),
      Error::Ffmpeg(ref e) => e.fmt(f),
    }
  }
}

impl std::error::Error for Error {}

impl From<snd::SndFileError> for Error {
  fn from(e: snd::SndFileError) -> Error {
    Error::Snd(e)
  }
}

impl From<ffmpeg::Error> for Error {
  fn from(e: ffmpeg::Error) -> Error {
    Error::Ffmpeg(e)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AudioMetadata {
  pub path: PathBuf,
  pub duration: f64,
  pub channels: u16,
  pub sr: u32,
  pub tags: Option<HashMap<String, String>>,
}

impl AudioMetadata {
  #[cfg(feature = "ffmpeg")]
  pub fn from_ffmpeg<P: AsRef<Path>>(path: P) -> Result<AudioMetadata, Error> {
    match ffmpeg::decode(&path) {
      Ok(ctx) => {
        let tags = ffmpeg::get_tags(&ctx);
        if let Some(stream) = ctx.streams().best(ffmpeg::media::Type::Audio) {
          let duration = stream.duration() as f64 * f64::from(stream.time_base());
          let codec =
            ffmpeg::codec::context::Context::from_parameters(stream.parameters())
              .unwrap();
          match codec.decoder().audio() {
            Ok(audio) => {
              let sr = audio.rate();
              let channels = audio.channels();
              Ok(AudioMetadata {
                path: path.as_ref().to_path_buf(),
                duration,
                channels,
                sr,
                tags,
              })
            }
            Err(e) => Err(e.into()),
          }
        } else {
          Err(ffmpeg::Error::InvalidData.into())
        }
      }
      Err(e) => Err(e.into()),
    }
  }
  #[cfg(feature = "snd")]
  pub fn from_snd<P: AsRef<Path>>(path: P) -> Result<AudioMetadata, Error> {
    match snd::decode(&path) {
      Ok(mut ctx) => {
        let sr = ctx.get_samplerate() as u32;
        let channels = ctx.get_channels() as u16;
        let duration = ctx.len().unwrap() as f64 / sr as f64;
        let tags = snd::get_tags(&ctx);
        Ok(AudioMetadata {
          path: path.as_ref().to_path_buf(),
          duration,
          channels,
          sr,
          tags,
        })
      }
      Err(e) => Err(e.into()),
    }
  }

  pub fn get_tag(&self, key: &str) -> Option<String> {
    if let Some(tags) = &self.tags {
      tags.get(key).map(|v| v.to_string())
    } else {
      None
    }
  }
}

pub struct Resample {}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(feature = "ffmpeg")]
  #[test]
  fn ffmpeg_decode_test() {
    assert!(ffmpeg::decode("../../tests/ch1.wav").is_ok())
  }

  #[cfg(feature = "ffmpeg")]
  #[test]
  fn ffmpeg_get_tags_test() {
    for (k, v) in
      ffmpeg::get_tags(&ffmpeg::decode("../../tests/aaaa-Sleepy_4_Life.flac").unwrap())
        .unwrap()
        .iter()
    {
      dbg!(format!("{}: {}", k, v));
    }
  }

  #[cfg(feature = "ffmpeg")]
  #[test]
  fn ffmpeg_metadata_test() {
    let meta = AudioMetadata::from_ffmpeg("../../tests/slayer-Flesh_Storm.mp3");
    assert!(meta.is_ok());
    let meta = meta.unwrap();
    dbg!(&meta.get_tag("artist"));
    dbg!(&meta.get_tag("album"));
    dbg!(&meta.get_tag("genre"));
    dbg!(&meta.get_tag("date"));
  }

  #[cfg(feature = "ffmpeg")]
  #[test]
  fn ffmpeg_transcode_audio_test() {
    ffmpeg::transcode_audio(
      "../../tests/slayer-Flesh_Storm.mp3",
      "../../tests/slayer-Flesh_Storm.wav",
      Some("atempo=2.0".into()),
      Some(2),
    );
  }

  #[cfg(feature = "ffmpeg")]
  #[test]
  fn ffmpeg_get_audio_data_test() {
    let (data, fmt, _, _) =
      ffmpeg::get_audio_data("../../tests/slayer-Flesh_Storm.mp3").unwrap();
    dbg!(data.len(), fmt);
    let f1 = &data[0];
    let f2 = &data[100];
    //    dbg!(f2);
    assert_ne!(f1, f2);
  }

  #[cfg(feature = "snd")]
  #[test]
  fn snd_decode_test() {
    assert!(snd::decode("../../tests/ch1.wav").is_ok())
  }

  #[cfg(feature = "snd")]
  #[test]
  fn snd_get_tags_test() {
    for (k, v) in
      snd::get_tags(&snd::decode("../../tests/aaaa-Sleepy_4_Life.flac").unwrap())
        .unwrap()
        .iter()
    {
      dbg!(format!("{}: {}", k, v));
    }
  }

  #[cfg(feature = "snd")]
  #[test]
  fn snd_metadata_test() {
    assert!(crate::AudioMetadata::from_snd("../../tests/ch1.wav").is_ok())
  }
}
