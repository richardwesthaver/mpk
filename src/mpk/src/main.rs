use std::path::{Path, MAIN_SEPARATOR};
use clap::{Parser, Subcommand, AppSettings};

use mpk_config::{Config, DEFAULT_PATH, CONFIG_FILE};
use mpk_db::Mdb;
use mpk_id3::id3_walk;

use mpk::Result;

#[derive(Parser)]
#[clap(name = "mpk")]
#[clap(about = "media programming kit")]
#[clap(author = "ellis <ellis@rwest.io>")]
#[clap(version = option_env!("CARGO_PKG_VERSION").unwrap_or("NULL"))]
/// MPK -- Media Programming Kit
///
/// Tools for building and managing creative workflows on UNIX systems.
struct Args {
  #[clap(subcommand)]
  cmd: Command,
  #[clap(short,long, default_value_t = [DEFAULT_PATH, &MAIN_SEPARATOR.to_string(), CONFIG_FILE].concat())]
  cfg: String,
}

#[derive(Subcommand)]
#[clap(setting = AppSettings::DeriveDisplayOrder)]
enum Command {
  /// Initialize MPK
  Init,
  /// Start MPK instance
  Start,
  /// Save MPK instance
  Save,
  /// Query MPK resources
  Query,
  /// Sync MPK resources with DB
  Sync {
    #[clap(short, long)]
    tracks: bool,
    #[clap(short, long)]
    samples: bool,
    #[clap(short, long)]
    projects: bool,
  },
  /// Print info about MPK resources
  Info,
  /// Package MPK resources [.tar.zst]
  Pack,
  /// Unpackage MPK resources [.tar.zst]
  Unpack,
  /// Shutdown MPK instance
  Quit,
}


fn ppln(i:&str,s:char) {
  match s {
    'p' => eprint!("  \x1b[1m{}\x1b[0m ... ", i),
    'd' => eprint!("\x1b[1;32m{}\x1b[0m\n", i),
    'e' => eprint!("\x1b[0:31m{}\x1b[0m", i),
    'E' => eprintln!("\x1b[0:31m{}\x1b[0m", i),
    _ => eprintln!("{}", i),
  }
}

fn main() -> Result<()> {
  let args = Args::parse();
  let cfg_path = Path::new(&args.cfg);
  let cfg = if cfg_path.exists() {
    Config::load(cfg_path).unwrap()
  } else {
    Config::default()
  };

  match args.cmd {
    Command::Init => {
      ppln("Initializing MPK", 'p');
      cfg.build().unwrap();
      cfg.write(cfg_path)?;
      Mdb::new(cfg.db.path())?.init()?;
      ppln("[DONE]", 'd');
    },
    Command::Info => {
      mpk_midi::list_midi_ports().unwrap();
    }
    Command::Sync { tracks, samples, projects } => {
      let conn = Mdb::new_with_config(cfg.db)?;

      if tracks {
	let ts = cfg.fs.get_path("tracks")?;
	let mut coll = Vec::new();
	id3_walk(&ts, &mut coll).unwrap();
	for i in coll {
	  let path = i.path.strip_prefix(&ts).unwrap().to_str().unwrap();
	  let title = i.get_tag("TIT2");
	  let artist = i.get_tag("TPE1");
	  let album = i.get_tag("TALB");
	  let genre = i.get_tag("TCON");
	  let year = i.get_tag("TDRC");

	  conn.insert_track(&path)?;
	  let track_id = conn.last_insert_rowid();
	  conn.insert_track_tags(track_id, artist, title, album, genre, year)?;
	}
      }
      if samples {
	let _ss = cfg.fs.get_path("samples")?;
      }
      if projects {
	let _ps = cfg.fs.get_path("projects")?;
      }
    }
    _ => ppln("Invalid command", 'E'),
  }

  Ok(())
}
