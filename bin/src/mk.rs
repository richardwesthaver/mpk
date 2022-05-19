//! BIN -- MK
use clap::Parser;
use mpk::repl;

pub const HELP: &'static str = "$mk
-d            / debug
-c [addr]     / client addr
-s [addr]     / server addr";

#[derive(Parser)]
#[clap(override_help = HELP)]
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
  let client = if let Some(s) = args.client {
    s
  } else {
    "127.0.0.1:0".into()
  };
  let server = if let Some(s) = args.server {
    s
  } else {
    "127.0.0.1:57813".into()
  };

  let mut rl = repl::init_repl().unwrap();
  let printer = rl.create_external_printer().unwrap();
  let (mut evaluator, rx) = repl::Repl::new(rl);
  let disp = tokio::spawn(async move {
    let mut d = repl::init_dispatcher(
      printer,
      client.as_str(),
      server.as_str(),
      rx,
    ).await.unwrap();
    d.run().await;
  });
  evaluator.parse(args.debug).await;
  disp.abort();
  std::process::exit(0);
}
