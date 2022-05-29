//! MPK_ENGINE --- ENGINE
use std::net::{SocketAddr, ToSocketAddrs};

use mpk_arena::Bump;
use mpk_config::Config;
use mpk_db::Db;
use mpk_osc::mpk::{
  client::VmMessageKind, server::ResultMessageKind, ClientMessage, ServerMessage,
};
use mpk_osc::{decoder, encoder, OscPacket, ToOsc};
use mpk_parser::decode_program;
use tokio::net::UdpSocket;

use crate::{Error, Result, Vm};

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

pub struct Engine<'eng> {
  socket: UdpSocket,
  buf: Vec<u8>,
  vm: Vm<'eng, &'eng Bump>,
  db: Db,
  sesh: Option<mpk_sesh::SeshServer>,
  proxies: Option<[bool; PROXY_COUNT]>,
}

impl<'eng> Engine<'eng> {
  pub async fn new<A: ToSocketAddrs>(addr: A, alc: &'eng Bump) -> Engine<'eng> {
    let addr = addr.to_socket_addrs().unwrap().next().unwrap();
    // initialize the default configuration
    let cfg = Config::default();
    let socket = UdpSocket::bind(addr).await.unwrap();
    let buf = vec![0; MTU];
    // build up the engine modules
    let vm = Vm::<&Bump>::new(&alc);
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

  pub async fn with_config(cfg: Config, alc: &'eng Bump) -> Engine<'eng> {
    let socket =
      UdpSocket::bind(cfg.engine.socket.to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    let buf = vec![0; MTU];
    // build up the engine modules from provided CFG
    let vm = Vm::<&Bump>::new(alc);
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
        match ClientMessage::parse(&mut packet.unwrap()) {
          Ok(msg) => {
            let res = self.dispatch(msg);
            self.send(res.unwrap(), &addr).await;
          }
          Err(e) => eprintln!("{}", e.to_string()),
        }
      }
    }
  }

  pub async fn recv(&mut self) -> Result<(SocketAddr, Option<OscPacket>)> {
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
          addr,
          packet
        );
        Ok((addr, packet))
      }
      Err(e) => Err(e.into()),
    }
  }

  pub fn dispatch(&self, msg: ClientMessage<'_>) -> Result<ServerMessage> {
    let res = match msg {
      ClientMessage::Vm(m) => match m {
        VmMessageKind::Eval(prog) => {
          let program = decode_program(
            prog
              .iter()
              .map(|node| node.as_slice())
              .collect::<Vec<_>>()
              .as_slice(),
          );
          self.vm.eval(program).map_err(|e| Error::from(e))?
        }
        _ => todo!(),
      },
      ClientMessage::Sesh(m) => {
        todo!()
      }
      ClientMessage::Db(m) => {
        todo!()
      }
      ClientMessage::Proxy(m) => {
        todo!()
      }
    };
    Ok(ServerMessage::Result(res))
  }

  pub async fn send(&self, msg: ServerMessage<'_>, to: &SocketAddr) {
    if let Ok(_) = self
      .socket
      .send_to(&mut encoder::encode(&msg.msg()).unwrap(), to)
      .await
    {
      eprintln!("send: {:?}", msg)
    } else {
      eprintln!("failed: {:?}", msg)
    }
  }
}
