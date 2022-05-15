use std::env;
use std::path::PathBuf;

fn main() {
  println!("cargo:rustc-link-lib=chromaprint");
  let out = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindgen::Builder::default()
    .header("wrapper.h")
    .allowlist_type("Chromaprint.*")
    .allowlist_var("CHROMAPRINT.*")
    .allowlist_function("chromaprint.*")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("failed to generate bindings")
    .write_to_file(out.join("bindings.rs"))
    .expect("failed to write bindings");
}
