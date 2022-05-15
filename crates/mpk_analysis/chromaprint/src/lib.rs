//! CHROMAPRINT
//!
//! Rust bindings for chromaprint
//!
//! REF: <https://github.com/acoustid/chromaprint>
pub use chromaprint_sys as sys;
use std::ffi::CStr;
use std::{ptr, slice};

pub fn version() -> String {
  String::from_utf8(unsafe { CStr::from_ptr(sys::chromaprint_get_version()).to_bytes().to_vec() }).unwrap()
}

pub fn encode_fingerprint(raw: &[u32], algo: sys::ChromaprintAlgorithm, base64: bool) -> Option<Vec<i8>> {
  let array: *mut i8 = ptr::null_mut();
  let size = 0 as *mut i32;
  let res = unsafe {
    sys::chromaprint_encode_fingerprint(
      raw.as_ptr(),
      raw.len() as i32,
      algo as i32,
      array.cast(),
      size,
      base64 as i32)
  };
  if res == 1 {
    let encoded = unsafe { slice::from_raw_parts(array, size as usize).to_vec() };
    unsafe { sys::chromaprint_dealloc(array as *mut std::ffi::c_void) }
    return Some(encoded);
  } else {
    None
  }
}

pub fn decode_fingerprint(encoded: &[i8], base64: bool) -> Option<(Vec<u32>, sys::ChromaprintAlgorithm)> {
  let array: *mut u32 = ptr::null_mut();
  let size = 0 as *mut i32;
  let algo = 0 as *mut i32;
  let res = unsafe {
    sys::chromaprint_decode_fingerprint(encoded.as_ptr(),
					encoded.len() as i32,
					array.cast(),
					size, algo, base64 as i32)
  };
  if res == 1 {
    let decoded = unsafe { slice::from_raw_parts(array, size as usize).to_vec() };
    unsafe { sys::chromaprint_dealloc(array as *mut std::ffi::c_void) }
    return Some((decoded, algo as sys::ChromaprintAlgorithm));
  }
  None
}

pub struct Chromaprint {
  ctx: *mut sys::ChromaprintContext,
}

impl Chromaprint {
  pub fn new() -> Chromaprint {
    unsafe {
      Chromaprint {
        ctx:  sys::chromaprint_new(sys::ChromaprintAlgorithm_CHROMAPRINT_ALGORITHM_DEFAULT as i32),
      }
    }
  }  
  pub fn algorithm(&self) -> sys::ChromaprintAlgorithm {
    unsafe { sys::chromaprint_get_algorithm(self.ctx) as sys::ChromaprintAlgorithm }
  }

    pub fn start(&mut self, sample_rate: u32, num_channels: u32) -> bool {
        unsafe { sys::chromaprint_start(self.ctx, sample_rate as i32, num_channels as i32) == 1 }
    }

    pub fn feed(&mut self, data: &[i16]) -> bool {
        unsafe { sys::chromaprint_feed(self.ctx, data.as_ptr(), data.len() as i32) == 1 }
    }

    pub fn finish(&mut self) -> bool {
        unsafe { sys::chromaprint_finish(self.ctx) == 1 }
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
  }
}
