//! BIN -- MK
use std::env;
use std::net::SocketAddr;

use clap::Parser;
use mpk::repl;

pub static HELP: &'static str = "$mk
-d  \t  / debug
-c  \t  / client addr
-s  \t  / server addr";

#[derive(Parser)]
struct Args {
  #[clap(long, short)]
  client: Option<String>,
  #[clap(long, short)]
  server: Option<String>,
  #[clap(long, short)]
  debug: bool,
}

#[tokio::main]
async fn main() {
  let args = Args::parse();
  let mut rl = repl::init_repl().unwrap();
  let printer = rl.create_external_printer().unwrap();
  let (mut evaluator, rx) = repl::Repl::new(rl);
  let disp = tokio::spawn(async move {
    let mut dispatcher = repl::Dispatcher::new(
      printer,
      args.client.unwrap_or("127.0.0.1:0".to_string()),
      args.server.unwrap_or("127.0.0.1:57813".to_string()),
      rx,
    )
    .await;
    dispatcher.run().await;
  });
  evaluator.parse(args.debug).await;
  disp.abort();
  std::process::exit(0);
}
