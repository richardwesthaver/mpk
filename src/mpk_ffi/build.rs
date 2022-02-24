use std::env;
use std::path::PathBuf;

fn main() {
  let crate_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var is not defined").into();
  let config = cbindgen::Config::from_file("cbindgen.toml").expect("Unable to find cbindgen.toml configuration file");

  cbindgen::generate_with_config(&crate_dir, config)
    .unwrap()
    .write_to_file(crate_dir.join("mpk_ffi.h"));
}
