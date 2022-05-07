use clap::Parser;
use mpk::{config::Config, engine::Engine, util::expand_tilde, Error};

#[derive(Parser)]
#[clap(name = "mpkd")]
#[clap(about = "media programming kit daemon")]
#[clap(author = "ellis <ellis@rwest.io>")]
#[clap(version = option_env!("CARGO_PKG_VERSION").unwrap_or("NULL"))]
/// MPKD -- Media Programming Kit Daemon
struct Args {
  /// UDP socket address
  addr: Option<String>,
  /// Use specified config file
  #[clap(short,long, default_value_t = String::from("~/mpk/mpk.toml"))]
  cfg: String,
  /// Enable DB server
  #[clap(short, long)]
  db: bool,
  /// Enable HTTP server
  #[clap(short, long)]
  http: bool,
  /// Enable SESH server
  #[clap(short, long)]
  sesh: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let mut args = Args::parse();
  let cfg_path = expand_tilde(&args.cfg).unwrap();
  let cfg = if cfg_path.exists() {
    Config::load(&cfg_path)?
  } else {
    Config::default()
  };

  let addr = if let Some(a) = args.addr {
    a
  } else {
    cfg.engine.socket
  };

  let mut engine = Engine::new(addr).await;

  if !(args.http || args.db || args.sesh) {
    args.http = true;
    args.db = true;
    args.sesh = true;
  }

  if args.http {}

  if args.db {}

  if args.sesh {}

  Ok(engine.run().await)
}
