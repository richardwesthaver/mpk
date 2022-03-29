//! MPK_MIDI MONITOR
//!
//! A simple MIDI monitor which prints all incoming messages from a
//! specified input to stdout.
use crate::{Error, Result};
use midir::{Ignore, MidiInput, MidiInputPort};
use std::io::{stdin, stdout, Write};

pub fn get_midi_input_port(midi_in: &MidiInput) -> Result<MidiInputPort> {
  let in_ports = midi_in.ports();
  let in_port = match in_ports.len() {
    0 => return Err(Error::MidiInit(midir::InitError)),
    1 => {
      println!(
        "Choosing the only available input port: {}",
        midi_in.port_name(&in_ports[0]).unwrap()
      );
      &in_ports[0]
    }
    _ => {
      println!("\nAvailable input ports:");
      for (i, p) in in_ports.iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(p).unwrap());
      }
      print!("Please select input port: ");
      stdout().flush()?;
      let mut input = String::new();
      stdin().read_line(&mut input)?;
      in_ports.get(input.trim().parse::<usize>()?).unwrap()
    }
  };
  Ok(in_port.clone())
}

pub fn monitor() -> Result<()> {
  let mut midi_in = MidiInput::new("mpk_monitor")?;
  midi_in.ignore(Ignore::None);
  let in_port = get_midi_input_port(&midi_in)?;
  // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
  let _conn_in = midi_in
    .connect(
      &in_port,
      "mpk_monitor",
      move |stamp, message, _| {
        println!("{}: {:?}", stamp, message);
      },
      (),
    )
    .unwrap();
  let mut input = String::new();
  stdin().read_line(&mut input)?; // wait for next enter key press
  Ok(())
}
