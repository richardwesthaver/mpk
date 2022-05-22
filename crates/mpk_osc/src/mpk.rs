//! MPK_OSC --- MPK ENGINE API
use crate::ToOsc;
use crate::{ardour, nsm, Error, OscPacket, OscType, Result};

#[derive(Debug)]
pub enum ClientMessage<'a> {
  Http(HttpMessageKind),
  Proxy(OscMessageKind<'a>),
  Db(DbMessageKind),
  Vm(VmMessageKind),
  Sesh(SeshMessageKind),
}

#[derive(Debug)]
pub enum HttpMessageKind {
  FreeSound(FreeSoundMsg),
  Musicbrainz(MusicbrainzMsg),
  AcoustId(AcoustIdMsg),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum MusicbrainzMsg {
  Lookup,
  Browse,
  Search,
}

#[derive(Debug)]
pub enum AcoustIdMsg {
  Lookup,
}

#[derive(Debug)]
pub enum OscMessageKind<'a> {
  Nsm(nsm::ClientMessage<'a>),
  Ardour(ardour::ClientMessage<'a>),
  Supercollider,
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
          } else if m.addr.starts_with("/ardour/") {
            Ok(OscMessageKind::Ardour(ardour::ClientMessage::parse(p)?))
          } else if m.addr.starts_with("/sc/") {
            Ok(OscMessageKind::Supercollider) // TODO
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
      OscMessageKind::Supercollider => "/mpk/sc".to_string(), // TODO
    }
  }
  fn args(&self) -> Vec<OscType> {
    match self {
      OscMessageKind::Nsm(m) => m.args(),
      OscMessageKind::Ardour(m) => m.args(),
      OscMessageKind::Supercollider => vec![], // TODO
    }
  }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum VmMessageKind {
  Eval,
  Load,
  Work,
  Vars,
  Exit,
  Gc,
}

#[derive(Debug)]
pub enum SeshMessageKind {}

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
      ServerMessage::Result => "/result".to_string(),
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
