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
// use crate::OscPacket;
use std::net::{SocketAddr, UdpSocket};

pub const NSM_API_VERSION_MAJOR: u8 = 1;
pub const NSM_API_VERSION_MINOR: u8 = 1;

pub struct NsmClient {
  pub name: String,
  pub socket: UdpSocket,
  pub addr: SocketAddr,
  pub nsm_url: SocketAddr,
  pub capabilities: Vec<ClientCaps>,
}

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
