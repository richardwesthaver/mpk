//! sample_chain --- combine multiple wav files and create .ot file with individual slices
use ot::OTWriter;
use std::path::{PathBuf, Path};
use hound::{WavReader, WavWriter, WavSpec, SampleFormat};

fn main() -> std::io::Result<()> {
  ChainWriter::new("sample_chain.wav", "examples/samples").process()
}

struct ChainWriter {
  input_files: Vec<PathBuf>,
  output_file: PathBuf,
  sample_rate: u32,
  offset: u32,
  channels: u16,
  ot_writer: OTWriter,
}

impl ChainWriter {
  fn new<P: AsRef<Path>>(output_file: P, input_files: P) -> Self {
    let output = output_file.as_ref();
    let ot_file = output.with_extension("ot");

    let mut writer = ChainWriter {
      input_files: vec![],
      output_file: output.to_path_buf(),
      sample_rate: 44100,
      offset: 0,
      channels: 1,
      ot_writer: OTWriter::new(ot_file).tempo(120.)
    };

    writer.add_files(input_files).expect("failed to add input files");

    writer
  }

  fn add_files<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
    let path = path.as_ref();

    if path.is_file() {
      println!("adding file: {}", path.display());
      self.input_files.push(path.to_path_buf());
    } else if path.is_dir() {
      for f in std::fs::read_dir(path)? {
	let f = f?;
	let file = f.path();
	if file.is_dir() {
	  self.add_files(file)?
	} else if file.is_file() {
	  self.input_files.push(file.to_path_buf());
	}
      }
    } else {
      return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "files not found"))
    }
    Ok(())
  }

  fn write(
    &mut self,
    mut writer: WavWriter<std::io::BufWriter<std::fs::File>>,
    samples: Vec<i16>
  ) -> u32 {
    for s in &samples {
      writer.write_sample(*s).unwrap();
    }
    writer.finalize().unwrap();
    samples.len() as u32
  }
  
  fn process(&mut self) -> std::io::Result<()> {
    for file in self.input_files.clone() {
      println!("processing file: {}", file.display());

      let mut source = WavReader::open(file).expect("failed to open wav file");

      let slice_len: u32 = match self.output_file.exists() {
	true => {
	  let wav_file = WavWriter::append(&self.output_file).unwrap();
	  self.write(wav_file, source.samples::<i16>().map(|s| s.unwrap()).collect())
	},
	false => {
	  let spec = WavSpec {
	    channels: self.channels,
	    sample_rate: self.sample_rate,
	    bits_per_sample: 16,
	    sample_format: SampleFormat::Int,
	  };
	  let wav_file = WavWriter::create(&self.output_file, spec).unwrap();
	  self.write(wav_file, source.samples::<i16>().map(|s| s.unwrap()).collect())
	}
      };
      self.ot_writer.add_slice(self.offset, self.offset + slice_len, None);
      self.offset += slice_len;
    }

    self.ot_writer.write(self.offset)?;

    Ok(())
  }
}
