//! ot --- Octatrack metadata file format
//!
//! Based on [https://github.com/KaiDrange/OctaChainer/blob/master/otwriter.h]
use std::io::Write;

pub const FILE_SIZE: usize = 832;
pub const HEADER_BYTES: [u8; 16] = [0x46, 0x4F, 0x52, 0x4D, 0x00, 0x00, 0x00, 0x00, 0x44, 0x50, 0x53, 0x31, 0x53, 0x4D, 0x50, 0x41];
pub const UNKNOWN_BYTES: [u8; 7] = [0x00,0x00,0x00,0x00,0x00,0x02,0x00];

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LoopType {
  Off = 0,
  Loop = 1,
  PingPong = 2,
}

impl TryFrom<u32> for LoopType {
  type Error = std::io::Error;
  fn try_from(n: u32) -> std::io::Result<Self> {
    match n {
      1 => Ok(LoopType::Loop),
      2 => Ok(LoopType::PingPong),
      0 => Ok(LoopType::Off),
      _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid data. valid = [0,1,2]"))
    }
  }
}

impl std::fmt::Display for LoopType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      LoopType::Off => write!(f, "off"),
      LoopType::Loop => write!(f, "loop"),
      LoopType::PingPong => write!(f, "pingpong"),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StretchType {
  Off = 0,
  Normal = 2,
  Beat = 3,
}

impl TryFrom<u32> for StretchType {
  type Error = std::io::Error;
  fn try_from(n: u32) -> std::io::Result<Self> {
    match n {
      2 => Ok(StretchType::Normal),
      3 => Ok(StretchType::Beat),
      0 => Ok(StretchType::Off),
      _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid data. valid = [0,2,3]"))
    }
  }
}

impl std::fmt::Display for StretchType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      StretchType::Off => write!(f, "off"),
      StretchType::Normal => write!(f, "normal"),
      StretchType::Beat => write!(f, "beat"),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TrigQuantType {
  Direct = 0xFF,
  Pattern = 0,
  S1 = 1,
  S2 = 2,
  S3 = 3,
  S4 = 4,
  S6 = 5,
  S8 = 6,
  S12 = 7,
  S16 = 8,
  S24 = 9,
  S32 = 10,
  S48 = 11,
  S64 = 12,
  S96 = 13,
  S128 = 14,
  S192 = 15,
  S256 = 16
}

impl TryFrom<u8> for TrigQuantType {
  type Error = std::io::Error;
  fn try_from(n: u8) -> std::io::Result<Self> {
    match n {
      0 => Ok(TrigQuantType::Pattern),
      1 => Ok(TrigQuantType::S1),
      2 => Ok(TrigQuantType::S2),
      3 => Ok(TrigQuantType::S3),
      4 => Ok(TrigQuantType::S4),
      5 => Ok(TrigQuantType::S6),
      6 => Ok(TrigQuantType::S8),
      7 => Ok(TrigQuantType::S12),
      8 => Ok(TrigQuantType::S16),
      9 => Ok(TrigQuantType::S24),
      10 => Ok(TrigQuantType::S32),
      11 => Ok(TrigQuantType::S48),
      12 => Ok(TrigQuantType::S64),
      13 => Ok(TrigQuantType::S96),
      14 => Ok(TrigQuantType::S128),
      15 => Ok(TrigQuantType::S192),
      16 => Ok(TrigQuantType::S256),
      0xFF => Ok(TrigQuantType::Direct),
      _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid data. valid = [0..16, 0xFF]"))
    }
  }
}

impl std::fmt::Display for TrigQuantType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      TrigQuantType::Direct => write!(f, "direct"),
      TrigQuantType::Pattern => write!(f, "pattern"),
      TrigQuantType::S1 => write!(f, "s1"),
      TrigQuantType::S2 => write!(f, "s2"),
      TrigQuantType::S3 => write!(f, "s3"),
      TrigQuantType::S4 => write!(f, "s4"),
      TrigQuantType::S6 => write!(f, "s6"),
      TrigQuantType::S8 => write!(f, "s8"),
      TrigQuantType::S12 => write!(f, "s12"),
      TrigQuantType::S16 => write!(f, "s16"),
      TrigQuantType::S24 => write!(f, "s24"),
      TrigQuantType::S32 => write!(f, "s32"),
      TrigQuantType::S48 => write!(f, "s48"),
      TrigQuantType::S64 => write!(f, "s64"),
      TrigQuantType::S96 => write!(f, "s96"),
      TrigQuantType::S128 => write!(f, "s128"),
      TrigQuantType::S192 => write!(f, "s192"),
      TrigQuantType::S256 => write!(f, "s256"),
    }
  }
}

fn push_u32(vector: &mut Vec<u8>, num: u32) {
  let array = num.to_le_bytes();
  for i in 0..4 {
    vector.push(array[3 - i]);
  }
}

fn push_u16(vector: &mut Vec<u8>, num: u16) {
  let array = num.to_le_bytes();
  vector.push(array[1]);
  vector.push(array[0]);
}

fn pop_u32(array: &mut [u8]) -> u32 {
  array.reverse();
  u32::from_le_bytes(array.try_into().unwrap())
}

fn pop_u16(array: &mut [u8]) -> u16 {
  u16::from_le_bytes(array.try_into().unwrap())
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Slice {
  start_point: u32,
  end_point: u32,
  loop_point: u32,
}

impl Slice {
  pub fn new(start_point: u32, end_point: u32) -> Self {
    Slice {
      start_point,
      end_point,
      loop_point: 0,
    }
  }

  pub fn loop_point(mut self, loop_point: u32) -> Self {
    self.loop_point = loop_point;
    self
  }

  pub fn len(&self) -> u32 {
    self.end_point - self.start_point
  }

  pub fn to_vec(&self) -> Vec<u8> {
    self.into()
  }
}

impl From<&Slice> for Vec<u8> {
  fn from(slice: &Slice) -> Self {
    let mut vec = vec![];
    push_u32(&mut vec, slice.start_point);
    push_u32(&mut vec, slice.end_point);
    push_u32(&mut vec, slice.loop_point);
    vec
  }
}

impl From<Slice> for Vec<u8> {
  fn from(slice: Slice) -> Self {
    let mut vec = vec![];
    push_u32(&mut vec, slice.start_point);
    push_u32(&mut vec, slice.end_point);
    push_u32(&mut vec, slice.loop_point);
    vec
  }
}

impl From<Vec<u8>> for Slice {
  fn from(mut bytes: Vec<u8>) -> Self {
    Slice {
      start_point: pop_u32(&mut bytes[0..4]),
      end_point: pop_u32(&mut bytes[4..8]),
      loop_point: pop_u32(&mut bytes[8..12]),
    }
  }
}

impl std::fmt::Display for Slice {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "start: {}, end: {}, loop: {}", self.start_point, self.end_point, self.loop_point)
  }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OTData {
  header: [u8; 16],
  unknown: [u8; 7],
  tempo: u32,
  trim_len: u32,
  loop_len: u32,
  stretch: u32,
  r#loop: u32,
  gain: u16,
  quantize: u8,
  trim_start: u32,
  trim_end: u32,
  loop_point: u32,
  slices: [Slice; 64],
  slice_count: u32,
  checksum: u16,
}

impl OTData {
  pub fn new() -> Self {
    OTData {
      header: HEADER_BYTES,
      unknown: UNKNOWN_BYTES,
      tempo: 0,
      trim_len: 0,
      loop_len: 0,
      stretch: 0,
      r#loop: 0,
      gain: 0,
      quantize: 0,
      trim_start: 0,
      trim_end: 0,
      loop_point: 0,
      slices: [Slice::default(); 64],
      slice_count: 0,
      checksum: 0,
    }
  }
  pub fn to_vec(self) -> Vec<u8> {
    self.into()
  }
}

impl From<Vec<u8>> for OTData {
  fn from(mut bytes: Vec<u8>) -> OTData {
    assert_eq!(bytes.len(), FILE_SIZE);
    assert_eq!(bytes[0..16], HEADER_BYTES);
    assert_eq!(bytes[16..23], UNKNOWN_BYTES);

    let mut data = OTData::new();

    data.tempo = pop_u32(&mut bytes[23..27]);
    data.trim_len = pop_u32(&mut bytes[27..31]);
    data.loop_len = pop_u32(&mut bytes[31..35]);
    data.stretch = pop_u32(&mut bytes[35..39]);
    data.r#loop = pop_u32(&mut bytes[39..43]);
    data.gain = pop_u16(&mut bytes[43..45]);
    data.quantize = bytes[45];
    data.trim_start = pop_u32(&mut bytes[46..50]);
    data.trim_end = pop_u32(&mut bytes[50..54]);
    data.loop_point = pop_u32(&mut bytes[54..58]);
    data.slices = bytes[58..826].chunks(12)
      .map(|chunk| Slice::from(chunk.to_vec()))
      .collect::<Vec<Slice>>().try_into().unwrap();
    data.slice_count = pop_u32(&mut bytes[826..830]);
    data.checksum = pop_u16(&mut bytes[830..832]);

    data
  }
}

impl From<OTData> for Vec<u8> {
  fn from(data: OTData) -> Vec<u8> {
    let mut vec = vec![];
    vec.extend(data.header);
    vec.extend(data.unknown);
    push_u32(&mut vec, data.tempo);
    push_u32(&mut vec, data.trim_len);
    push_u32(&mut vec, data.loop_len);
    push_u32(&mut vec, data.stretch);    
    push_u32(&mut vec, data.r#loop);
    push_u16(&mut vec, data.gain);
    vec.push(data.quantize);
    push_u32(&mut vec, data.trim_start);
    push_u32(&mut vec, data.trim_end);
    push_u32(&mut vec, data.loop_point);
    for i in data.slices {
      vec.extend(i.to_vec());
    }
    push_u32(&mut vec, data.slice_count);
    push_u16(&mut vec, data.checksum);
    vec
  }
}

#[derive(Debug, PartialEq)]
pub struct OTWriter {
  total_sample_count: u32,
  file_name: std::path::PathBuf,
  sample_rate: u32,
  tempo: f32,
  gain: u16,
  loop_type: LoopType,
  stretch_type: StretchType,
  trig_quant_type: TrigQuantType,
  slices: Vec<Slice>,
  pub data: OTData,
}

impl OTWriter {
  pub fn new<P: AsRef<std::path::Path>>(file_name: P) -> Self {
    OTWriter {
      total_sample_count: 0,
      file_name: file_name.as_ref().to_path_buf(),
      sample_rate: 0,
      tempo: 0.,
      gain: 0,
      loop_type: LoopType::Off,
      stretch_type: StretchType::Normal,
      trig_quant_type: TrigQuantType::Direct,
      slices: Vec::with_capacity(64),
      data: OTData::new(),
    }
  }
  pub fn total_sample_count(mut self, total_sample_count: u32) -> Self {
    self.total_sample_count = total_sample_count;
    self
  }
  pub fn file_name<P: AsRef<std::path::Path>>(mut self, file_name: P) -> Self {
    self.file_name = file_name.as_ref().to_path_buf();
    self
  }
  pub fn sample_rate(mut self, sample_rate: u32) -> Self {
    self.sample_rate = sample_rate;
    self
  }
  pub fn tempo(mut self, tempo: f32) -> Self {
    self.tempo = tempo;
    self
  }
  pub fn gain(mut self, gain: u16) -> Self {
    self.gain = gain;
    self
  }
  pub fn loop_type(mut self, loop_type: LoopType) -> Self {
    self.loop_type = loop_type.into();
    self
  }
  pub fn stretch_type(mut self, stretch_type: StretchType) -> Self {
    self.stretch_type = stretch_type.into();
    self
  }
  pub fn trig_quant_type(mut self, trig_quant_type: TrigQuantType) -> Self {
    self.trig_quant_type = trig_quant_type.into();
    self
  }
  pub fn slices(mut self, slices: Vec<Slice>) -> Self {
    self.slices = slices;
    self
  }
  pub fn data(mut self, data: OTData) -> Self {
    self.data = data;
    self
  }

  pub fn reset<P: AsRef<std::path::Path>>(self, file_name: P) -> Self {
    self.file_name(file_name)
      .total_sample_count(0)
      .sample_rate(0)
      .tempo(0.)
      .gain(0)
      .loop_type(0.try_into().unwrap())
      .stretch_type(0.try_into().unwrap())
      .trig_quant_type(0.try_into().unwrap())
      .slices(vec![])
      .data(OTData::new())
  }

  pub fn from_file<P: AsRef<std::path::Path>>(file: P, sample_rate: u32) -> Self {
    let contents = std::fs::read(&file).unwrap();
    let data = OTData::from(contents);
    OTWriter::new(file)
      .data(data)
      .sample_rate(sample_rate)
      .tempo(data.tempo as f32/24.)
      .gain(data.gain.to_be() - 48)
      .loop_type(data.r#loop.try_into().unwrap())
      .stretch_type(data.stretch.try_into().unwrap())
      .trig_quant_type(data.quantize.try_into().unwrap())
  }

  pub fn write(&mut self, total_samples: u32) -> std::io::Result<()> {
    self.total_sample_count = total_samples;
    self.update_data();
    // write data buffer only if file_name doesn't exist, else return error
    if !self.file_name.exists() {
      let mut file = std::fs::File::create(&self.file_name)?;
      file.write_all(&self.data.to_vec())
    } else {
      Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, format!("file already exists: {}", self.file_name.display())))
    }
  }

  pub fn add_slice(&mut self, start_point: u32, end_point: u32, loop_point: Option<u32>) {
    let slice = Slice {
      start_point,
      end_point,
      loop_point: if let Some(n) = loop_point { n } else { 0xFF }
    };

    self.slices.push(slice);
  }

  pub fn update_data(&mut self) {
    // tempo * 6 * 4
    self.data.tempo = (self.tempo * 6. * 4.) as u32;

    // 25 * ((tempo*s_count)/(s_rate*60) + 0.5)
    let bars: f32 = ((self.tempo as f32 * self.total_sample_count as f32) / (self.sample_rate * 60) as f32 + 0.5) * 25.;

    self.data.trim_len = bars as u32;
    self.data.loop_len = bars as u32;
    self.data.stretch = self.stretch_type as u32;
    self.data.r#loop = self.loop_type as u32;
    // gain + 48
    self.data.gain = (self.gain + 48) as u16;
    self.data.quantize = self.trig_quant_type as u8;
    self.data.slice_count = self.slices.len() as u32;
    self.data.trim_start = 0;
    self.data.trim_end = self.total_sample_count;
    self.data.loop_point = 0;
    let mut temp_slices = self.slices.clone();
    temp_slices.resize(64, Slice::new(0,0));
    self.data.slices = temp_slices.try_into().unwrap();
    self.set_checksum();
  }

  pub fn set_checksum(&mut self) {
    let mut val: u16 = 0;
    let data: Vec<u8> = self.data.into();
    // sum all bytes except for header and checksum (first 16, last 2)
    for i in 16..(data.len()-2) {
      val += data[i] as u16;
    }
    self.data.checksum = val;
  }
}

impl std::fmt::Display for OTWriter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "file_name: {}", self.file_name.display())?;
    writeln!(f, "total_sample_count: {}", self.total_sample_count)?;
    writeln!(f, "sample_rate:: {}", self.sample_rate)?;
    writeln!(f, "tempo: {}", self.tempo)?;
    writeln!(f, "gain: {}", self.gain)?;
    writeln!(f, "loop_type: {}", self.loop_type)?;
    writeln!(f, "stretch_type: {}", self.stretch_type)?;
    writeln!(f, "trig_quant_type: {}", self.trig_quant_type)?;
    writeln!(f, "slices: [")?;
    for slice in &self.slices {
      writeln!(f, "{}", slice)?;
    }
    writeln!(f, "]")?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
      let path = std::path::Path::new("test.ot");
      if path.exists() {
	std::fs::remove_file(path).unwrap(); // cleanup last run
      }

      // init writer
      let mut writer = OTWriter::new("")
	.file_name(path)
       	.sample_rate(44100)
	.tempo(200.)
	.gain(12)
	.loop_type(LoopType::Off)
	.stretch_type(StretchType::Normal)
	.trig_quant_type(TrigQuantType::Direct);
	
      // add a slice given start,end
      writer.add_slice(1010,2020, None);

      // generate test.ot
      assert!(writer.write(100).is_ok());
      // keep fn param same as previous to prevent eq check errors
      assert!(writer.write(100).is_err()); // don't overwrite existing file

      // decode .ot file and confirm it's the same
      let contents = std::fs::read(path).unwrap();
      let data = OTData::from(contents);
      assert_eq!(data, writer.data); 
      
      let mut other = OTWriter::from_file(path, 44100).total_sample_count(100);
      other.add_slice(1010,2020,None);
      assert_eq!(other, writer);
      // debug
      dbg!(data);
      dbg!(writer.data);
    }
}
