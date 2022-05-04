//! MPK_CODEC -- SND
pub use sndfile::{
  Endian, MajorFormat, OpenOptions, ReadOptions, SndFile, SndFileError, SubtypeFormat,
  TagType, WriteOptions,
};
use std::path::Path;

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
