//! MPK_OSC -- SUPERCOLLIDER
//! REF: <https://doc.sccode.org/Reference/Server-Command-Reference.html>
//! REF: <https://opensoundcontrol.stanford.edu/publications/2004-Use-of-Open-Sound-Control-in-SuperCollider-Server.html>
use crate::{
  // decoder, encoder,
  Error,
  OscPacket,
  OscType,
  Result,
  ToOsc,
};
// use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

/// A Client for SuperCollider.
///
/// This struct communicates with the Sclang server over OSC.
// #[derive(Debug)]
// pub struct SuperColliderClient<'client> {
//   name: &'client str,
// }

#[derive(Debug)]
pub enum ClientMessage<'a> {
  Quit,
  DLoad(&'a str, Option<&'a [u8]>),
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
      OscPacket::Message(ref m) => match m.addr.as_str() {
        "/quit" => Ok(ClientMessage::Quit),
        "/d_load" => {
          let path = if let OscType::String(ref s) = m.args[0] {
            Some(s)
          } else {
            None
          };
          let bytes = if let OscType::Blob(ref b) = m.args[1] {
            Some(b.as_slice())
          } else {
            None
          };
          Ok(ClientMessage::DLoad(path.unwrap(), bytes))
        }
        e => Err(Error::BadAddr(e.to_string())),
      },
      e => Err(Error::BadPacket(e.to_owned())),
    }
  }
}

impl<'a> ToOsc for ClientMessage<'a> {
  fn addr(&self) -> String {
    match self {
      ClientMessage::Quit => "/quit".to_string(),
      ClientMessage::DLoad { .. } => "/d_load".to_string(),
    }
  }
  fn args(&self) -> Vec<OscType> {
    match self {
      ClientMessage::Quit => vec![],
      ClientMessage::DLoad(s, b) => {
        let mut args = vec![OscType::String(s.to_string())];
        if let Some(b) = b {
          args.push(OscType::Blob(b.to_vec()))
        }
        args
      }
    }
  }
}
