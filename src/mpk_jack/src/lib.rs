//! MPK_JACK
//!
//! JACK Audio Connection Kit wrappers for MPK.
pub use jack::Client;
use std::sync::mpsc;
pub fn internal_client(name: &str, stop: mpsc::Receiver<()>) {
  // Create client
  let (client, _status) =
    jack::Client::new(name, jack::ClientOptions::NO_START_SERVER).unwrap();

  let int_client = client
    .load_internal_client("Jack Profiler (rust-jack test)", "profiler", "-c -p -e")
    .expect("Failed to Load Client");

  stop.recv().unwrap();

  let _ = client.unload_internal_client(int_client);
}
