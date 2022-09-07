//! decode --- decode .ot file and print metadata
use ot::OTWriter;
use hound::WavReader;

fn main() {
//  let file = "examples/tracks/164.75_Paradox_Deep Sleep.ot";
  let file = "sample_chain.ot";
  let decoded = OTWriter::from_file(file, 44100);
  println!("{}", decoded);
}
