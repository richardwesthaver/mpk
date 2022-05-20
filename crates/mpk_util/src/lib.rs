//! MPK_UTIL
pub mod nsm;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub use indicatif::{ProgressBar, ProgressStyle};

/// utility function to expand `~` in PATH.
pub fn expand_tilde<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
  let p = path.as_ref();
  if !p.starts_with("~") {
    return Some(p.to_path_buf());
  }
  if p == Path::new("~") {
    return dirs::home_dir();
  }
  dirs::home_dir().map(|mut h| {
    if h == Path::new("/") {
      p.strip_prefix("~").unwrap().to_path_buf()
    } else {
      h.push(p.strip_prefix("~/").unwrap());
      h
    }
  })
}

/// OS-specific browser command. supports Win/Mac/Linux
pub fn open_browser(url: &str) {
  if cfg!(target_os = "windows") {
    // https://stackoverflow.com/a/49115945
    std::process::Command::new("rundll32.exe")
      .args(&["url.dll,FileProtocolHandler", url])
      .status()
      .expect("failed to open file");
  } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
    // https://dwheeler.com/essays/open-files-urls.html
    #[cfg(target_os = "macos")]
    let cmd = "open";
    #[cfg(target_os = "linux")]
    let cmd = "xdg-open";

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
      std::process::Command::new(cmd)
        .arg(url)
        .status()
        .expect("failed to open URL");
    }
  } else {
    unimplemented!() // ignore others
  }
}

pub fn timestamp() -> u64 {
  std::time::SystemTime::now()
    .duration_since(std::time::SystemTime::UNIX_EPOCH)
    .expect("SystemTime is before UNIX_EPOCH?")
    .as_secs()
}

pub fn timestamp_nanos() -> u128 {
  std::time::SystemTime::now()
    .duration_since(std::time::SystemTime::UNIX_EPOCH)
    .expect("SystemTime is before UNIX_EPOCH?")
    .as_nanos()
}

/// format a size in bytes to a human-readable format.
pub fn format_byte_size(b: u64) -> String {
  let b: f32 = b as f32;
  let i: f32 = 1024_f32;
  if b < i {
    format!("{}", b)
  } else if b < i * i {
    format!("{}{}", b / i, "kb")
  } else if b < i * i * i {
    format!("{}{}", b / (i * i), "mb")
  } else {
    format!("{}{}", b / (i * i * i), "gb")
  }
}

/// Walk a directory PATH, applying function WALKER to each file and
/// collecting results in a vec.
pub fn walk_dir<P: AsRef<Path>, T: Clone>(
  path: P,
  walker: fn(PathBuf) -> Option<T>,
  coll: &mut Vec<T>,
) -> Result<(), io::Error> {
  let path = path.as_ref();
  if path.is_dir() {
    for elt in fs::read_dir(path)? {
      let elt = elt?;
      let p = elt.path();
      if p.is_dir() {
        walk_dir(p, walker, coll)?;
      } else if p.is_file() {
        if let Some(t) = walker(p.to_path_buf()) {
          coll.push(t)
        }
      }
    }
  } else if path.is_file() {
    if let Some(t) = walker(path.to_path_buf()) {
      coll.push(t)
    }
  }
  Ok(())
}

/// Check if a slice of bytes is full of zeroes. Slightly optimized by
/// aligning to u128.
pub fn is_zeroes(buf: &[u8]) -> bool {
  let (prefix, aligned, suffix) = unsafe { buf.align_to::<u128>() };
  prefix.iter().all(|&x| x == 0)
    && suffix.iter().all(|&x| x == 0)
    && aligned.iter().all(|&x| x == 0)
}
