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
use crate::{Error, Result};
use rosc::{decoder, encoder, OscMessage, OscPacket, OscType};
use std::fmt;
use std::net::{SocketAddr, UdpSocket, ToSocketAddrs};
use std::str::FromStr;

pub const NSM_API_VERSION_MAJOR: u8 = 1;
pub const NSM_API_VERSION_MINOR: u8 = 1;
pub const NSM_URL_VAR: &str = "NSM_URL";

pub fn nsm_url_from_env() -> SocketAddr {
  let url = std::env::var(NSM_URL_VAR).unwrap();
  url.strip_prefix("osc.udp://").unwrap().strip_suffix('/').unwrap().to_socket_addrs().unwrap().next().unwrap()
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

  pub fn announce(&mut self) -> Result<()> {
    let m = ClientMessage::Announce(self.name, self.caps).msg();
    let msg = encoder::encode(&m)?;
    self.socket.send_to(&msg, self.nsm_url)?;
    Ok(())
  }

  pub fn send(&self, _msg: ClientMessage) -> Result<()> {
    Ok(())
  }

  pub fn recv(&mut self) -> Result<OscPacket> {
    match self.socket.recv_from(&mut self.buf) {
      Ok((size, _addr)) => {
	let packet = decoder::decode_udp(&self.buf[..size])
          .expect("failed to decode packet")
          .1;
	Ok(packet)
      },
      Err(e) => Err(Error::Io(e)),
    }
  }

  pub fn reply(&mut self, p: &'a OscPacket) -> Result<ClientReply<'a>> {
    ClientReply::parse(p)
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
      e => Err(Error::BadType(e.to_string())),
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

impl<'a> ClientCaps<'a> {
  pub fn new(caps: &'a [ClientCap]) -> ClientCaps<'a> {
    ClientCaps(caps)
  }
  pub fn from_vec(caps: Vec<ClientCap>) -> ClientCaps<'a> {
    ClientCaps(caps.leak())
  }
}

impl<'a> FromStr for ClientCaps<'a> {
  type Err = Error;
  fn from_str(input: &str) -> Result<Self> {
    let mut vec = Vec::new();
    for i in input.split(':') {
      vec.push(ClientCap::from_str(i)?);
    }
    Ok(ClientCaps::from_vec(vec))
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
      e => Err(Error::BadType(e.to_string())),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct ServerCaps<'a>(&'a [ServerCap]);

impl<'a> ServerCaps<'a> {
  pub fn new(caps: &'a [ServerCap]) -> ServerCaps<'a> {
    ServerCaps(caps)
  }
  pub fn from_vec(caps: Vec<ServerCap>) -> ServerCaps<'a> {
    ServerCaps(caps.leak())
  }
}

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

impl<'a> FromStr for ServerCaps<'a> {
  type Err = Error;
  fn from_str(input: &str) -> Result<Self> {
    let mut vec = Vec::new();
    for i in input.split(':') {
      vec.push(ServerCap::from_str(i)?);
    }
    Ok(ServerCaps::from_vec(vec))
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
  Progress(f32),
  Status(i32, &'a str),
  GuiShown,
  GuiHidden,
  Dirty,
  Clean,
  Broadcast,
  Control(ClientControl<'a>),
  Reply(ClientReply<'a>),
}

impl<'a> ClientMessage<'a> {
  pub fn msg(&self) -> OscPacket {
    OscPacket::Message(OscMessage {
      addr: self.addr(),
      args: self.args(),
    })
  }

  pub fn addr(&self) -> String {
    match self {
      ClientMessage::Announce(_, _) => "/nsm/server/announce".to_string(),
      ClientMessage::Status(_, _) => "/nsm/client/message".to_string(),
      ClientMessage::Progress(_) => "/nsm/client/progress".to_string(),
      ClientMessage::GuiShown => "/nsm/client/gui_is_hidden".to_string(),
      ClientMessage::GuiHidden => "/nsm/client/gui_is_shown".to_string(),
      ClientMessage::Clean => "/nsm/client/is_clean".to_string(),
      ClientMessage::Dirty => "/nsm/client/is_dirty".to_string(),
      ClientMessage::Broadcast => "/nsm/server/broadcast".to_string(),
      ClientMessage::Control(m) => m.addr(),
      ClientMessage::Reply(m) => m.addr(),
    }
  }

  pub fn args(&self) -> Vec<OscType> {
    match self {
      ClientMessage::Announce(a, b) => {
        vec![
          OscType::String(a.to_string()),
          OscType::String(b.to_string()),
          OscType::String(std::env::args().nth(0).unwrap()),
          OscType::Int(NSM_API_VERSION_MAJOR as i32),
          OscType::Int(NSM_API_VERSION_MINOR as i32),
          OscType::Int(std::process::id() as i32),
        ]
      }
      ClientMessage::Status(a, b) => {
        vec![OscType::Int(*a), OscType::String(b.to_string())]
      }
      ClientMessage::Progress(a) => {
        vec![OscType::Float(*a)]
      }
      ClientMessage::GuiShown => {
        vec![]
      }
      ClientMessage::GuiHidden => {
        vec![]
      }
      ClientMessage::Clean => {
        vec![]
      }
      ClientMessage::Dirty => {
        vec![]
      }
      ClientMessage::Broadcast => {
        vec![]
      }
      ClientMessage::Control(p) => p.args(),
      ClientMessage::Reply(p) => p.args(),
    }
  }
}

pub enum ClientReply<'a> {
  Open(&'a str),
  Save(&'a str),
}

impl<'a> ClientReply<'a> {
  pub fn msg(&self) -> OscPacket {
    OscPacket::Message(OscMessage {
      addr: self.addr(),
      args: self.args(),
    })
  }

  pub fn addr(&self) -> String {
    "/reply".to_string()
  }

  pub fn args(&self) -> Vec<OscType> {
    match self {
      ClientReply::Open(m) => {
        vec![
          OscType::String("/nsm/client/open".to_string()),
          OscType::String(m.to_string()),
        ]
      }
      ClientReply::Save(m) => {
        vec![
          OscType::String("/nsm/client/save".to_string()),
          OscType::String(m.to_string()),
        ]
      }
    }
  }

  pub fn parse(p: &'a OscPacket) -> Result<Self> {
    ClientReply::try_from(p)
  }
}

impl<'a> TryFrom<&'a OscPacket> for ClientReply<'a> {
  type Error = Error;
  fn try_from(p: &'a OscPacket) -> Result<Self> {
    match p {
      OscPacket::Message(m) => {
	if &m.addr == "/reply" {
	  let args = &m.args;

	  let raddr = if let OscType::String(ref s) = args[0] {
	    Some(s)
	  } else {
	    None
	  };

	  let msg = if let OscType::String(ref s) = args[1] {
	    Some(s)
	  } else {
	    None
	  };

	  if let Some(ref s) = raddr {
	    match s.as_str() {
	      "/nsm/client/open" => Ok(ClientReply::Open(msg.unwrap())),
	      "nsm/client/save" => Ok(ClientReply::Open(msg.unwrap())),
	      _ => Err(Error::Osc(rosc::OscError::BadMessage("Unable to parse ClientReply")))
	    }
	  } else {
	    Err(Error::Osc(rosc::OscError::BadMessage("Unable to parse ClientReply")))
	  }
	} else {
	  Err(Error::Osc(rosc::OscError::BadMessage("Unable to parse ClientReply")))
	}
      },
      _ => Err(Error::Osc(rosc::OscError::BadMessage("Unable to parse ClientReply")))
    }
  }
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

impl<'a> ClientControl<'a> {
  pub fn msg(&self) -> OscPacket {
    OscPacket::Message(OscMessage {
      addr: self.addr(),
      args: self.args(),
    })
  }

  pub fn addr(&self) -> String {
    let addr = match self {
      ClientControl::Add(_) => "/nsm/server/add",
      ClientControl::Save => "/nsm/server/save",
      ClientControl::Open(_) => "/nsm/server/open",
      ClientControl::New(_) => "/nsm/server/new",
      ClientControl::Duplicate(_) => "/nsm/server/duplicate",
      ClientControl::Close => "/nsm/server/close",
      ClientControl::Abort => "/nsm/server/abort",
      ClientControl::Quit => "/nsm/server/quiit",
      ClientControl::List => "/nsm/server/list",
    };
    addr.to_string()
  }

  pub fn args(&self) -> Vec<OscType> {
    match self {
      ClientControl::Add(a) => {
        vec![OscType::String(a.to_string())]
      }
      ClientControl::Save => {
        vec![]
      }
      ClientControl::Open(a) => {
        vec![OscType::String(a.to_string())]
      }
      ClientControl::New(a) => {
        vec![OscType::String(a.to_string())]
      }
      ClientControl::Duplicate(a) => {
        vec![OscType::String(a.to_string())]
      }
      ClientControl::Close => {
        vec![]
      }
      ClientControl::Abort => {
        vec![]
      }
      ClientControl::Quit => {
        vec![]
      }
      ClientControl::List => {
        vec![]
      }
    }
  }
}

pub enum ServerMessage<'a> {
  Reply(ServerReply<'a>),
  Open(&'a str, &'a str, &'a str),
  Save,
  SessionLoaded,
  ShowGui,
  HideGui,
  Broadcast(&'a str, &'a str),
}

impl<'a> ServerMessage<'a> {
  pub fn msg(&self) -> OscPacket {
    OscPacket::Message(OscMessage {
      addr: self.addr(),
      args: self.args(),
    })
  }

  pub fn addr(&self) -> String {
    match self {
      ServerMessage::Reply(m) => m.addr(),
      ServerMessage::Open(_, _, _) => "/nsm/client/open".to_string(),
      ServerMessage::Save => "/nsm/client/save".to_string(),
      ServerMessage::SessionLoaded => "/nsm/client/session_is_loaded".to_string(),
      ServerMessage::ShowGui => "/nsm/client/show_optional_gui".to_string(),
      ServerMessage::HideGui => "/nsm/client/hide_optional_gui".to_string(),
      ServerMessage::Broadcast(a, _) => a.to_string(),
    }
  }

  pub fn args(&self) -> Vec<OscType> {
    match self {
      ServerMessage::Reply(m) => m.args(),
      ServerMessage::Open(a, b, c) => {
        vec![
          OscType::String(a.to_string()),
          OscType::String(b.to_string()),
          OscType::String(c.to_string()),
        ]
      }
      ServerMessage::Save => vec![],
      ServerMessage::SessionLoaded => vec![],
      ServerMessage::ShowGui => vec![],
      ServerMessage::HideGui => vec![],
      ServerMessage::Broadcast(_, b) => {
        vec![OscType::String(b.to_string())]
      }
    }
  }
}

pub enum ServerReply<'a> {
  Announce(&'a str, &'a str, ServerCaps<'a>),
  Add(&'a str),
  Save(&'a str),
  Open(&'a str),
  New(&'a str),
  Duplicate(&'a str),
  Close(&'a str),
  Abort(&'a str),
  Quit(&'a str),
  List(&'a str),
}

impl<'a> ServerReply<'a> {
  pub fn msg(&self) -> OscPacket {
    OscPacket::Message(OscMessage {
      addr: self.addr(),
      args: self.args(),
    })
  }

  pub fn addr(&self) -> String {
    "/reply".to_string()
  }

  pub fn args(&self) -> Vec<OscType> {
    match self {
      ServerReply::Announce(a, b, c) => {
        vec![
          OscType::String("/nsm/server/announce".to_string()),
          OscType::String(a.to_string()),
          OscType::String(b.to_string()),
          OscType::String(c.to_string()),
        ]
      }
      ServerReply::Add(a) => {
        vec![OscType::String("/nsm/server/add".to_string()),
	     OscType::String(a.to_string())]
      }
      ServerReply::Save(a) => {
        vec![OscType::String("/nsm/server/save".to_string()),
	     OscType::String(a.to_string())]
      }
      ServerReply::Open(a) => {
        vec![OscType::String("/nsm/server/open".to_string()),
	     OscType::String(a.to_string())]
      }
      ServerReply::New(a) => {
        vec![OscType::String("/nsm/server/new".to_string()),
	  OscType::String(a.to_string())]
      }
      ServerReply::Duplicate(a) => {
        vec![OscType::String("/nsm/server/duplicate".to_string()),
	  OscType::String(a.to_string())]
      }
      ServerReply::Close(a) => {
        vec![OscType::String("/nsm/server/close".to_string()),
	  OscType::String(a.to_string())]
      }
      ServerReply::Abort(a) => {
        vec![OscType::String("/nsm/server/abort".to_string()),
	  OscType::String(a.to_string())]
      }
      ServerReply::Quit(a) => {
        vec![OscType::String("/nsm/server/quit".to_string()),
	  OscType::String(a.to_string())]
      }
      ServerReply::List(a) => {
        vec![OscType::String("/nsm/server/list".to_string()),
	  OscType::String(a.to_string())]
      }
    }
  }

  pub fn parse(p: &'a OscPacket) -> Result<Self> {
    ServerReply::try_from(p)
  }
}

impl<'a> TryFrom<&'a OscPacket> for ServerReply<'a> {
  type Error = Error;
  fn try_from(p: &'a OscPacket) -> Result<Self> {
    match p {
      OscPacket::Message(m) => {
	if &m.addr == "/reply" {
	  let args = &m.args;

	  let raddr = if let OscType::String(ref s) = args[0] {
	    Some(s)
	  } else {
	    None
	  };

	  let msg = if let OscType::String(ref s) = args[1] {
	    Some(s)
	  } else {
	    None
	  };

	  if let Some(ref s) = raddr {
	    match s.as_str() {
	      "/nsm/server/announce" => {
		let name = if let OscType::String(ref s) = args[2] {
		  Some(s)
		} else {
		  None
		};

		let caps = if let OscType::String(ref s) = args[3] {
		  let c = ServerCaps::from_str(s).unwrap();
		  Some(c)
		} else {
		  None
		};

		Ok(ServerReply::Announce(msg.unwrap(), name.unwrap(), caps.unwrap()))
	      },
	      "/nsm/server/add" => Ok(ServerReply::Add(msg.unwrap())),
	      "nsm/server/save" => Ok(ServerReply::Save(msg.unwrap())),
	      "/nsm/server/open" => Ok(ServerReply::Open(msg.unwrap())),
	      "/nsm/server/new" => Ok(ServerReply::New(msg.unwrap())),
	      "/nsm/server/duplicate" => Ok(ServerReply::Duplicate(msg.unwrap())),
	      "/nsm/server/close" => Ok(ServerReply::Close(msg.unwrap())),
	      "/nsm/server/abort" => Ok(ServerReply::Abort(msg.unwrap())),
	      "/nsm/server/quit" => Ok(ServerReply::Quit(msg.unwrap())),
	      "/nsm/server/list" => Ok(ServerReply::List(msg.unwrap())),
	      _ => Err(Error::Osc(rosc::OscError::BadMessage("Unable to parse ClientReply")))
	    }
	  } else {
	    Err(Error::Osc(rosc::OscError::BadMessage("Unable to parse ClientReply")))
	  }
	} else {
	  Err(Error::Osc(rosc::OscError::BadMessage("Unable to parse ClientReply")))
	}
      },
      _ => Err(Error::Osc(rosc::OscError::BadMessage("Unable to parse ClientReply")))
    }
  }
}
