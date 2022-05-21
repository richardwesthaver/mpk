//! MPK_REPL DISPATCH
//!
//! Receives an AST structure from parser and dispatches command to
//! remote processes. The Dispatcher takes ownership of an
//! ExternalPrinter which is used to asynchronously print status and
//! response messages back to the client.
use std::net::{SocketAddr, ToSocketAddrs};

use mpk_osc::{decoder, encoder, OscMessage, OscPacket, OscType};
use mpk_util::is_zeroes;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{error::TryRecvError, Receiver};

use crate::parser::ast::{AstNode, SysVerb};
use crate::ExternalPrinter;

pub const MTU: usize = 1536;

/// The mk dispatcher.
///
/// This struct is intended to run in a separate thread. A
/// syntactically correct Vec<AstNode> is received via self.rx(),
/// semantically verified, and sent to the engine using OscPackets
/// over UDP where evaluation occurs. Status updates are printed
/// asynchronously using the ExternalPrinter.
#[derive(Debug)]
pub struct Dispatcher<T: ExternalPrinter> {
  printer: T,
  socket: UdpSocket,
  engine_url: SocketAddr,
  buf: [u8; MTU],
  rx: Receiver<Vec<AstNode>>,
  timeout: u64,
}

impl<T: ExternalPrinter> Dispatcher<T> {
  /// Create a new dispatcher.
  pub async fn new<A: ToSocketAddrs>(
    printer: T,
    addr: A,
    engine_url: A,
    rx: Receiver<Vec<AstNode>>,
    timeout: u64,
  ) -> Self {
    let addr = addr.to_socket_addrs().unwrap().next().unwrap();
    let mut dispatcher = Dispatcher {
      printer,
      socket: UdpSocket::bind(addr).await.unwrap(),
      engine_url: engine_url.to_socket_addrs().unwrap().next().unwrap(),
      buf: [0; MTU],
      rx,
      timeout,
    };
    dispatcher.print(format!(
      "dispatching {} -> {} / !{}",
      dispatcher.socket.local_addr().unwrap().to_string(),
      dispatcher.engine_url,
      timeout
    ));
    dispatcher
  }

  /// Print a new line asynchronously to the REPL.
  pub fn print(&mut self, msg: String) {
    let mut msg = msg;
    msg.push_str("\n");
    self.printer.print(msg).unwrap()
  }

  /// Run the dispatcher loop, receiving a Vec<AstNode> from the
  /// channel, encoding to OSC, dispatching to engine and receiving
  /// response.
  pub async fn run(&mut self) {
    loop {
      while let Ok(_) = self.rx().await {
        match self.dispatch().await {
          Ok(_) => {
            if let Err(_) = self.recv().await {
              self.print("err: dispatch:recv deadline has passed".to_string())
            }
            self.buf.fill(0);
          }
          Err(_) => (),
        }
      }
    }
  }

  /// Receive a packet from the engine, waiting for at most TIMEOUT milliseconds.
  pub async fn recv(&mut self) -> Result<(), tokio::time::error::Elapsed> {
    tokio::time::timeout(std::time::Duration::from_millis(self.timeout), async move {
      if let Ok((size, _addr)) = self.socket.recv_from(&mut self.buf).await {
        match decoder::decode_udp(&self.buf[..size]) {
          Ok((_, res)) => self.print(format!("{:?}", res)),
          Err(e) => self.print(e.to_string()),
        }
      }
    })
    .await
  }

  /// Send an encoded OscPacket to the engine for processing.
  pub async fn dispatch(&mut self) -> std::io::Result<usize> {
    if is_zeroes(&self.buf) {
      Err(std::io::Error::new(
        std::io::ErrorKind::WriteZero,
        "send buffer is empty",
      ))
    } else {
      self.socket.send_to(&mut self.buf, self.engine_url).await
    }
  }

  /// Receive a Vec<AstNode> from channel.
  pub async fn rx(&mut self) -> Result<(), TryRecvError> {
    if let Ok(prog) = self.rx.try_recv() {
      if !prog.is_empty() {
        for node in prog {
          match node {
            AstNode::SysFn { ref verb, ref args } => {
              // init OSC addr and args
              let mut addr = "/mpk/".to_string();
              let mut args_buf = vec![];

              match verb {
                SysVerb::Exit => addr.push_str("vm/exit/"),
                SysVerb::Vars => addr.push_str("vm/vars/"),
                SysVerb::Work => addr.push_str("vm/work/"),
                SysVerb::Import => addr.push_str("vm/import/"),
                SysVerb::Timeit => addr.push_str("vm/timeit/"),
                SysVerb::Sesh => addr.push_str("sesh/"),
                SysVerb::Db => addr.push_str("db/"),

                SysVerb::Http => addr.push_str("http/"),

                SysVerb::Osc => {
                  addr.push_str("osc/");
                  match args.clone().map(|n| *n).as_ref() {
                    Some(AstNode::Name(s)) => match s.as_str() {
                      "mpk" => addr.push_str("self/"),
                      "nsm" => addr.push_str("nsm/"),
                      "ardour" | "ard" => addr.push_str("ard/"),
                      "supercollider" | "sc" => addr.push_str("sc/"),
                      _ => self.print("invalid sys target".to_string()),
                    },
                    Some(AstNode::Str(s)) => match s.as_str() {
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
                        Some(AstNode::Str(s)) => match s.as_str() {
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
                        _ => {
                          self.print("first arg should be string or symbol".to_string())
                        }
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
                          args_buf.push(OscType::String(s.to_string()))
                        }
                        _ => self.print("third arg should be string".to_string()),
                      }

                      match l.next() {
                        Some(AstNode::Str(s)) => {
                          args_buf.push(OscType::String(s.to_string()))
                        }
                        _ => self.print("fourth arg should be string".to_string()),
                      }
                      for i in l {
                        match i {
                          AstNode::Str(s) => {
                            args_buf.push(OscType::String(s.to_string()))
                          }
                          AstNode::Int(n) => args_buf.push(OscType::Long(*n)),
                          AstNode::Float(n) => args_buf.push(OscType::Double(*n)),
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
              let mut packet = encoder::encode(&msg).unwrap();
              self.buf[..packet.len()].swap_with_slice(packet.as_mut_slice());
            }
            _ => (),
          }
        }
      }
      Ok(())
    } else {
      Err(TryRecvError::Empty)
    }
  }
}
