use rodio::cpal::{ALL_HOSTS, available_hosts, host_from_id};
pub use rodio::cpal::traits::{DeviceTrait, HostTrait};

pub mod gen;

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
      let input_configs = match device.supported_input_configs() {
        Ok(f) => f.collect(),
        Err(e) => {
          println!("    Error getting supported input configs: {:?}", e);
          Vec::new()
        }
      };
      if !input_configs.is_empty() {
        println!("    All supported input stream configs:");
        for (config_index, config) in input_configs.into_iter().enumerate() {
          println!(
            "      {}.{}. {:?}",
            device_index + 1,
            config_index + 1,
            config
          );
        }
      }

      // Output configs
      if let Ok(conf) = device.default_output_config() {
        println!("    Default output stream config:\n      {:?}", conf);
      }
      let output_configs = match device.supported_output_configs() {
        Ok(f) => f.collect(),
        Err(e) => {
          println!("    Error getting supported output configs: {:?}", e);
          Vec::new()
        }
      };
      if !output_configs.is_empty() {
        println!("    All supported output stream configs:");
        for (config_index, config) in output_configs.into_iter().enumerate() {
          println!(
            "      {}.{}. {:?}",
            device_index + 1,
            config_index + 1,
            config
          );
        }
      }
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
  chain.add_file("ch1.wav").unwrap();
  chain.add_file("ch2.wav").unwrap();
  chain.process_file("ch1.wav", false).unwrap();
  chain.process_file("ch2.wav", false).unwrap();
}

#[test]
fn metro() {
  use gen::Metro;
  use gen::metro::MetroMsg::Stop;
  let metro = Metro::new(128, 4, 4).start();
  std::thread::sleep(std::time::Duration::from_secs(1));
  metro.send(Stop).unwrap();
}
