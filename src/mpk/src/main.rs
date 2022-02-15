use clap::Parser;
use std::path::PathBuf;
use mpk_config::Config;


#[derive(Parser, Debug)]
struct Args {
  #[clap(short,long)]
  cfg: PathBuf,
}
fn main() {
  let args = Args::parse();
  let cfg_path = args.cfg;
  let cfg = Config::default();

  cfg.write(cfg_path).unwrap();
  cfg.build().unwrap();
}
