use crate::Result;
use mpk_audio::gen::SampleChain;
use std::io::Write;

pub const OT_FILE_HEADER: [u8; 23] = [
  0x46, 0x4F, 0x52, 0x4D, 0x00, 0x00, 0x00, 0x00, 0x44, 0x50, 0x53, 0x31, 0x53, 0x4D,
  0x50, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
];

fn push_u32(mut vector: Vec<u8>, num: u32) -> Vec<u8> {
  let array = num.to_le_bytes();
  for i in 0..4 {
    vector.push(array[3 - i]);
  }
  vector
}

fn push_u16(mut vector: Vec<u8>, num: u16) -> Vec<u8> {
  let array = num.to_le_bytes();
  vector.push(array[1]);
  vector.push(array[0]);
  vector
}

pub fn generate_ot_file(chain: &mut SampleChain) -> Result<()> {
  eprintln!("generating .ot file...");

  let mut data: Vec<u8> = OT_FILE_HEADER.to_vec();
  let tempo = chain.tempo * 6 * 4;

  let mut total_samples: u32 = 0;
  for i in 0..chain.slices.len() {
    total_samples += chain.slices[i].length;
  }

  let bars_mult: f32 =
    (124.0 * total_samples as f32) / (chain.sample_rate * 60) as f32 + 0.5;
  let bars: u32 = bars_mult as u32 * 25;

  // Add data to the .ot buffer
  data = push_u32(data, tempo); // Tempo
  data = push_u32(data, bars.clone()); // Trimlen
  data = push_u32(data, bars.clone()); // loopLen
  data = push_u32(data, 0); // Stretch
  data = push_u32(data, 0); // Loop
  data = push_u16(data, 48); // Gain
  data.push(255); // Quantize
  data = push_u32(data, 0); // trimStart
  data = push_u32(data, total_samples.clone()); // trimEnd
  data = push_u32(data, 0); // loopPoint

  // Add data for each of the slices
  for i in 0..64 {
    if i < chain.slices.len() {
      data = push_u32(data, chain.slices[i].start_point);
      data = push_u32(data, chain.slices[i].start_point + chain.slices[i].length);
      data = push_u32(data, chain.slices[i].loop_point);
    } else {
      data = push_u32(data, 0);
      data = push_u32(data, 0);
      data = push_u32(data, 0);
    }
  }

  data = push_u32(data, chain.slices.len() as u32); // slice count

  let mut checksum: u16 = 0;

  // Checksum formula (basically add all values except the data)
  let len = data.len();
  for i in 16..len {
    checksum += data[i] as u16;
  }

  data = push_u16(data, checksum); // Push Checksum

  let file_name = chain.output_file.with_extension("ot");

  if file_name.is_file() {
    std::fs::remove_file(&file_name).unwrap();
  };

  // Create .ot file and write the data buffer
  let mut buf = std::fs::File::create(file_name).unwrap();
  buf.write_all(&data).unwrap();
  Ok(())
}
