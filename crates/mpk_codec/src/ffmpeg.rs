//! MPK_CODEC -- FFMPEG
extern crate ffmpeg_next as ffmpeg;
pub use ffmpeg::{Error, format, decoder, init, encoder, media, codec, frame, filter};

use std::collections::HashMap;
use std::path::Path;

pub fn decode<P: AsRef<Path>>(path: P) -> Result<format::context::Input, Error> {
  init()?;
  format::input(&path)
}

pub fn get_tags(input: &format::context::Input) -> Option<HashMap<String, String>> {
  let mut metadata = HashMap::new();
  for (k,v) in input.metadata().iter() {
    metadata.insert(k.to_string(), v.to_string());
  }
  if metadata.is_empty() {
    None
  } else {
    Some(metadata)
  }
}
