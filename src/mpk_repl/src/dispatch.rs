//! MPK_REPL DISPATCH
//!
//! Receives an AST structure from parser and dispatches command to
//! remote processes. The Dispatcher takes ownership of an
//! ExternalPrinter which is used to asynchronously print status and
//! response messages back to the client.
use crate::parser::ast::{AstNode, SysVerb};
use crate::ExternalPrinter;
use mpk_osc::mpk::OscMessageKind;
use mpk_osc::{decoder, encoder, OscMessage, OscPacket, OscType};
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::Receiver;

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
      buf: Vec::with_capacity(MTU),
      rx,
    };
    dispatcher
      .printer
      .print(format!(
        "MPK dispatcher listening on {}",
        dispatcher.socket.local_addr().unwrap().to_string()
      ))
      .unwrap();

    dispatcher
  }

  pub async fn run(&mut self) {
    loop {
      self.rx().await;
      self.dispatch().await;
      self.recv().await;
    }
  }

  pub async fn recv(&mut self) {
    if let Ok((_, addr)) = self.socket.recv_from(&mut self.buf).await {
      self.printer.print(format!("{}", addr)).unwrap();
      match decoder::decode_udp(&self.buf) {
        Ok((_, res)) => self.printer.print(format!("{:?}", res)).unwrap(),
        Err(e) => self.printer.print(e.to_string()).unwrap(),
      }
    }
  }

  pub async fn dispatch(&mut self) {
    if let Ok(_) = self.socket.send_to(&mut self.buf, self.engine_url).await {
      self.buf.clear()
    }
  }

  pub async fn rx(&mut self) {
    match self.rx.recv().await {
      Some(AstNode::SysOp { verb, expr }) => {
        match verb {
          SysVerb::Http => {
            //	    self.buf.append(&mut "http message!".as_bytes().to_vec());
          }
          SysVerb::Osc => {
            let addr = "/mpk/osc/nsm/".to_string();
            let mut args = vec![];
            match expr {
              Some(b) => match *b {
                AstNode::Nouns(l) => {
                  for i in l {
                    match i {
                      AstNode::Str(s) => {
                        args.push(OscType::String(s.into_string().unwrap()))
                      }
                      AstNode::Integer(n) => args.push(OscType::Int(n)),
                      _ => self.printer.print("invalid args".to_string()).unwrap(),
                    }
                  }
                }
                _ => self.printer.print("invalid args".to_string()).unwrap(),
              },
              _ => self.printer.print("invalid args".to_string()).unwrap(),
            }
            let msg = OscPacket::Message(OscMessage { addr, args });
            self.buf = encoder::encode(&msg).unwrap();
          }
          SysVerb::Sql => {
            //	    self.buf.append(&mut "sql message!".as_bytes().to_vec());
          }
        }
      }
      _ => (),
    }
  }
}
