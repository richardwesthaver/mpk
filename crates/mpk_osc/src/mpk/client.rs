//! MPK_OSC -- MPK/CLIENT
use crate::ToOsc;
use crate::{ardour, nsm, Error, OscPacket, OscType, Result};

#[derive(Debug, Clone)]
pub enum ClientMessage<'a> {
  Vm(VmMessageKind<'a>),
  Sesh(SeshMessageKind),
  Proxy(ProxyMessageKind<'a>),
  Db(DbMessageKind),
}

impl<'a> ClientMessage<'a> {
  pub fn parse(p: &'a mut OscPacket) -> Result<Self> {
    ClientMessage::try_from(p)
  }
}

impl<'a> TryFrom<&'a mut OscPacket> for ClientMessage<'a> {
  type Error = Error;
  fn try_from(p: &'a mut OscPacket) -> Result<Self> {
    match p {
      OscPacket::Message(m) => {
        if m.addr.starts_with("/mpk/vm/") {
          Ok(ClientMessage::Vm(VmMessageKind::parse(p)?))
        } else if m.addr.starts_with("/mpk/proxy/") {
          Ok(ClientMessage::Proxy(ProxyMessageKind::parse(p)?))
        } else {
          Err(Error::BadAddr(m.addr.clone()))
        }
      }
      e => Err(Error::BadPacket(e.to_owned())),
    }
  }
}

impl<'a> ToOsc for ClientMessage<'a> {
  fn addr(&self) -> String {
    match self {
      ClientMessage::Vm(m) => m.addr(),
      ClientMessage::Proxy(m) => m.addr(),
      _ => todo!(),
    }
  }
  fn args(&self) -> Vec<OscType> {
    match self {
      ClientMessage::Vm(m) => m.args(),
      ClientMessage::Proxy(_m) => vec![], // TODO
      _ => todo!(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum HttpMessageKind {
  FreeSound(FreeSoundMsg),
  Musicbrainz(MusicbrainzMsg),
  AcoustId(AcoustIdMsg),
}

#[derive(Debug, Clone)]
pub enum FreeSoundMsg {
  SearchText,
  SearchContent,
  SearchCombined,
  Sound,
  SoundAnalysis,
  SoundSimilar,
  SoundDownload,
  SoundUpload,
  SoundDescribe,
  SoundPendingUpload,
  SoundEdit,
  SoundBookmark,
  SoundRate,
  SoundComment,
  User,
  UserSounds,
  UserPacks,
  UserBookmarkCategories,
  UserBookmarkCategorySounds,
  Pack,
  PackSounds,
  PackDownload,
  Me,
  Descriptors,
}

#[derive(Debug, Clone)]
pub enum MusicbrainzMsg {
  Lookup,
  Browse,
  Search,
}

#[derive(Debug, Clone)]
pub enum AcoustIdMsg {
  Lookup,
}

#[derive(Debug, Clone)]
pub enum ProxyMessageKind<'a> {
  Nsm(nsm::ClientMessage<'a>),
  Ardour(ardour::ClientMessage<'a>),
  Supercollider,
}

impl<'a> ProxyMessageKind<'a> {
  pub fn parse(p: &'a mut OscPacket) -> Result<Self> {
    ProxyMessageKind::try_from(p)
  }
}

impl<'a> TryFrom<&'a mut OscPacket> for ProxyMessageKind<'a> {
  type Error = Error;
  fn try_from(p: &'a mut OscPacket) -> Result<Self> {
    match p {
      OscPacket::Message(m) => {
        if let Some(addr) = m.addr.strip_prefix("/mpk/osc") {
          m.addr = addr.to_string();
          if m.addr.starts_with("/nsm/") {
            Ok(ProxyMessageKind::Nsm(nsm::ClientMessage::parse(p)?))
          } else if m.addr.starts_with("/ardour/") {
            Ok(ProxyMessageKind::Ardour(ardour::ClientMessage::parse(p)?))
          } else if m.addr.starts_with("/sc/") {
            Ok(ProxyMessageKind::Supercollider) // TODO
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

impl<'a> ToOsc for ProxyMessageKind<'a> {
  fn addr(&self) -> String {
    match self {
      ProxyMessageKind::Nsm(m) => m.addr(),
      ProxyMessageKind::Ardour(m) => m.addr(),
      ProxyMessageKind::Supercollider => "/mpk/sc".to_string(), // TODO
    }
  }
  fn args(&self) -> Vec<OscType> {
    match self {
      ProxyMessageKind::Nsm(m) => m.args(),
      ProxyMessageKind::Ardour(m) => m.args(),
      ProxyMessageKind::Supercollider => vec![], // TODO
    }
  }
}

#[derive(Debug, Clone)]
pub enum DbMessageKind {
  Open,
  Drop,
  Flush,
  List,
  Query,
  Get,
  Watch,
  Insert,
  Swap,
  Remove,
  Info,
}

#[derive(Debug, Clone)]
pub enum VmMessageKind<'a> {
  Eval(Vec<Vec<u8>>),
  Load(&'a str),
  Work,
  Vars,
  Exit,
  Gc,
}

impl<'a> VmMessageKind<'a> {
  pub fn parse(p: &'a mut OscPacket) -> Result<Self> {
    VmMessageKind::try_from(p)
  }
}

impl<'a> TryFrom<&'a mut OscPacket> for VmMessageKind<'a> {
  type Error = Error;
  fn try_from(p: &'a mut OscPacket) -> Result<Self> {
    match p {
      OscPacket::Message(ref m) => {
        if m.addr.starts_with("/mpk/vm/eval/") {
          let args = m
            .args
            .iter()
            .map_while(|x| x.clone().blob())
            .collect::<Vec<Vec<u8>>>();
          Ok(VmMessageKind::Eval(args))
        } else if m.addr.starts_with("/mpk/vm/load/") {
          let arg = if let OscType::String(ref s) = m.args[0] {
            Some(s)
          } else {
            None
          };
          Ok(VmMessageKind::Load(arg.unwrap()))
        } else {
          Err(Error::BadAddr(m.addr.clone()))
        }
      }
      e => Err(Error::BadPacket(e.to_owned())),
    }
  }
}

impl<'a> ToOsc for VmMessageKind<'a> {
  fn addr(&self) -> String {
    match self {
      VmMessageKind::Eval(_) => "/mpk/vm/eval/".to_string(),
      VmMessageKind::Load(_) => "/mpk/vm/load/".to_string(),
      _ => todo!(),
    }
  }
  fn args(&self) -> Vec<OscType> {
    match self {
      VmMessageKind::Eval(m) => {
        m.to_vec().into_iter().map(|p| OscType::Blob(p)).collect()
      }
      VmMessageKind::Load(s) => vec![OscType::String(s.to_string())], // TODO
      _ => todo!(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum SeshMessageKind {}
