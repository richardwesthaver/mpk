//! track_prep --- create .ot files for all tracks in a directory with tempo in file name
use ot::{OTWriter, TrigQuantType};
use hound::WavReader;

fn main() -> std::io::Result<()> {
  let input = "examples/tracks";
  
  // empty file_name since we overwrite it in each iteration of the
  // loop below
  let mut writer = OTWriter::new("")
    .trig_quant_type(TrigQuantType::Pattern);
  
  for f in std::fs::read_dir(input)? {
    let f = f?.path();
    if let Some(ext) = f.extension() {
      match ext.to_str().unwrap() {
	"wav" => {
	  // first 6 characters of a file_name is the tempo:
	  // (140.00_ARTIST_TRACK.wav)
	  let tempo: f32 = f.file_name().unwrap()
	    .to_str().unwrap()[..6]
	    .parse().expect("failed to parse tempo from file name");

	  let len: u32 = WavReader::open(&f).unwrap().samples::<i16>().count() as u32;

	  writer = writer.file_name(f.with_extension("ot"))
	    .tempo(tempo);
	
	  writer.write(len)?;
	},
	_ => (),
      }
    }
  }

  Ok(())
}
