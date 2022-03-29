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
use crate::Result;
use std::net::{SocketAddr, UdpSocket};

pub const NSM_API_VERSION_MAJOR: u8 = 1;
pub const NSM_API_VERSION_MINOR: u8 = 1;

#[derive(Debug)]
pub struct NsmClient {
  pub name: String,
  pub socket: UdpSocket,
  pub addr: SocketAddr,
  pub caps: Vec<ClientCaps>,
  pub nsm_url: SocketAddr,
}

impl NsmClient {
  pub fn new(
    name: &str,
    addr: SocketAddr,
    caps: Vec<ClientCaps>,
    nsm_url: Option<SocketAddr>,
  ) -> Result<Self> {
    let socket = UdpSocket::bind(addr)?;
    let name = name.to_owned();
    let nsm_url = if let Some(u) = nsm_url {
      u
    } else {
      std::env::var("NSM_URL").unwrap().parse().unwrap()
    };

    Ok(NsmClient {
      name,
      socket,
      addr,
      caps,
      nsm_url,
    })
  }

  pub fn announce(&self) -> Result<ServerReply> {
    Ok(ServerReply::Announce)
  }
}

#[derive(Debug)]
pub enum ClientCaps {
  Dirty,
  Switch,
  Progress,
  Message,
  OptionalGui,
}

pub enum ServerCaps {
  ServerControl,
  Broadcast,
  OptionalGui,
}

pub enum ErrorCode {
  General,
  IncompatibleApi,
  Blacklisted,
  BadProject,
  CreateFailed,
  UnsavedChanges,
  NotNow,
  SaveFailed,
  NoSuchFile,
  LaunchFailed,
  NoSessionOpen,
}

pub enum ClientMessage {
  Announce,
  Reply(ClientReply),
  Progress,
  Status,
  GuiShown,
  GuiHidden,
  Dirty,
  Clean,
  Broadcast,
}

pub enum ClientReply {
  Open,
  Save,
}

pub enum ServerMessage {
  Error,
  Reply(ServerReply),
  Open,
  Save,
  SessionLoaded,
  ShowGui,
  HideGui,
  Broadcast,
}

pub enum ServerReply {
  Announce,
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

pub enum ControlMessage {
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
