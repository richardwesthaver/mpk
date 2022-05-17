//! MPK_REPL DISPATCH
//!
//! Receives an AST structure from parser and dispatches command to
//! remote processes. The Dispatcher takes ownership of an
//! ExternalPrinter which is used to asynchronously print status and
//! response messages back to the client.
use std::net::{SocketAddr, ToSocketAddrs};

use mpk_osc::{decoder, encoder, OscMessage, OscPacket, OscType};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{error::TryRecvError, Receiver};

use crate::parser::ast::{AstNode, SysVerb};
use crate::ExternalPrinter;

pub const MTU: usize = 1536;

#[derive(Debug)]
pub struct Dispatcher<T: ExternalPrinter> {
  printer: T,
  socket: UdpSocket,
  engine_url: SocketAddr,
  buf: Vec<u8>,
  rx: Receiver<AstNode>,
}

impl<T: ExternalPrinter> Dispatcher<T> {
  pub async fn new<A: ToSocketAddrs>(
    printer: T,
    addr: A,
    engine_url: A,
    rx: Receiver<AstNode>,
  ) -> Self {
    let addr = addr.to_socket_addrs().unwrap().next().unwrap();
    let mut dispatcher = Dispatcher {
      printer,
      socket: UdpSocket::bind(addr).await.unwrap(),
      engine_url: engine_url.to_socket_addrs().unwrap().next().unwrap(),
      buf: vec![0; MTU],
      rx,
    };
    dispatcher
      .printer
      .print(format!(
        "dispatching {} -> {}",
        dispatcher.socket.local_addr().unwrap().to_string(),
        dispatcher.engine_url
      ))
      .unwrap();

    dispatcher
  }

  pub fn print(&mut self, msg: String) {
    let mut msg = msg;
    msg.push_str("\n");
    self.printer.print(msg).unwrap()
  }

  pub async fn run(&mut self) {
    loop {
      while let Ok(_) = self.rx().await {
        self.dispatch().await.unwrap();
        if let Err(_) = self.recv().await {
          self.print("err: dispatch:recv deadline has passed".to_string())
        }
      }
      self.buf.clear();
    }
  }

  pub async fn recv(&mut self) -> Result<(), tokio::time::error::Elapsed> {
    tokio::time::timeout(std::time::Duration::from_millis(1000), async move {
      if let Ok((size, _addr)) = self.socket.recv_from(&mut self.buf).await {
        match decoder::decode_udp(&self.buf[..size]) {
          Ok((_, res)) => self.print(format!("{:?}", res)),
          Err(e) => self.print(e.to_string()),
        }
      }
    })
    .await
  }

  pub async fn dispatch(&mut self) -> std::io::Result<usize> {
    self.socket.send_to(&mut self.buf, self.engine_url).await
  }

  pub async fn rx(&mut self) -> Result<(), TryRecvError> {
    match self.rx.try_recv() {
      Ok(AstNode::SysFn { verb, args }) => {
        // init OSC addr and args
        let mut addr = "/mpk/".to_string();
        let mut args_buf = vec![];

        match verb {
          SysVerb::Sesh => addr.push_str("sesh/"),
          SysVerb::Db => addr.push_str("db/"),

          SysVerb::Http => addr.push_str("http/"),

          SysVerb::Osc => {
            addr.push_str("osc/");
            match args.map(|n| *n) {
              Some(AstNode::Name(s)) => match s.as_str() {
                "mpk" => addr.push_str("self/"),
                "nsm" => addr.push_str("nsm/"),
                "ardour" | "ard" => addr.push_str("ard/"),
                "supercollider" | "sc" => addr.push_str("sc/"),
                _ => self.print("invalid sys target".to_string()),
              },
              Some(AstNode::Str(s)) => match s.to_str().unwrap() {
                "mpk" => addr.push_str("self/"),
                "nsm" => addr.push_str("nsm/"),
                "ardour" | "ard" => addr.push_str("ard/"),
                "supercollider" | "sc" => addr.push_str("sc/"),
                _ => self.print("invalid sys target".to_string()),
              },
              Some(AstNode::Symbol(s)) => match s.as_str() {
                "mpk" => addr.push_str("self/"),
                "nsm" => addr.push_str("nsm/"),
                "ardour" | "ard" => addr.push_str("ard/"),
                "supercollider" | "sc" => addr.push_str("sc/"),
                _ => self.print("invalid sys target".to_string()),
              },
              Some(AstNode::Nouns(l)) => {
                let mut l = l.into_iter();
                // first arg is the service target
                match l.next() {
                  Some(AstNode::Str(s)) => match s.to_str().unwrap() {
                    "mpk" => addr.push_str("self/"),
                    "nsm" => addr.push_str("nsm/"),
                    "ardour" | "ard" => addr.push_str("ard/"),
                    "supercollider" | "sc" => addr.push_str("sc/"),

                    _ => self.print("invalid sys target".to_string()),
                  },
                  Some(AstNode::Symbol(s)) => match s.as_str() {
                    "mpk" => addr.push_str("self/"),
                    "nsm" => addr.push_str("nsm/"),
                    "ardour" | "ard" => addr.push_str("ard/"),
                    "supercollider" | "sc" => addr.push_str("sc/"),
                    _ => self.print("invalid sys target".to_string()),
                  },
                  _ => self.print("first arg should be string or symbol".to_string()),
                }

                match l.next() {
                  Some(AstNode::Symbol(s)) => match s.as_str() {
                    "announce" => addr.push_str("server/announce"),
                    _ => self.print("invalid sys path".to_string()),
                  },
                  _ => self.print("second arg should be symbol".to_string()),
                }

                match l.next() {
                  Some(AstNode::Str(s)) => {
                    args_buf.push(OscType::String(s.into_string().unwrap()))
                  }
                  _ => self.print("third arg should be string".to_string()),
                }

                match l.next() {
                  Some(AstNode::Str(s)) => {
                    args_buf.push(OscType::String(s.into_string().unwrap()))
                  }
                  _ => self.print("fourth arg should be string".to_string()),
                }
                for i in l {
                  match i {
                    AstNode::Str(s) => {
                      args_buf.push(OscType::String(s.into_string().unwrap()))
                    }
                    AstNode::Int(n) => args_buf.push(OscType::Int(n)),
                    AstNode::Float(n) => args_buf.push(OscType::Double(n)),
                    _ => self.print("invalid sysverb args".to_string()),
                  }
                }
              }
              _ => self.print("first arg should be string or symbol".to_string()),
            }
          }
        }
        // wrap SysFn in OscMessage and store in self.buf as bytes
        let msg = OscPacket::Message(OscMessage {
          addr,
          args: args_buf,
        });
        self.buf = encoder::encode(&msg).unwrap();
        Ok(())
      }
      Err(e) => Err(e),
      _ => Err(TryRecvError::Empty),
    }
  }
}
