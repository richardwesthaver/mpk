//! MPK_HTTP
pub use reqwest::Client;

mod err;
pub use err::{Error, Result};

pub mod freesound;
pub mod musicbrainz;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
