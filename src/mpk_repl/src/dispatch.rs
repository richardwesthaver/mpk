//! MPK_REPL DISPATCH
//!
//! Receives an AST structure from parser and dispatches command to
//! remote processes. The Dispatcher takes ownership of an
//! ExternalPrinter which is used to asynchronously print status and
//! response messages back to the client.
use crate::ExternalPrinter;
use tokio::net;

#[derive(Debug)]
pub struct Dispatcher<T: ExternalPrinter> {
  printer: T,
  socket: net::UdpSocket,
}
