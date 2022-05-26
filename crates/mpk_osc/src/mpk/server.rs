//! MPK_OSC -- MPK/SERVER
use crate::{Error, OscPacket, OscType, Result, ToOsc};

#[derive(Debug, Clone)]
pub enum ServerMessage<'a> {
  Ack,
  Reply(&'a str, &'a str),
  Result(ResultMessageKind),
}

impl<'a> ServerMessage<'a> {
  pub fn parse(p: &'a mut OscPacket) -> Result<Self> {
    ServerMessage::try_from(p)
  }
}

impl<'a> TryFrom<&'a mut OscPacket> for ServerMessage<'a> {
  type Error = Error;
  fn try_from(p: &'a mut OscPacket) -> Result<Self> {
    match p {
      OscPacket::Message(ref m) => {
        if m.addr.starts_with("/ack") {
          Ok(ServerMessage::Ack)
        } else if m.addr.starts_with("/reply") {
          Ok(ServerMessage::Reply("", ""))
        } else if m.addr.starts_with("/result") {
          Ok(ServerMessage::Result(ResultMessageKind::parse(p)?))
        } else {
          Err(Error::BadAddr(m.addr.clone()))
        }
      }
      e => Err(Error::BadPacket(e.to_owned())),
    }
  }
}

impl<'a> ToOsc for ServerMessage<'a> {
  fn addr(&self) -> String {
    match self {
      ServerMessage::Ack => "/ack".to_string(),
      ServerMessage::Reply(_, _) => "/reply".to_string(),
      ServerMessage::Result(r) => r.addr(),
    }
  }
  fn args(&self) -> Vec<OscType> {
    match self {
      ServerMessage::Ack => vec![],
      ServerMessage::Reply(r, m) => vec![
        OscType::String(r.to_string()),
        OscType::String(m.to_string()),
      ],
      ServerMessage::Result(r) => r.args(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum ResultMessageKind {
  Ok(String),
  Err(String),
}

impl ResultMessageKind {
  pub fn parse(p: &mut OscPacket) -> Result<Self> {
    ResultMessageKind::try_from(p)
  }
}

impl TryFrom<&mut OscPacket> for ResultMessageKind {
  type Error = Error;
  fn try_from(p: &mut OscPacket) -> Result<Self> {
    match p {
      OscPacket::Message(m) => {
        if m.addr.starts_with("/result/ok") {
          let msg = m.args[0].clone().string();
          Ok(ResultMessageKind::Ok(msg.unwrap()))
        } else if m.addr.starts_with("/result/err") {
          Ok(ResultMessageKind::Err("".to_string()))
        } else {
          Err(Error::BadAddr(m.addr.clone()))
        }
      }
      e => Err(Error::BadPacket(e.to_owned())),
    }
  }
}

impl ToOsc for ResultMessageKind {
  fn addr(&self) -> String {
    match self {
      ResultMessageKind::Ok(_) => "/result/ok/".to_string(),
      ResultMessageKind::Err(_) => "/result/err/".to_string(),
    }
  }
  fn args(&self) -> Vec<OscType> {
    match self {
      ResultMessageKind::Ok(r) => vec![OscType::String(r.to_string())],
      ResultMessageKind::Err(e) => vec![OscType::String(e.to_string())],
    }
  }
}
