//! MPK_ENGINE --- ENGINE
use std::net::{SocketAddr, ToSocketAddrs};

use mpk_config::Config;
use mpk_db::Db;
use mpk_osc::mpk::{OscMessageKind, ServerMessage};
use mpk_osc::{decoder, encoder, OscPacket, ToOsc};
use tokio::net::UdpSocket;

use crate::Vm;

pub const MTU: usize = 1536;

pub const PROXY_COUNT: usize = 6;
pub const PROXIES: [&str; PROXY_COUNT] = [
  "/osc/nsm",
  "/osc/ardour",
  "/osc/sc",
  "/http/freesound",
  "/http/acoustid",
  "/http/musicbrainz",
];

pub struct Engine {
  socket: UdpSocket,
  buf: Vec<u8>,
  vm: Vm,
  db: Db,
  sesh: Option<mpk_sesh::SeshServer>,
  proxies: Option<[bool; PROXY_COUNT]>,
}

impl Engine {
  pub async fn new<A: ToSocketAddrs>(addr: A) -> Self {
    let addr = addr.to_socket_addrs().unwrap().next().unwrap();
    // initialize the default configuration
    let cfg = Config::default();
    let socket = UdpSocket::bind(addr).await.unwrap();
    let buf = vec![0; MTU];
    // build up the engine modules
    let vm = Vm;
    let db = Db::with_config(cfg.db).unwrap();
    let sesh = None; // TODO
    let proxies = None;
    let engine = Engine {
      socket,
      buf,
      vm,
      db,
      sesh,
      proxies,
    };
    println!(
      "MPK_ENGINE listening on {}",
      engine.socket.local_addr().unwrap()
    );
    engine
  }

  pub async fn with_config(cfg: Config) -> Self {
    let socket =
      UdpSocket::bind(cfg.engine.socket.to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    let buf = vec![0; MTU];
    // build up the engine modules from provided CFG
    let vm = Vm;
    let db = Db::with_config(cfg.db).unwrap();
    let sesh = None; // TODO
    let proxies = None;
    let engine = Engine {
      socket,
      buf,
      vm,
      db,
      sesh,
      proxies,
    };
    println!(
      "MPK_ENGINE listening on {}",
      engine.socket.local_addr().unwrap()
    );
    engine
  }
  pub async fn run(&mut self) {
    loop {
      while let Ok((addr, packet)) = self.recv().await {
        self.reply(packet, &addr).await;
      }
    }
  }

  pub async fn recv(&mut self) -> std::io::Result<(SocketAddr, Option<OscPacket>)> {
    match self.socket.recv_from(&mut self.buf).await {
      Ok((size, addr)) => {
        let packet = match decoder::decode_udp(&self.buf[..size]) {
          Ok((_, res)) => Some(res),
          Err(e) => {
            eprintln!("{}", e.to_string());
            None
          }
        };
        println!(
          "{} rx@{}: {:?}",
          std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("SystemTime is before UNIX_EPOCH?")
            .as_secs(),
          &addr,
          &packet
        );
        Ok((addr, packet))
      }
      Err(e) => Err(e),
    }
  }

  pub async fn reply(&mut self, mut inc: Option<OscPacket>, to: &SocketAddr) {
    if let Some(ref mut p) = inc {
      match OscMessageKind::parse(p) {
        Ok(m) => eprintln!("{:?}", m),
        Err(e) => eprintln!("{}", e),
      }
    }

    let msg = ServerMessage::Ack.msg();
    if let Ok(_) = self
      .socket
      .send_to(&mut encoder::encode(&msg).unwrap(), to)
      .await
    {
      println!(
        "{} tx@{}: OK",
        std::time::SystemTime::now()
          .duration_since(std::time::SystemTime::UNIX_EPOCH)
          .expect("SystemTime is before UNIX_EPOCH?")
          .as_secs(),
        to
      );
    } else {
      println!(
        "{} tx@{}: ERR",
        std::time::SystemTime::now()
          .duration_since(std::time::SystemTime::UNIX_EPOCH)
          .expect("SystemTime is before UNIX_EPOCH?")
          .as_secs(),
        to
      );
    }
  }
}
