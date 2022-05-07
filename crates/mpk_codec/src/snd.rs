//! MPK_CODEC -- SND
use std::collections::HashMap;
use std::path::Path;

pub use sndfile::{
  Endian, MajorFormat, OpenOptions, ReadOptions, SndFile, SndFileError, SubtypeFormat,
  TagType, WriteOptions,
};

pub fn decode<P: AsRef<Path>>(path: P) -> Result<SndFile, SndFileError> {
  OpenOptions::ReadOnly(ReadOptions::Auto).from_path(path)
}

pub fn encode<P: AsRef<Path>>(
  path: P,
  major: MajorFormat,
  minor: SubtypeFormat,
  sr: usize,
  channels: usize,
) -> Result<SndFile, SndFileError> {
  OpenOptions::WriteOnly(WriteOptions::new(major, minor, Endian::File, sr, channels))
    .from_path(path)
}

pub fn get_tags(input: &SndFile) -> Option<HashMap<String, String>> {
  let mut metadata = HashMap::new();
  let tags = [
    ("title", TagType::Title),
    ("artist", TagType::Artist),
    ("album", TagType::Album),
    ("track_number", TagType::Tracknumber),
    ("genre", TagType::Genre),
    ("date", TagType::Date),
    ("comment", TagType::Comment),
  ];
  for i in tags {
    if let Some(t) = input.get_tag(i.1) {
      metadata.insert(i.0.to_string(), t);
    }
  }
  if metadata.is_empty() {
    None
  } else {
    Some(metadata)
  }
}
