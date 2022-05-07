//! MPK_ENGINE --- ENGINE
use std::net::{SocketAddr, ToSocketAddrs};

use mpk_osc::mpk::{OscMessageKind, ServerMessage};
use mpk_osc::{decoder, encoder, OscPacket, ToOsc};
use tokio::net::UdpSocket;

pub const MTU: usize = 1536;

pub struct Engine {
  socket: UdpSocket,
  buf: Vec<u8>,
}

impl Engine {
  pub async fn new<A: ToSocketAddrs>(addr: A) -> Self {
    let addr = addr.to_socket_addrs().unwrap().next().unwrap();
    let socket = UdpSocket::bind(addr).await.unwrap();
    let buf = vec![0; MTU];
    let engine = Engine { socket, buf };
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
