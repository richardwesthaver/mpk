use clap::Parser;
use std::fs;
use std::path::PathBuf;
use mdb_id3::id3_walk;

/// mdb-id3
#[derive(Parser, Debug)]
struct Args {
  path: PathBuf,
  output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();
  println!("parsing contents of {:?}", args.path.canonicalize()?);

  let mut coll = Vec::new();
  id3_walk(args.path, &mut coll)?;

  if let Some(ref f) = args.output {
    let out = fs::OpenOptions::new()
      .create_new(true)
      .append(true)
      .open(f)?;
    serde_json::to_writer_pretty(out, &coll)?;
  } else {
      for t in coll {
	println!("{}", serde_json::to_string_pretty(&t)?);
      }
    }
  Ok(())
}
