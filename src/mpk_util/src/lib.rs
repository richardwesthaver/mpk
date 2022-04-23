//! MPK_UTIL
pub mod nsm;
pub use indicatif::{ProgressBar, ProgressStyle};

use std::path::{Path, PathBuf};

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
