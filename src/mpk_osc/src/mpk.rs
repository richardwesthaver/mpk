//! MPK_OSC MPK ENGINE API
use crate::ToOsc;
use crate::{nsm, Error, OscPacket, OscType, Result};

#[derive(Debug)]
pub enum ClientMessage<'a> {
  Http(HttpMessageKind),
  Osc(OscMessageKind<'a>),
  Sql(SqlMessageKind),
}

#[derive(Debug)]
pub enum HttpMessageKind {}

#[derive(Debug)]
pub enum OscMessageKind<'a> {
  Nsm(nsm::ClientMessage<'a>),
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
        if let Some(addr) = m.addr.strip_prefix("/osc/nsm") {
          m.addr = addr.to_string();
          Ok(OscMessageKind::Nsm(nsm::ClientMessage::parse(p)?))
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
    "/osc".to_string()
  }
  fn args(&self) -> Vec<OscType> {
    match self {
      OscMessageKind::Nsm(m) => m.args(),
    }
  }
}

#[derive(Debug)]
pub enum SqlMessageKind {}
