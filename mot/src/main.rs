//! mot --- create Octatrack metadata files and handle transfers to
//! device in USB mode.

use ot::{OTWriter, TrigQuantType, StretchType};
use hound::WavReader;
use std::path::Path;
use std::{fs, env, io};

fn auto_gen_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
  let mut writer = OTWriter::new("")
    .trig_quant_type(TrigQuantType::Pattern);

  let path = path.as_ref();
  if path.is_file() {
    if let Some(ext) = &path.extension() {
      match ext.to_str().unwrap() {
	"wav" => {
	  if path.file_name().unwrap().to_str().unwrap().starts_with("._") {
	    println!("skipping temp_file: {}", path.display());
	  } else if !path.with_extension("ot").exists() {
	    println!("generating ot for file: {}", path.display());
	    let mut stretch_type = StretchType::Normal;
	    let mut tempo: f32 = path.file_name().unwrap().to_str().unwrap()[..5]
	      .parse().expect(format!("failed to parse tempo from file: {}", path.display()).as_str());
	    // tempo of 0.00 means this is a freerun track. set the
	    // tempo to 120 and turn off timestretch.
	    if tempo.eq(&0.) {
	      tempo = 120.;
	      stretch_type = StretchType::Off;
	    }
	    let mut wav = WavReader::open(&path).unwrap();
	    let len: u32 = wav.samples::<i16>().count() as u32;
	    let sample_rate = wav.spec().sample_rate;
	    writer = writer.file_name(path.with_extension("ot"))
	      .tempo(tempo)
	      .sample_rate(sample_rate)
	      .stretch_type(stretch_type);

	    writer.write(len)?;
	  }
	},
	_ => (),
      }
    }
  } else if path.is_dir() {
    for f in fs::read_dir(path)? {
      let f = f?;
      let file = f.path();
      auto_gen_dir(file)?
    }
  } else {
      return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "files not found"))
  }
  Ok(())
}

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();
  if let Some(path) = args.iter().skip(1).next() {
    auto_gen_dir(path)
  } else {
    println!("mot PATH");
    Ok(())
  }
}
