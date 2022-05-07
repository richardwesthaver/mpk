use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

pub use rodio::cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
pub use rodio::cpal::{self, Device, Devices};
use rodio::cpal::{available_hosts, host_from_id, ALL_HOSTS};

mod err;
pub mod gen;
pub use err::{Error, Result};

pub fn info() {
  println!("supported hosts: {:?}", ALL_HOSTS);
  let available_hosts = available_hosts();
  println!("available hosts: {:?}", available_hosts);
  for h in available_hosts {
    println!("{}", h.name());
    let host = host_from_id(h).unwrap();
    let default_in = host.default_input_device().map(|e| e.name().unwrap());
    let default_out = host.default_output_device().map(|e| e.name().unwrap());
    println!("  Default Input Device:\n    {:?}", default_in.unwrap());
    println!("  Default Output Device:\n    {:?}", default_out.unwrap());
    let devices = host.devices().unwrap();
    println!("  Devices: ");
    for (device_index, device) in devices.enumerate() {
      println!("  {}. \"{}\"", device_index + 1, device.name().unwrap());

      // Input configs
      if let Ok(conf) = device.default_input_config() {
        println!("    Default input stream config:\n      {:?}", conf);
      }

      // Output configs
      if let Ok(conf) = device.default_output_config() {
        println!("    Default output stream config:\n      {:?}", conf);
      }
    }
  }
}

pub fn play<P: AsRef<Path>>(
  path: P,
  device: &Option<Device>,
  vol: Option<f32>,
  speed: Option<f32>,
  pause: Receiver<bool>,
) {
  let (_stream, handle) = if let Some(d) = device {
    rodio::OutputStream::try_from_device(d).unwrap()
  } else {
    rodio::OutputStream::try_default().unwrap()
  };
  let sink = rodio::Sink::try_new(&handle).unwrap();
  if let Some(v) = vol {
    sink.set_volume(v)
  }
  if let Some(s) = speed {
    sink.set_speed(s)
  }
  let file = std::fs::File::open(path).unwrap();
  sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
  while !sink.empty() {
    match sink.is_paused() {
      true => {
        if !pause.recv().unwrap() {
          sink.play();
        }
      }
      false => {
        if pause
          .recv_timeout(std::time::Duration::from_millis(500))
          .is_ok()
        {
          sink.pause()
        } else {
          continue;
        }
      }
    }
  }
}

pub fn device_from_str(s: &str) -> Option<Device> {
  let mut dev = None;
  for i in available_hosts() {
    for h in host_from_id(i).unwrap().devices().unwrap() {
      if s == h.name().unwrap() {
        dev = Some(h);
      }
    }
  }
  dev
}

pub fn pause_controller_cli() -> Receiver<bool> {
  let (tx, rx) = channel();
  println!("Press enter to pause, C-c to quit...");
  thread::spawn(move || {
    let mut pause = false;
    let mut input = String::new();
    loop {
      io::stdin().read_line(&mut input).ok();
      if pause {
        tx.send(false).unwrap();
        pause = false;
      } else if !pause {
        tx.send(true).unwrap();
        pause = true;
      }
    }
  });
  rx
}

pub fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
  match format {
    cpal::SampleFormat::U16 => hound::SampleFormat::Int,
    cpal::SampleFormat::I16 => hound::SampleFormat::Int,
    cpal::SampleFormat::F32 => hound::SampleFormat::Float,
  }
}

pub fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
  hound::WavSpec {
    channels: config.channels() as _,
    sample_rate: config.sample_rate().0 as _,
    bits_per_sample: (config.sample_format().sample_size() * 8) as _,
    sample_format: sample_format(config.sample_format()),
  }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
  T: cpal::Sample,
  U: cpal::Sample + hound::Sample,
{
  if let Ok(mut guard) = writer.try_lock() {
    if let Some(writer) = guard.as_mut() {
      for &sample in input.iter() {
        let sample: U = cpal::Sample::from(&sample);
        writer.write_sample(sample).ok();
      }
    }
  }
}

pub fn record<P: AsRef<Path>>(
  device: Option<Device>,
  output: P,
  stop: Receiver<bool>,
) -> Result<()> {
  let dev = if let Some(d) = device {
    d
  } else {
    cpal::default_host().default_input_device().unwrap()
  };
  let cfg = dev.default_input_config().unwrap();

  let spec = wav_spec_from_config(&cfg);
  let writer = hound::WavWriter::create(output, spec).unwrap();
  let writer = Arc::new(Mutex::new(Some(writer)));

  let writer_2 = writer.clone();

  let err_fn = move |err| {
    eprintln!("an error occurred on stream: {}", err);
  };
  let stream = match cfg.sample_format() {
    cpal::SampleFormat::F32 => dev
      .build_input_stream(
        &cfg.into(),
        move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
        err_fn,
      )
      .unwrap(),
    cpal::SampleFormat::I16 => dev
      .build_input_stream(
        &cfg.into(),
        move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
        err_fn,
      )
      .unwrap(),
    cpal::SampleFormat::U16 => dev
      .build_input_stream(
        &cfg.into(),
        move |data, _: &_| write_input_data::<u16, i16>(data, &writer_2),
        err_fn,
      )
      .unwrap(),
  };

  stream.play().unwrap();
  loop {
    if stop
      .recv_timeout(std::time::Duration::from_millis(500))
      .is_ok()
    {
      drop(&stream);
      writer.lock().unwrap().take().unwrap().finalize().unwrap();
    } else {
      continue;
    }
  }
}

#[test]
fn all_hosts() {
  info()
}

#[test]
fn beep() {
  let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
  let sink = rodio::Sink::try_new(&stream_handle).unwrap();
  sink.set_volume(0.2);
  let sin = rodio::source::SineWave::new(55.0);
  sink.append(sin);
  std::thread::sleep(std::time::Duration::from_secs(1));
  sink.detach();
}

#[test]
fn sample_chain() {
  use gen::SampleChain;
  let mut chain = SampleChain::default();
  chain.output_file = "../../tests/ch1.wav".into();
  chain.add_file("../../tests/ch1.wav").unwrap();
  chain.add_file("../../tests/ch2.wav").unwrap();
  chain.process_file("../../tests/ch1.wav", false).unwrap();
  chain.process_file("../../tests/ch2.wav", false).unwrap();
}

#[test]
fn metro() {
  use gen::metro::MetroMsg::Stop;
  use gen::Metro;
  let metro = Metro::new(128, 4, 4).start("../../tests/ch1.wav", "../../tests/ch2.wav");
  std::thread::sleep(std::time::Duration::from_secs(1));
  metro.send(Stop).unwrap();
}
