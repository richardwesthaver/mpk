//! MPK_OSC
pub use rosc::OscPacket;

mod err;
pub mod nsm;
pub use err::{Error, Result};

#[cfg(test)]
mod tests {
  use super::nsm::*;
  use rosc::encoder;
  use rosc::{OscMessage, OscPacket, OscType};
  use std::net::UdpSocket;
  fn test_nsm_client<'a>() -> NsmClient<'a> {
    NsmClient::new(
      "test_client",
      "127.0.0.1:0",
      Some("127.0.0.1:0"),
      &[ClientCap::Dirty, ClientCap::Switch],
    )
    .unwrap()
  }
  #[test]
  fn test_send_recv() {
    let sock1 = UdpSocket::bind("127.0.0.1:9264").unwrap();
    let sock2 = UdpSocket::bind("127.0.0.1:9265").unwrap();
    let mut rbuf = [0u8; rosc::decoder::MTU];
    for i in 0..20 {
      let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/nothisispatrick".to_string(),
        args: vec![OscType::Int(i)],
      }))
      .unwrap();
      sock1.send_to(&msg_buf, "127.0.0.1:9265").unwrap();
      match sock2.recv_from(&mut rbuf) {
        Ok((size, addr)) => {
          dbg!("Received packet with size {} from: {}", size, addr);
          assert!(rosc::decoder::decode_udp(&rbuf[..size]).is_ok());
        }
        Err(e) => {
          dbg!("Error receiving from socket: {}", e);
        }
      }
    }
  }
  #[test]
  fn test_nsm_announce() {
    let mut client = test_nsm_client();
    assert!(client.announce().is_ok());
  }

  #[test]
  fn test_nsm_replies() {
    let mut client = test_nsm_client();
    let p = ClientReply::Open("/some/path").msg();
    dbg!(&p);
    client.reply(&p).unwrap();
  }
}
