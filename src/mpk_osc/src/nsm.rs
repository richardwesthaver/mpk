//! MPK_OSC NSM API
//!
//! NSM (formerly Non-Session Manager) is a music session manager for Linux. It
//! provides a server (nsmd) that manages a JACK session and
//! communicates with clients via OSC messages. The recommended GUI is
//! Agordejo.
//!
//! In this module we define the full client-side API. Note that the
//! server and most clients (Agordejo) are LINUX ONLY. This client
//! works on Linux or Darwin but must communicate with a server hosted
//! on a Linux machine.
//!
//! This module and nsmd are used in MPK_SESH for session management,
//! so for full session support you need a Linux box available.
//!
//! REF: https://new-session-manager.jackaudio.org
// use crate::OscPacket;
use crate::{Result, Error};
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::fmt;
use rosc::{encoder, decoder, OscPacket, OscMessage, OscType};

pub const NSM_API_VERSION_MAJOR: u8 = 1;
pub const NSM_API_VERSION_MINOR: u8 = 1;
pub const NSM_URL_VAR: &str = "NSM_URL";

pub fn nsm_url_from_env() -> SocketAddr {
  std::env::var(NSM_URL_VAR).unwrap().parse().unwrap()
}

#[derive(Debug)]
pub struct NsmClient<'a> {
  pub name: &'a str,
  pub socket: UdpSocket,
  pub addr: SocketAddr,
  pub nsm_url: SocketAddr,
  pub caps: ClientCaps<'a>,
  pub buf: [u8; decoder::MTU],
}

impl<'a> NsmClient<'a> {
  pub fn new(
    name: &'a str,
    addr: &'a str,
    nsm_url: Option<&'a str>,
    caps: &'a [ClientCap],
  ) -> Result<Self> {
    let socket = UdpSocket::bind(addr)?;
    let nsm_url: SocketAddr = if let Some(u) = nsm_url {
      u.parse().unwrap()
    } else {
      nsm_url_from_env()
    };
    let addr = addr.parse().unwrap();
    let caps = ClientCaps(caps);
    Ok(NsmClient {
      name,
      socket,
      addr,
      caps,
      nsm_url,
      buf: [0u8; decoder::MTU],
    })
  }

  pub fn announce(&self) -> Result<()> {
    let msg = encoder::encode(&ClientMessage::Announce(self.name, self.caps).msg())?;
    self.socket.send_to(&msg, self.nsm_url)?;
    Ok(())
  }

  pub fn send(&self, _msg: ClientMessage) -> Result<()> {
    Ok(())
  }

  pub fn recv(&mut self) -> Result<OscPacket> {
    match self.socket.recv_from(&mut self.buf) {
      Ok((size, _addr)) => {
	Ok(decoder::decode_udp(&self.buf[..size]).expect("failed to decode packet").1)
      }
      Err(e) => {
	Err(Error::Io(e))
      }
    }
  }

  pub fn reply(&self, _reply: ClientReply) -> Result<()> {
    Ok(())
  }
}

#[derive(Debug)]
pub enum ClientCap {
  Dirty,
  Switch,
  Progress,
  Message,
  OptionalGui,
}

impl FromStr for ClientCap {
  type Err = Error;
  fn from_str(input: &str) -> Result<Self> {
    match input {
      "dirty" => Ok(ClientCap::Dirty),
      "switch" => Ok(ClientCap::Switch),
      "progress" => Ok(ClientCap::Progress),
      "message" => Ok(ClientCap::Message),
      "optional-gui" => Ok(ClientCap::OptionalGui),
      e => Err(Error::BadType(e.to_string()))
    }
  }
}

impl fmt::Display for ClientCap {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ClientCap::Dirty => f.write_str("dirty"),
      ClientCap::Switch => f.write_str("switch"),      
      ClientCap::Progress => f.write_str("progress"),
      ClientCap::Message => f.write_str("message"),
      ClientCap::OptionalGui => f.write_str("optional-gui"),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct ClientCaps<'a>(&'a [ClientCap]);

impl<'a> fmt::Display for ClientCaps<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut s = String::new();
    for i in self.0.iter() { 
     s.push_str(&i.to_string());
      s.push(':');
    }
    f.write_str(&s)
  }
}

#[derive(Debug)]
pub enum ServerCap {
  ServerControl,
  Broadcast,
  OptionalGui,
}

impl fmt::Display for ServerCap {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ServerCap::ServerControl => f.write_str("server-control"),
      ServerCap::Broadcast => f.write_str("broadcast"),
      ServerCap::OptionalGui => f.write_str("optional-gui"),
    }
  }
}

impl FromStr for ServerCap {
  type Err = Error;
  fn from_str(input: &str) -> Result<Self> {
    match input {
      "server-control" => Ok(ServerCap::ServerControl),
      "broadcast" => Ok(ServerCap::Broadcast),
      "optional-gui" => Ok(ServerCap::OptionalGui),
      e => Err(Error::BadType(e.to_string()))
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct ServerCaps<'a>(&'a [ServerCap]);

impl<'a> fmt::Display for ServerCaps<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut s = String::new();
    for i in self.0.iter() { 
     s.push_str(&i.to_string());
      s.push(':');
    }
    f.write_str(&s)
  }
}

impl<'a> ServerCaps<'a> {
  pub fn new(caps: &'a [ServerCap]) -> ServerCaps<'a> {
    ServerCaps(caps)
  }
}

pub enum ErrorCode {
  General = -1,
  IncompatibleApi = -2,
  Blacklisted = -3,
  LaunchFailed = -4,
  NoSuchFile = -5,
  NoSessionOpen = -6,
  UnsavedChanges = -7,
  NotNow = -8,
  BadProject = -9,
  CreateFailed = -10,
  SaveFailed = -11,
}

pub type NsmResult<T> = std::result::Result<T, ErrorCode>;

pub enum ClientMessage<'a> {
  Announce(&'a str, ClientCaps<'a>),
  Reply(ClientReply<'a>),
  Control(ClientControl<'a>),
  Progress(f32),
  Status(i32, &'a str),
  GuiShown,
  GuiHidden,
  Dirty,
  Clean,
  Broadcast,
}

impl<'a> ClientMessage<'a> {
  pub fn msg(&self) -> OscPacket {
    match self {
      ClientMessage::Announce(a, b) => {
	OscPacket::Message(
	  OscMessage {
	    addr: "/nsm/server/announce".to_string(),
	    args: vec![
	      OscType::String(a.to_string()),
	      OscType::String(b.to_string()),
	      OscType::String(std::env::args().nth(0).unwrap()),
	      OscType::Int(NSM_API_VERSION_MAJOR as i32),
	      OscType::Int(NSM_API_VERSION_MINOR as i32),
	      OscType::Int(std::process::id() as i32),
	    ],
	  }
	)
      },
      ClientMessage::Reply(p) => {
	p.msg()
      },
      ClientMessage::Status(a, b) => {
	OscPacket::Message(
	  OscMessage {
	    addr: "/nsm/client/message".to_string(),
	    args: vec![OscType::Int(*a), OscType::String(b.to_string())]
	  }
	)
      },
      ClientMessage::Progress(a) => {
	OscPacket::Message(
	  OscMessage {
	    addr: "/nsm/client/progress".to_string(),
	    args: vec![OscType::Float(*a)]
	  }
	)
      },
      ClientMessage::Clean => {
	OscPacket::Message(
	  OscMessage {
	    addr: "/nsm/client/clean".to_string(),
	    args: vec![],
	  }
	)
      },
      ClientMessage::Dirty => {
	OscPacket::Message(
	  OscMessage {
	    addr: "/nsm/client/dirty".to_string(),
	    args: vec![],
	  }
	)
      },
      _ => {
	OscPacket::Message(OscMessage { addr: "".to_string(), args: vec![] })
      }
    }
  }
}

pub enum ClientReply<'a> {
  Open(&'a str),
  Save(&'a str),
  
}

pub enum ClientControl<'a> {
  Add(&'a str),
  Save,
  Open(&'a str),
  New(&'a str),
  Duplicate(&'a str),
  Close,
  Abort,
  Quit,
  List,
}

impl<'a> ClientReply<'a> {
  pub fn msg(&self) -> OscPacket {
    match self {
      ClientReply::Open(m) => {
	OscPacket::Message(
	  OscMessage {
	    addr: "/nsm/client/open".to_string(),
	    args: vec![OscType::String(m.to_string())],
	  }
	)
      },
      ClientReply::Save(m) => {
	OscPacket::Message(
	  OscMessage {
	    addr: "/nsm/client/save".to_string(),
	    args: vec![OscType::String(m.to_string())],
	  }
	)
      }
    }
  }
}

pub enum ServerMessage<'a> {
  Error,
  Reply(ServerReply<'a>),
  Open,
  Save,
  SessionLoaded,
  ShowGui,
  HideGui,
  Broadcast,
}

pub enum ServerReply<'a> {
  Announce(&'a str, &'a str, ServerCaps<'a>),
  Add,
  Save,
  Open,
  New,
  Duplicate,
  Close,
  Abort,
  Quit,
  List,
}
