//! MPK_OSC
pub use rosc::{decoder, encoder, OscMessage, OscPacket, OscType};

mod err;
pub use err::{Error, OscError, Result};

pub mod ardour;
pub mod mpk;
pub mod nsm;
pub mod supercollider;

pub trait ToOsc {
  fn msg(&self) -> OscPacket {
    OscPacket::Message(OscMessage {
      addr: self.addr(),
      args: self.args(),
    })
  }
  fn addr(&self) -> String;
  fn args(&self) -> Vec<OscType>;
}

// this doesn't really save space
// #[macro_export]
// macro_rules! matcher {
//   ($t:tt, $($v:ident$(($($a:ident),+))? => $e:expr),+ ) => {
//     match $t {
//     $(Self::$v$(($($a),+))? => {$e}),*
//     }
//   }
// }

#[cfg(test)]
mod tests {
  use std::net::UdpSocket;

  use rosc::encoder;
  use rosc::{OscMessage, OscPacket, OscType};

  use super::{nsm::*, ToOsc};

  fn spawn_nsmd() {
    std::process::Command::new("nohup")
      .args(["nsmd", "&"])
      .spawn()
      .unwrap();
  }

  fn test_nsm_client<'a>(url: Option<&'a str>) -> NsmClient<'a> {
    NsmClient::new("test_client", "127.0.0.1:0", url, ClientCaps::all()).unwrap()
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

  #[cfg(linux)]
  #[test]
  fn test_nsm() {
    spawn_nsmd();
    let mut client = test_nsm_client(None);
    //    client.new_project("a_lovely_day").unwrap();
    client.list().unwrap();
    client.open("a_lovely_day").unwrap();
    client.announce().unwrap();
    client.add("zynaddsubfx").unwrap();
    client.abort().unwrap()
  }

  #[cfg(linux)]
  #[test]
  fn test_nsm_replies() {
    let mut client = test_nsm_client(None);
    let p = ClientReply::Open("/some/path").msg();
    dbg!(&p);
    client.reply(&p).unwrap();
  }
}
