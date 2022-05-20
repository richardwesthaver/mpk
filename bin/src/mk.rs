//! BIN -- MK
use clap::Parser;
use mpk::repl;

pub const HELP: &'static str = "$mk
-d          / debug
-c addr     / client
-s addr     / server
-t N        / timeout";

#[derive(Parser)]
#[clap(override_help = HELP)]
struct Args {
  #[clap(long, short)]
  client: Option<String>,
  #[clap(long, short)]
  server: Option<String>,
  #[clap(long, short)]
  debug: bool,
  #[clap(long, short, default_value_t=1_000)]
  timeout: u64,
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
  repl::exec(client.as_str(), server.as_str(), args.timeout, args.debug).await.unwrap();
  std::process::exit(0);
}
