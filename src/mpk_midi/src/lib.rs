//! MPK_MIDI
//!
//! MIDI functions and types for MPK.
use midir::{Ignore, MidiInput, MidiOutput};

mod err;
pub use err::{Error, Result};

mod monitor;
pub use monitor::monitor;

/// Detect available midi devices and print a summary.
pub fn list_midi_ports() -> Result<()> {
  let mut midi_in = MidiInput::new("list_midi_input")?;
  midi_in.ignore(Ignore::None);
  let midi_out = MidiOutput::new("list_midi_output")?;

  println!("MIDI inputs:");
  for (i, p) in midi_in.ports().iter().enumerate() {
    println!("  {}: {}", i, midi_in.port_name(p)?);
  }

  println!("MIDI outputs:");
  for (i, p) in midi_out.ports().iter().enumerate() {
    println!("  {}: {}", i, midi_out.port_name(p)?);
  }
  Ok(())
}
