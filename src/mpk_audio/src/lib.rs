pub use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::cpal::{available_hosts, host_from_id, ALL_HOSTS};
pub use rodio::cpal::{Device, Devices};
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

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
      false => if pause.recv_timeout(std::time::Duration::from_millis(500)).is_ok() {
        sink.pause()
      } else {
        continue;
      },
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
  chain.add_file("../../tests/ch1.wav").unwrap();
  chain.add_file("../../tests/ch2.wav").unwrap();
  chain.process_file("../../tests/ch1.wav", false).unwrap();
  chain.process_file("../../tests/ch2.wav", false).unwrap();
}

#[test]
fn metro() {
  use gen::metro::MetroMsg::Stop;
  use gen::Metro;
  let metro = Metro::new(128, 4, 4).start("ch1.wav", "ch2.wav");
  std::thread::sleep(std::time::Duration::from_secs(1));
  metro.send(Stop).unwrap();
}
