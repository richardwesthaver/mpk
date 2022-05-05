//! MPK FFI
//!
//! This crate provides FFI-safe bindings for MPK. The cdylib
//! generated from this crate can be used from other C-compatible
//! languages as you see fit.
//!
//! Cbindgen is used in the build.rs script to generate a C header
//! file ('mpk_ffi.h') which is also compatible with C++. This header
//! is in turn utilized by the Python package cffi in build.py to
//! generate Python-compatible bindings (_mpk.c, _mpk.o, and
//! _mpk.cpython-*.so). All of these files can be found in the build
//! directory at the project root after executing 'nim build'.
//!
//! The Python bindings are required by MPK_EXTRACTOR so if you plan
//! to work with the files mpk_analysis/{mpk_extract.py,
//! mpk_essentia/{extract.py lib.py}} directly, be sure to build the
//! project first. When the `dev` flag is defined (default) the Python
//! bindings will be automatically copied to the appropriate
//! directory.
#![allow(clippy::missing_safety_doc)]
use libc::{c_char, size_t};
use mpk_config::{Config, DbConfig, FsConfig, JackConfig};
use mpk_hash::Checksum;
use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::slice;
use std::path::Path;

/// An array of bytes with a fixed length. Represents a BLAKE3 hash.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CChecksum {
  pub ptr: *const u8,
  pub len: size_t,
}

impl From<CChecksum> for Checksum {
  fn from(c: CChecksum) -> Self {
    let b = unsafe {
      assert!(!c.ptr.is_null());
      slice::from_raw_parts(c.ptr, c.len)
    };
    Checksum::hash(b)
  }
}

impl From<Checksum> for CChecksum {
  fn from(r: Checksum) -> Self {
    let b = r.0.as_bytes();
    Self {
      ptr: b.as_ptr(),
      len: b.len(),
    }
  }
}

/// Build a Blake3 Checksum from array BYTES of LEN.
#[no_mangle]
pub extern "C" fn mpk_checksum_new(ptr: *const u8, len: size_t) -> *mut CChecksum {
  let cs = CChecksum { ptr, len };
  Box::into_raw(Box::new(cs))
}

/// Build a Blake3 Checksum from PATH given as c_char[].
#[no_mangle]
pub unsafe extern "C" fn mpk_checksum_path(path: *const c_char) -> *mut CChecksum {
  assert!(!path.is_null());
  let p = CStr::from_ptr(path).to_str().unwrap();
  let cs = Checksum::from_path(p);
  Box::into_raw(Box::new(cs.into()))
}

#[no_mangle]
pub unsafe extern "C" fn mpk_checksum_free(ptr: *mut Checksum) {
  if ptr.is_null() {
    return;
  }
  Box::from_raw(ptr);
}

/// Build a new Config from inner configs FS, DB, and JACK. Returns a
/// mutable pointer.
#[no_mangle]
pub unsafe extern "C" fn mpk_config_new(
  fs: *mut FsConfig,
  db: *mut DbConfig,
  jack: *mut JackConfig,
) -> *mut Config {
  if !fs.is_null() | !db.is_null() | !jack.is_null() {
    let fs = &*fs;
    let db = &*db;
    let jack = &*jack;
    Box::into_raw(Box::new(
      Config::new(fs.to_owned(), db.to_owned(), jack.to_owned()).unwrap(),
    ))
  } else {
    Box::into_raw(Box::new(Config::default()))
  }
}

/// Drop a Config
#[no_mangle]
pub unsafe extern "C" fn mpk_config_free(ptr: *mut Config) {
  if ptr.is_null() {
    return;
  }
  Box::from_raw(ptr);
}

/// Load a Config from PATH. Returns mutable pointer to Config.
#[no_mangle]
pub unsafe extern "C" fn mpk_config_load(path: *const c_char) -> *mut Config {
  assert!(!path.is_null());
  let cstr = CStr::from_ptr(path);
  let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));

  Box::into_raw(Box::new(Config::load(p).unwrap()))
}

/// Write a Config CFG to PATH.
#[no_mangle]
pub unsafe extern "C" fn mpk_config_write(cfg: *const Config, path: *const c_char) {
  assert!(!path.is_null());
  let cstr = CStr::from_ptr(path);
  let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));
  let cfg = &*cfg;
  cfg.write(p).unwrap()
}

/// Build a Config CFG
#[no_mangle]
pub unsafe extern "C" fn mpk_config_build(cfg: *const Config) {
  (&*cfg).build().unwrap()
}

/// Build a FsConfig from ROOT. Returns a mutable pointer to FsConfig.
#[no_mangle]
pub unsafe extern "C" fn mpk_fs_config_new(root: *const c_char) -> *mut FsConfig {
  if !root.is_null() {
    let cstr = CStr::from_ptr(root);
    let r: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));

    Box::into_raw(Box::new(FsConfig::new(r).unwrap()))
  } else {
    Box::into_raw(Box::new(FsConfig::default()))
  }
}

/// Drop a FsConfig
#[no_mangle]
pub unsafe extern "C" fn mpk_fs_config_free(ptr: *mut FsConfig) {
  if ptr.is_null() {
    return;
  }
  Box::from_raw(ptr);
}

/// Get a PATH from FsConfig given CFG of type Config. Returns mutable
/// char pointer.
#[no_mangle]
pub unsafe extern "C" fn mpk_fs_config_get_path(
  cfg: *const Config,
  path: *const c_char,
) -> *mut c_char {
  assert!(!path.is_null());
  let p = CStr::from_ptr(path).to_str().unwrap();
  let cfg = &*cfg;
  let res = cfg.fs.get_path(p).unwrap();
  CString::new(res.as_os_str().as_bytes()).unwrap().into_raw()
}

/// Build a DbConfig. Returns mutable pointer to DbConfig.
#[no_mangle]
pub extern "C" fn mpk_db_config_new() -> *mut DbConfig {
  Box::into_raw(Box::new(DbConfig::default()))
}

/// Drop a DbConfig
#[no_mangle]
pub unsafe extern "C" fn mpk_db_config_free(ptr: *mut DbConfig) {
  if ptr.is_null() {
    return;
  }
  Box::from_raw(ptr);
}

/// Get the DbConfig path from CFG of type Config. Returns a mutable
/// char pointer.
#[no_mangle]
pub unsafe extern "C" fn mpk_db_config_path(cfg: *const Config) -> *mut c_char {
  let cfg = &*cfg;
  let res = &cfg.db.path;
  CString::new(res.as_os_str().as_bytes()).unwrap().into_raw()
}

/// Build JackConfig. Returns mutable JackConfig pointer.
#[no_mangle]
pub extern "C" fn mpk_jack_config_new() -> *mut JackConfig {
  Box::into_raw(Box::new(JackConfig::new().unwrap()))
}

/// Drop a JackConfig.
#[no_mangle]
pub unsafe extern "C" fn mpk_jack_config_free(ptr: *mut JackConfig) {
  if ptr.is_null() {
    return;
  }
  Box::from_raw(ptr);
}
