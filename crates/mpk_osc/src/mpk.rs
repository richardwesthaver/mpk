//! MPK_OSC --- MPK ENGINE API
use crate::ToOsc;
use crate::{ardour, nsm, Error, OscPacket, OscType, Result};

#[derive(Debug)]
pub enum ClientMessage<'a> {
  Http(HttpMessageKind),
  Osc(OscMessageKind<'a>),
  Db(DbMessageKind),
}

#[derive(Debug)]
pub enum HttpMessageKind {
  FreeSound(FreeSoundMsg),
}

#[derive(Debug)]
pub enum FreeSoundMsg {}

#[derive(Debug)]
pub enum OscMessageKind<'a> {
  Nsm(nsm::ClientMessage<'a>),
  Ardour(ardour::ClientMessage<'a>),
}

impl<'a> OscMessageKind<'a> {
  pub fn parse(p: &'a mut OscPacket) -> Result<Self> {
    OscMessageKind::try_from(p)
  }
}

impl<'a> TryFrom<&'a mut OscPacket> for OscMessageKind<'a> {
  type Error = Error;
  fn try_from(p: &'a mut OscPacket) -> Result<Self> {
    match p {
      OscPacket::Message(m) => {
        if let Some(addr) = m.addr.strip_prefix("/mpk/osc") {
          m.addr = addr.to_string();
          if m.addr.starts_with("/nsm/") {
            Ok(OscMessageKind::Nsm(nsm::ClientMessage::parse(p)?))
          } else if m.addr.starts_with("/ard/") {
            Ok(OscMessageKind::Ardour(ardour::ClientMessage::parse(p)?))
          } else {
            Err(Error::BadAddr(m.addr.clone()))
          }
        } else {
          Err(Error::BadAddr(m.addr.clone()))
        }
      }
      e => Err(Error::BadPacket(e.to_owned())),
    }
  }
}

impl<'a> ToOsc for OscMessageKind<'a> {
  fn addr(&self) -> String {
    match self {
      OscMessageKind::Nsm(m) => m.addr(),
      OscMessageKind::Ardour(m) => m.addr(),
    }
  }
  fn args(&self) -> Vec<OscType> {
    match self {
      OscMessageKind::Nsm(m) => m.args(),
      OscMessageKind::Ardour(m) => m.args(),
    }
  }
}

#[derive(Debug)]
pub enum DbMessageKind {
  QueryMsg,
}

#[derive(Debug)]
pub enum ServerMessage<'a> {
  Ack,
  Reply(&'a str, &'a str),
  Result,
}

impl<'a> ToOsc for ServerMessage<'a> {
  fn addr(&self) -> String {
    match self {
      ServerMessage::Ack => "/ack".to_string(),
      ServerMessage::Reply(_, _) => "/reply".to_string(),
      ServerMessage::Result => "/res".to_string(),
    }
  }
  fn args(&self) -> Vec<OscType> {
    match self {
      ServerMessage::Ack => vec![],
      ServerMessage::Reply(r, m) => vec![
        OscType::String(r.to_string()),
        OscType::String(m.to_string()),
      ],
      ServerMessage::Result => vec![],
    }
  }
}
