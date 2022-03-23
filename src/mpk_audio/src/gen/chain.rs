use crate::err::{Error, Result};
use rodio::{source::UniformSourceIterator, Decoder};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::str::FromStr;
pub const OT_FILE_HEADER: [u8; 23] = [
  0x46, 0x4F, 0x52, 0x4D, 0x00, 0x00, 0x00, 0x00, 0x44, 0x50, 0x53, 0x31, 0x53, 0x4D,
  0x50, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
];

#[derive(Debug)]
pub enum ChainExt {
  Wav,
  Flac,
  Mp3,
}

impl std::fmt::Display for ChainExt {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      ChainExt::Wav => f.write_str("wav"),
      ChainExt::Flac => f.write_str("flac"),
      ChainExt::Mp3 => f.write_str("mp3"),
    }
  }
}

impl FromStr for ChainExt {
  type Err = Error;
  fn from_str(input: &str) -> Result<ChainExt> {
    match input {
      "wav" => Ok(ChainExt::Wav),
      "flac" => Ok(ChainExt::Flac),
      "mp3" => Ok(ChainExt::Mp3),
      e => Err(Error::BadChainExt(e.to_string())),
    }
  }
}

#[derive(Debug)]
pub struct ChainSlice {
  pub loop_point: u32,
  pub start_point: u32,
  pub length: u32,
}

#[derive(Debug)]
pub struct SampleChain {
  pub slices: Vec<ChainSlice>,
  pub input_files: Vec<PathBuf>,
  pub output_file: PathBuf,
  pub output_ext: ChainExt,
  pub sample_rate: u32,
  pub start_offset: u32,
  pub channels: u16,
  pub max_duration: usize,
  pub tempo: u32,
}

impl Default for SampleChain {
  fn default() -> Self {
    SampleChain {
      slices: vec![],
      input_files: vec![],
      output_file: PathBuf::from("chain"),
      output_ext: ChainExt::Wav,
      sample_rate: 44100,
      start_offset: 0,
      channels: 1,
      max_duration: 0,
      tempo: 120,
    }
  }
}

impl SampleChain {
  pub fn clear(&mut self) {
    self.slices.clear();
    self.start_offset = 0;
  }
  pub fn add_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
    let path = path.as_ref();
    if path.is_file() {
      eprintln!("Adding file to sample_chainer: {:?}", path.display());
      let file = std::fs::File::open(path).unwrap();
      let samps = Decoder::new(BufReader::new(file)).unwrap().count();
      if samps > self.max_duration {
        self.max_duration = samps;
      }
    }
    Ok(())
  }

  pub fn process_file<P: AsRef<Path>>(
    &mut self,
    path: P,
    even_spacing: bool,
  ) -> Result<()> {
    let path = path.as_ref();
    eprintln!("Processing file: {:?}", path);
    let file = std::fs::File::open(path).unwrap();
    //    if input.sample_rate() != self.sample_rate
    //       || input.channels() != self.channels {
    let source = UniformSourceIterator::<_, i16>::new(
      Decoder::new(BufReader::new(file)).unwrap(),
      self.channels,
      self.sample_rate,
    );
    //      }
    let out_file = self.output_file.with_extension(self.output_ext.to_string());
    // Define valid sample format

    let slice_len: u32 = match out_file.is_file() {
      true => {
        let wav_file = hound::WavWriter::append(out_file).unwrap();
        self.write(wav_file, source.collect(), even_spacing)
      }
      false => {
        let spec = hound::WavSpec {
          channels: 1,
          sample_rate: self.sample_rate.clone(),
          bits_per_sample: 16,
          sample_format: hound::SampleFormat::Int,
        };
        let wav_file = hound::WavWriter::create(out_file, spec).unwrap();
        self.write(wav_file, source.collect(), even_spacing)
      }
    };
    let new_slice = ChainSlice {
      start_point: self.start_offset,
      length: slice_len,
      loop_point: slice_len,
    };
    self.slices.push(new_slice);
    self.start_offset += slice_len;
    eprintln!("file processed successfully");
    Ok(())
  }
  pub fn write(
    &mut self,
    mut writer: hound::WavWriter<std::io::BufWriter<std::fs::File>>,
    samples: Vec<i16>,
    even_spacing: bool,
  ) -> u32 {
    match even_spacing {
      true => {
        for i in 0..self.max_duration {
          let mut s_value: i16 = 0;
          if i < samples.len() {
            s_value = samples[i].clone()
          }
          writer.write_sample(s_value).unwrap();
        }
      }
      false => {
        for i in 0..samples.len() {
          writer.write_sample(samples[i].clone()).unwrap();
        }
      }
    }
    writer.finalize().unwrap();
    samples.len() as u32
  }
}
