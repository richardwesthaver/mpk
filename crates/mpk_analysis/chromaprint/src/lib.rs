//! CHROMAPRINT
//!
//! Rust bindings for chromaprint
//!
//! REF: <https://github.com/acoustid/chromaprint>
use std::ffi::CStr;
use std::{os::raw, ptr, slice};

pub use chromaprint_sys as sys;

#[derive(Debug)]
pub enum Error {
  EncodingFailed,
  DecodingFailed,
  HashingFailed,
  InitFailed,
  BadFingerprint,
  BadAlgo,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChromaprintAlgorithm {
  Test1 = 0,
  Test2 = 1,
  Test3 = 2,
  Test4 = 3,
  Test5 = 4,
}

impl Default for ChromaprintAlgorithm {
  fn default() -> ChromaprintAlgorithm {
    ChromaprintAlgorithm::Test2
  }
}

impl From<i32> for ChromaprintAlgorithm {
  fn from(i: i32) -> ChromaprintAlgorithm {
    match i {
      0 => ChromaprintAlgorithm::Test1,
      1 => ChromaprintAlgorithm::Test2,
      2 => ChromaprintAlgorithm::Test3,
      3 => ChromaprintAlgorithm::Test4,
      4 => ChromaprintAlgorithm::Test5,
      // fallback to default
      _ => ChromaprintAlgorithm::Test2,
    }
  }
}

impl TryFrom<*mut i32> for ChromaprintAlgorithm {
  type Error = Error;
  fn try_from(i: *mut i32) -> Result<ChromaprintAlgorithm, Error> {
    if let Some(i) = unsafe { i.as_ref() } {
      match *i {
        0 => Ok(ChromaprintAlgorithm::Test1),
        1 => Ok(ChromaprintAlgorithm::Test2),
        2 => Ok(ChromaprintAlgorithm::Test3),
        3 => Ok(ChromaprintAlgorithm::Test4),
        4 => Ok(ChromaprintAlgorithm::Test5),
        // fallback to default
        _ => Err(Error::BadAlgo),
      }
    } else {
      Err(Error::BadAlgo)
    }
  }
}

impl From<ChromaprintAlgorithm> for raw::c_int {
  fn from(c: ChromaprintAlgorithm) -> raw::c_int {
    match c {
      ChromaprintAlgorithm::Test1 => 0,
      ChromaprintAlgorithm::Test2 => 1,
      ChromaprintAlgorithm::Test3 => 2,
      ChromaprintAlgorithm::Test4 => 3,
      ChromaprintAlgorithm::Test5 => 4,
    }
  }
}

/// Return the current libchromaprint version
pub fn version() -> String {
  String::from_utf8(unsafe {
    CStr::from_ptr(sys::chromaprint_get_version())
      .to_bytes()
      .to_vec()
  })
  .unwrap()
}

/// Encode a raw fingerprint of u32s, returning a Vec<c_char>
pub fn encode_fingerprint(
  raw: &[u32],
  algo: ChromaprintAlgorithm,
  base64: bool,
) -> Result<String, Error> {
  let mut array: *mut raw::c_char = ptr::null_mut();
  let mut size: raw::c_int = 0;
  let res = unsafe {
    sys::chromaprint_encode_fingerprint(
      raw.as_ptr() as *const u32,
      raw.len() as raw::c_int,
      algo.into(),
      &mut array,
      &mut size,
      base64 as raw::c_int,
    )
  };
  if res == 1 {
    let encoded = unsafe {
      &*(slice::from_raw_parts(array, size as usize) as *const [i8] as *const [u8])
    };
    unsafe { sys::chromaprint_dealloc(array as *mut std::ffi::c_void) }
    if let Ok(s) = String::from_utf8(encoded.to_vec()) {
      Ok(s)
    } else {
      Err(Error::EncodingFailed)
    }
  } else {
    Err(Error::EncodingFailed)
  }
}

pub fn decode_fingerprint(
  encoded: &[i8],
  base64: bool,
) -> Result<(Vec<u32>, ChromaprintAlgorithm), Error> {
  let mut array: *mut u32 = ptr::null_mut();
  let mut size: raw::c_int = 0;
  let mut algo: raw::c_int = -1;
  let res = unsafe {
    sys::chromaprint_decode_fingerprint(
      encoded.as_ptr(),
      encoded.len() as i32,
      &mut array,
      &mut size,
      &mut algo,
      base64 as i32,
    )
  };
  if res == 1 {
    let decoded = unsafe { slice::from_raw_parts(array, size as usize).to_vec() };
    unsafe { sys::chromaprint_dealloc(array as *mut std::ffi::c_void) }
    Ok((decoded, algo.into()))
  } else {
    Err(Error::DecodingFailed)
  }
}

pub fn hash_fingerprint(raw: &[u32]) -> Result<u32, Error> {
  let mut hash = 0;
  let res = unsafe {
    sys::chromaprint_hash_fingerprint(raw.as_ptr(), raw.len() as i32, &mut hash)
  };
  if res == 1 {
    Ok(hash)
  } else {
    Err(Error::HashingFailed)
  }
}

pub struct Chromaprint {
  ctx: *mut sys::ChromaprintContext,
}

impl Chromaprint {
  pub fn new() -> Chromaprint {
    unsafe {
      Chromaprint {
        ctx: sys::chromaprint_new(ChromaprintAlgorithm::default().into()),
      }
    }
  }

  // FIXME
  pub fn algorithm(&self) -> ChromaprintAlgorithm {
    unsafe { sys::chromaprint_get_algorithm(self.ctx) }.into()
  }

  pub fn start(&self, sample_rate: u32, num_channels: u32) -> Result<(), Error> {
    if unsafe {
      sys::chromaprint_start(
        self.ctx,
        sample_rate as raw::c_int,
        num_channels as raw::c_int,
      )
    } == 1
    {
      Ok(())
    } else {
      Err(Error::InitFailed)
    }
  }

  pub fn feed(&mut self, data: &[i16]) -> bool {
    unsafe {
      sys::chromaprint_feed(self.ctx, data.as_ptr(), data.len() as raw::c_int) == 1
    }
  }

  pub fn finish(&self) -> bool {
    unsafe { sys::chromaprint_finish(self.ctx) == 1 }
  }

  pub fn fingerprint(&self) -> Result<String, Error> {
    let mut fingerprint: *mut raw::c_char = ptr::null_mut();
    if unsafe { sys::chromaprint_get_fingerprint(self.ctx, &mut fingerprint) } == 1 {
      let ret =
        String::from_utf8(unsafe { CStr::from_ptr(fingerprint) }.to_bytes().to_vec());
      unsafe { sys::chromaprint_dealloc(fingerprint as *mut std::ffi::c_void) }
      ret.map_err(|_| Error::BadFingerprint)
    } else {
      Err(Error::BadFingerprint)
    }
  }

  pub fn raw_fingerprint(&self) -> Result<Vec<u32>, Error> {
    let mut array: *mut u32 = ptr::null_mut();
    let mut size: raw::c_int = 0;
    if unsafe { sys::chromaprint_get_raw_fingerprint(self.ctx, &mut array, &mut size) }
      == 1
    {
      let ret = unsafe { slice::from_raw_parts(array, size as usize) }.to_vec();
      unsafe { sys::chromaprint_dealloc(array as *mut std::ffi::c_void) }
      Ok(ret)
    } else {
      Err(Error::BadFingerprint)
    }
  }

  pub fn size(&self) -> Result<usize, Error> {
    let mut size: std::os::raw::c_int = 0;
    if unsafe { sys::chromaprint_get_raw_fingerprint_size(self.ctx, &mut size) } == 1 {
      Ok(size as usize)
    } else {
      Err(Error::BadFingerprint)
    }
  }
}

impl Drop for Chromaprint {
  fn drop(&mut self) {
    unsafe { sys::chromaprint_free(self.ctx) }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn chromaprint_test() {
    dbg!(version());
    let data: [i16; 10000] = [10101; 10000];
    let mut cp = Chromaprint::new();
    cp.start(44100, 2).unwrap();
    dbg!(cp.algorithm());
    assert!(cp.feed(&data));
    assert!(cp.feed(&data));
    assert!(cp.feed(&data));
    assert!(cp.feed(&data));
    assert!(cp.finish());
    let fp = cp.raw_fingerprint().unwrap();
    let hash = hash_fingerprint(&fp).unwrap();
    dbg!(&fp);
    dbg!(&hash);
    dbg!(cp.size().unwrap());
    //    let encoded = encode_fingerprint(&fp, 1.into(), false).unwrap();
    //    decode_fingerprint(&encoded, false).unwrap();
  }
}
