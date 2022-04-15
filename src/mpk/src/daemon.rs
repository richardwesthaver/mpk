use clap::Parser;
use mpk::Result;
use mpk_config::Config;
use mpk_util::expand_tilde;

#[derive(Parser)]
#[clap(name = "mpkd")]
#[clap(about = "media programming kit daemon")]
#[clap(author = "ellis <ellis@rwest.io>")]
#[clap(version = option_env!("CARGO_PKG_VERSION").unwrap_or("NULL"))]
/// MPKD -- Media Programming Kit Daemon
struct Args {
  /// Use specified config file
  #[clap(short,long, default_value_t = String::from("~/mpk/mpk.toml"))]
  cfg: String,
  /// Enable DB server
  #[clap(short, long)]
  db: bool,
  /// Enable Net server
  #[clap(short, long)]
  net: bool,
  /// Enable Sesh server
  #[clap(short, long)]
  sesh: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
  let mut args = Args::parse();
  let cfg_path = expand_tilde(&args.cfg).unwrap();
  let mut cfg = if cfg_path.exists() {
    Config::load(&cfg_path)?
  } else {
    Config::default()
  };

  if !(args.net || args.db || args.sesh) {
    args.net = true;
    args.db = true;
    args.sesh = true;
  }

  if args.net {}

  if args.db {}

  if args.sesh {}

  Ok(())
}
