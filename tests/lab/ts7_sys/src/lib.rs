#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let sc: *mut scheme = unsafe{scheme_init_new()};
    unsafe{scheme_init(sc)};
  }
}
