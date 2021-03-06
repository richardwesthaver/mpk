//! MPK_CODEC -- FFMPEG
extern crate ffmpeg_next as ffmpeg;
use std::collections::HashMap;
use std::path::Path;

pub use ffmpeg::{
  codec, decoder, encoder, filter, format, frame, init, media, packet, rescale, Error,
  Rescale,
};

pub fn decode<P: AsRef<Path>>(path: P) -> Result<format::context::Input, Error> {
  init()?;
  format::input(&path)
}

pub fn get_tags(input: &format::context::Input) -> Option<HashMap<String, String>> {
  let mut metadata = HashMap::new();
  for (k, v) in input.metadata().iter() {
    metadata.insert(k.to_string(), v.to_string());
  }
  if metadata.is_empty() {
    None
  } else {
    Some(metadata)
  }
}

pub fn get_audio_data<P: AsRef<Path>>(
  path: P,
) -> Result<(Vec<Vec<u8>>, format::Sample, u32, u16), Error> {
  let mut ictx = decode(path)?;
  let mut res = vec![];
  let stream = ictx
    .streams()
    .best(media::Type::Audio)
    .expect("could not find best audio stream");
  let stream_id = stream.id();
  let ctx = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
  let mut decoder = ctx.decoder().audio()?;
  for p in ictx.packets() {
    if p.0.id() == stream_id {
      decoder.send_packet(&p.1)?;
      let mut decoded = frame::Audio::empty();
      while decoder.receive_frame(&mut decoded).is_ok() {
        let data = decoded.data(0).to_vec();
        //	let l = decoded.plane_mut::<u8>(0).to_vec();
        //	let r = decoded.plane_mut::<u8>(1).to_vec();
        //	let lr = (l, r);
        res.push(data)
      }
    }
  }
  Ok((res, decoder.format(), decoder.rate(), decoder.channels()))
}

pub fn get_audio_resample<P: AsRef<Path>>(
  path: P,
  format: format::Sample,
) -> Result<(Vec<frame::Audio>, u32, u16), Error> {
  let mut ictx = decode(path)?;
  let mut res = vec![];
  let stream = ictx
    .streams()
    .best(media::Type::Audio)
    .expect("could not find best audio stream");
  let stream_id = stream.id();
  let ctx = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
  let mut decoder = ctx.decoder().audio()?;
  for p in ictx.packets() {
    if p.0.id() == stream_id {
      decoder.send_packet(&p.1)?;
      let mut decoded = frame::Audio::empty();
      while decoder.receive_frame(&mut decoded).is_ok() {
        let mut resampled = frame::Audio::empty();
        let mut resampler =
          decoded.resampler(format, decoded.channel_layout(), decoded.rate())?;
        resampler.run(&decoded, &mut resampled)?;
        //	let l = decoded.plane_mut::<u8>(0).to_vec();
        //	let r = decoded.plane_mut::<u8>(1).to_vec();
        //	let lr = (l, r);
        res.push(resampled)
      }
    }
  }
  Ok((res, decoder.rate(), decoder.channels()))
}

fn filter(
  spec: &str,
  decoder: &codec::decoder::Audio,
  encoder: &codec::encoder::Audio,
) -> Result<filter::Graph, Error> {
  let mut filter = filter::Graph::new();

  let args = format!(
    "time_base={}:sample_rate={}:sample_fmt={}:channel_layout=0x{:x}",
    decoder.time_base(),
    decoder.rate(),
    decoder.format().name(),
    decoder.channel_layout().bits()
  );

  filter.add(&filter::find("abuffer").unwrap(), "in", &args)?;
  filter.add(&filter::find("abuffersink").unwrap(), "out", "")?;

  {
    let mut out = filter.get("out").unwrap();

    out.set_sample_format(encoder.format());
    out.set_channel_layout(encoder.channel_layout());
    out.set_sample_rate(encoder.rate());
  }

  filter.output("in", 0)?.input("out", 0)?.parse(spec)?;
  filter.validate()?;

  println!("{}", filter.dump());

  if let Some(codec) = encoder.codec() {
    if !codec
      .capabilities()
      .contains(ffmpeg::codec::capabilities::Capabilities::VARIABLE_FRAME_SIZE)
    {
      filter
        .get("out")
        .unwrap()
        .sink()
        .set_frame_size(encoder.frame_size());
    }
  }

  Ok(filter)
}

pub struct AudioTranscoder {
  stream: usize,
  filter: filter::Graph,
  decoder: codec::decoder::Audio,
  encoder: codec::encoder::Audio,
  in_time_base: ffmpeg::Rational,
  out_time_base: ffmpeg::Rational,
}

pub fn audio_transcoder<P: AsRef<Path>>(
  ictx: &mut format::context::Input,
  octx: &mut format::context::Output,
  path: &P,
  filter_spec: &str,
) -> Result<AudioTranscoder, Error> {
  let input = ictx
    .streams()
    .best(media::Type::Audio)
    .expect("could not find best audio stream");
  let context = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
  let mut decoder = context.decoder().audio()?;
  let codec = ffmpeg::encoder::find(octx.format().codec(&path, media::Type::Audio))
    .expect("failed to find encoder")
    .audio()?;
  let global = octx
    .format()
    .flags()
    .contains(ffmpeg::format::flag::Flags::GLOBAL_HEADER);

  decoder.set_parameters(input.parameters())?;

  let mut output = octx.add_stream(codec)?;
  let context = ffmpeg::codec::context::Context::from_parameters(output.parameters())?;
  let mut encoder = context.encoder().audio()?;

  let channel_layout = codec
    .channel_layouts()
    .map(|cls| cls.best(decoder.channel_layout().channels()))
    .unwrap_or(ffmpeg::channel_layout::ChannelLayout::STEREO);

  if global {
    encoder.set_flags(ffmpeg::codec::flag::Flags::GLOBAL_HEADER);
  }

  encoder.set_rate(decoder.rate() as i32);
  encoder.set_channel_layout(channel_layout);
  encoder.set_channels(channel_layout.channels());
  encoder.set_format(
    codec
      .formats()
      .expect("unknown supported formats")
      .next()
      .unwrap(),
  );
  encoder.set_bit_rate(decoder.bit_rate());
  encoder.set_max_bit_rate(decoder.max_bit_rate());

  encoder.set_time_base((1, decoder.rate() as i32));
  output.set_time_base((1, decoder.rate() as i32));

  let encoder = encoder.open_as(codec)?;
  output.set_parameters(&encoder);

  let filter = filter(filter_spec, &decoder, &encoder)?;

  let in_time_base = decoder.time_base();
  let out_time_base = output.time_base();

  Ok(AudioTranscoder {
    stream: input.index(),
    filter,
    decoder,
    encoder,
    in_time_base,
    out_time_base,
  })
}

impl AudioTranscoder {
  fn send_frame_to_encoder(&mut self, frame: &ffmpeg::Frame) {
    self.encoder.send_frame(frame).unwrap();
  }

  fn send_eof_to_encoder(&mut self) {
    self.encoder.send_eof().unwrap();
  }

  fn receive_and_process_encoded_packets(
    &mut self,
    octx: &mut format::context::Output,
  ) {
    let mut encoded = ffmpeg::Packet::empty();
    while self.encoder.receive_packet(&mut encoded).is_ok() {
      encoded.set_stream(0);
      encoded.rescale_ts(self.in_time_base, self.out_time_base);
      encoded.write_interleaved(octx).unwrap();
    }
  }

  fn add_frame_to_filter(&mut self, frame: &ffmpeg::Frame) {
    self.filter.get("in").unwrap().source().add(frame).unwrap();
  }

  fn flush_filter(&mut self) {
    self.filter.get("in").unwrap().source().flush().unwrap();
  }

  fn get_and_process_filtered_frames(&mut self, octx: &mut format::context::Output) {
    let mut filtered = frame::Audio::empty();
    while self
      .filter
      .get("out")
      .unwrap()
      .sink()
      .frame(&mut filtered)
      .is_ok()
    {
      self.send_frame_to_encoder(&filtered);
      self.receive_and_process_encoded_packets(octx);
    }
  }

  fn send_packet_to_decoder(&mut self, packet: &ffmpeg::Packet) {
    self.decoder.send_packet(packet).unwrap();
  }

  fn send_eof_to_decoder(&mut self) {
    self.decoder.send_eof().unwrap();
  }

  fn receive_and_process_decoded_frames(&mut self, octx: &mut format::context::Output) {
    let mut decoded = frame::Audio::empty();
    while self.decoder.receive_frame(&mut decoded).is_ok() {
      let timestamp = decoded.timestamp();
      decoded.set_pts(timestamp);
      self.add_frame_to_filter(&decoded);
      self.get_and_process_filtered_frames(octx);
    }
  }
}

pub fn transcode_audio<P: AsRef<Path>>(
  input: P,
  output: P,
  filter: Option<String>,
  seek: Option<i64>,
) {
  init().unwrap();
  let filter = filter.unwrap_or_else(|| "anull".to_owned());
  let mut ictx = format::input(&input).unwrap();
  let mut octx = format::output(&output).unwrap();
  let mut transcoder = audio_transcoder(&mut ictx, &mut octx, &output, &filter)
    .expect("transcoder init failed");

  if let Some(position) = seek {
    // If the position was given in seconds, rescale it to ffmpegs base timebase.
    let position = position.rescale((1, 1), rescale::TIME_BASE);
    // If this seek was embedded in the transcoding loop, a call of `flush()`
    // for every opened buffer after the successful seek would be advisable.
    ictx.seek(position, ..position).unwrap();
  }

  octx.set_metadata(ictx.metadata().to_owned());
  octx.write_header().unwrap();

  for (stream, mut packet) in ictx.packets() {
    if stream.index() == transcoder.stream {
      packet.rescale_ts(stream.time_base(), transcoder.in_time_base);
      transcoder.send_packet_to_decoder(&packet);
      transcoder.receive_and_process_decoded_frames(&mut octx);
    }
  }

  transcoder.send_eof_to_decoder();
  transcoder.receive_and_process_decoded_frames(&mut octx);

  transcoder.flush_filter();
  transcoder.get_and_process_filtered_frames(&mut octx);

  transcoder.send_eof_to_encoder();
  transcoder.receive_and_process_encoded_packets(&mut octx);

  octx.write_trailer().unwrap();
}
