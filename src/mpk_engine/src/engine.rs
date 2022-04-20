//! MPK_ENGINE ENGINE
use mpk_osc::decoder;
use std::net::{SocketAddr, ToSocketAddrs};
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
    let buf = Vec::with_capacity(MTU);
    let engine = Engine { socket, buf };
    println!(
      "MPK_ENGINE listening on {}",
      engine.socket.local_addr().unwrap()
    );
    engine
  }

  pub async fn recv(&mut self) {
    if let Ok((_, addr)) = self.socket.recv_from(&mut self.buf).await {
      print!("rx@{}: ", addr);
      match decoder::decode_udp(&self.buf) {
        Ok((_, res)) => println!("{:?}", res),
        Err(e) => println!("{}", e),
      }
    }
  }
}
