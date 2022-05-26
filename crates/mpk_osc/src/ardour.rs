//! MPK_OSC -- ARDOUR
//!
//! REF: https://manual.ardour.org/using-control-surfaces/controlling-ardour-with-osc/
use crate::{
  // decoder, encoder,
  Error,
  OscPacket,
  OscType,
  Result,
  ToOsc,
};
// use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

/// A Client for Ardour.
///
/// This struct communicates with the Ardour OSC control surface API over a UDP Socket.
// #[derive(Debug)]
// pub struct ArdourClient<'client> {
//   name: &'client str,
// }

#[derive(Debug, Clone)]
pub enum ClientMessage<'a> {
  StripList,
  StripSends(&'a str), // this is actually an int but we're just testing and too lazy to remove lifetime
}

impl<'a> ClientMessage<'a> {
  pub fn parse(p: &'a OscPacket) -> Result<Self> {
    ClientMessage::try_from(p)
  }
}

impl<'a> TryFrom<&'a OscPacket> for ClientMessage<'a> {
  type Error = Error;
  fn try_from(p: &'a OscPacket) -> Result<Self> {
    match p {
      OscPacket::Message(ref m) => {
        match m.addr.as_str() {
          "/strip/list" => Ok(ClientMessage::StripList),
          "/strip/sends" => Ok(ClientMessage::StripSends("")), // TODO
          e => Err(Error::BadAddr(e.to_string())),
        }
      }
      e => Err(Error::BadPacket(e.to_owned())),
    }
  }
}

impl<'a> ToOsc for ClientMessage<'a> {
  fn addr(&self) -> String {
    match self {
      ClientMessage::StripList => "/strip/list".to_string(),
      ClientMessage::StripSends(_) => "/strip/sends".to_string(),
    }
  }
  fn args(&self) -> Vec<OscType> {
    match self {
      ClientMessage::StripList => vec![],
      ClientMessage::StripSends(s) => vec![OscType::String(s.to_string())],
    }
  }
}
