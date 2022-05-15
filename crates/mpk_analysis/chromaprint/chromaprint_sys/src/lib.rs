#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
  use super::*;
    #[test]
    fn sys_test() {
      unsafe {
	let ctx = chromaprint_new(0);
	let v = chromaprint_get_version();
	let algo = ChromaprintAlgorithm_CHROMAPRINT_ALGORITHM_DEFAULT;
	assert_eq!(algo, 1);
	assert!(!v.is_null());
	chromaprint_free(ctx);
      }
    }
}
