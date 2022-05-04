//! MPK_CODEC
#[cfg(feature = "ffmpeg")]
pub mod ffmpeg;
#[cfg(feature = "snd")]
pub mod snd;

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn snd_decode_test() {
    let mut snd = snd::decode("../../tests/ch1.wav").unwrap();
    let samplerate = snd.get_samplerate();
    let n_frame = snd.len().unwrap();
    let n_channels = snd.get_channels();
    println!(
      "  Length: {:.2} seconds",
      n_frame as f64 / samplerate as f64
    );
    println!("  Sample rate: {} Hz", samplerate);
    println!("  Channel count: {}", n_channels);
  }
}
