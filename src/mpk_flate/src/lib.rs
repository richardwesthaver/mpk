//! MPK_FLATE
//!
//! ZSTD and TAR utilities.
use std::{fs, io, path::Path};

/// Level of compression data should be compressed with.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Level {
  /// Fastest quality of compression, usually produces bigger size.
  Fastest,
  /// Best quality of compression, usually produces the smallest size.
  Best,
  /// Default quality of compression defined by the selected compression
  /// algorithm.
  Default,
  /// Precise quality based on the underlying compression algorithms'
  /// qualities. The interpretation of this depends on the algorithm chosen
  /// and the specific implementation backing it.
  /// Qualities are implicitly clamped to the algorithm's maximum.
  Precise(u8),
}

impl Level {
  fn into_zstd(self) -> i32 {
    match self {
      Self::Fastest => 1,
      Self::Best => 21,
      Self::Precise(quality) => quality.min(21) as i32,
      Self::Default => 0,
    }
  }
}

impl From<u8> for Level {
  fn from(n: u8) -> Self {
    match n {
      0 => Self::Default,
      1 => Self::Fastest,
      2 => Self::Best,
      n => Self::Precise(n),
    }
  }
}

/// Pack a SRC directory, and return a compressed archive at DST.
pub fn pack<P: AsRef<Path>>(src: P, dst: P, level: Option<Level>) {
  let mut tar = tar::Builder::new(Vec::new());
  let src = src.as_ref();
  let parent = src.parent().unwrap();
  let art = src.strip_prefix(parent).unwrap();
  tar.append_dir_all(art, src).unwrap();

  let tar = tar.into_inner().unwrap();
  let dst = dst.as_ref();
  let file = fs::File::create(dst).expect("failed to create output path");
  zstd::stream::copy_encode(
    &tar[..],
    file,
    level.unwrap_or(Level::Default).into_zstd(),
  )
  .unwrap();
}

/// unpack a tar.zst compressed archive or zst file
pub fn unpack<P: AsRef<Path>>(src: P, dst: P) {
  let src = src.as_ref();
  let input = fs::File::open(src).expect("failed to open input");
  let mut buff = Vec::new();
  zstd::stream::copy_decode(input, &mut buff).unwrap();
  if tar::Archive::new(&buff[..]).entries().is_ok() {
    tar::Archive::new(&buff[..]).unpack(dst).unwrap();
  } else {
    decompress(src).unwrap();
  }
}

/// unpack a tar.zst compressed archive, removing the source file before
/// returning
pub fn unpack_replace<P: AsRef<Path>>(src: P, dst: P) {
  unpack(&src, &dst);
  fs::remove_file(src).expect("could not remove source package");
}

/// compress a file with zstd
pub fn compress<P: AsRef<Path>>(src: P, dst: P) -> io::Result<()> {
  let mut file = fs::File::open(&src)?;
  let mut encoder = {
    let target = fs::File::create(dst.as_ref())?;
    zstd::Encoder::new(target, 22)?
  };
  io::copy(&mut file, &mut encoder)?;
  encoder.finish()?;
  Ok(())
}

/// decompress a zst file into the current directory
pub fn decompress<P: AsRef<Path>>(source: P) -> io::Result<()> {
  let mut decoder = {
    let file = fs::File::open(&source)?;
    zstd::Decoder::new(file)?
  };
  let mut target = fs::File::create(source.as_ref().to_str().unwrap())?;
  io::copy(&mut decoder, &mut target)?;
  decoder.finish();
  Ok(())
}

#[test]
fn flate_test() {
  let dir_path = Path::new("pack_test");

  std::fs::create_dir(&dir_path).unwrap();

  for i in 0..10 {
    std::fs::File::create(&dir_path.join(format!("{}.test", i))).unwrap();
  }

  pack(&dir_path, &Path::new("pack_test.tar.zst"), None);
  unpack("pack_test.tar.zst", "pack_test");
  unpack_replace("pack_test.tar.zst", "pack_test");

  std::fs::remove_dir_all(dir_path).unwrap();
}
