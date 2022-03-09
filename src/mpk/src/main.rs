use std::path::{Path, MAIN_SEPARATOR, PathBuf};
use clap::{Parser, Subcommand, AppSettings};

use mpk_config::{Config, DEFAULT_PATH, CONFIG_FILE};
use mpk_db::{Mdb, TrackTags};
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
  /// Run a service
  Run {
    #[clap(subcommand)]
    runner: Runner,
  },
  /// Save a session
  Save,
  /// Query MDB
  Query,
  /// Sync resources with DB
  Sync {
    #[clap(short, long)]
    tracks: bool,
    #[clap(short, long)]
    samples: bool,
    #[clap(short, long)]
    projects: bool,
  },
  /// Print info
  Info {
    #[clap(short, long)]
    audio: bool,
    #[clap(short,long)]
    midi: bool,
  },
  /// Package resources [.tar.zst]
  Pack,
  /// Unpackage resources [.tar.zst]
  Unpack,
  /// Shutdown services
  Quit,
}

#[derive(Subcommand)]
enum Runner {
  // start the jack server
  Jack,
  // start a network server
  Net,
  // create a sample chain
  Chain {
    #[clap(parse(from_os_str))]
    input: Vec<PathBuf>,
    #[clap(parse(from_os_str))]
    output: PathBuf,
  },
  Metro {
    bpm: u16,
    time_sig: String,
  }
}

fn ppln(i:&str,s:char) {
  match s {
    // progress
    'p' => eprint!("  \x1b[1m{}\x1b[0m ... ", i),
    // done
    'd' => eprint!("\x1b[1;32m{}\x1b[0m\n", i),
    // err
    'e' => eprint!("\x1b[0:31m{}\x1b[0m", i),
    // Error
    'E' => eprintln!("\x1b[0:31m{}\x1b[0m", i),
    _ => eprintln!("{}", i),
  }
}

fn main() -> Result<()> {
  let args = Args::parse();
  let cfg_path = Path::new(&args.cfg);
  let cfg = if cfg_path.exists() {
    Config::load(cfg_path)?
  } else {
    Config::default()
  };

  match args.cmd {
    Command::Init => {
      ppln("Initializing MPK", 'p');
      cfg.build()?;
      cfg.write(cfg_path)?;
      let db_path = cfg.db.path();
      Mdb::new(db_path.as_deref())?.init()?;
      ppln("[DONE]", 'd');
    },
    Command::Info{audio, midi} => {
      if audio {
	mpk_audio::info();
      } else if midi {
	mpk_midi::list_midi_ports()?;
      } else {
	mpk_audio::info();
	mpk_midi::list_midi_ports()?;
      }
    },
    Command::Sync { tracks, samples, projects } => {
      let conn = Mdb::new_with_config(cfg.db)?;

      if tracks {
	let ts = cfg.fs.get_path("tracks")?;
	let mut coll = Vec::new();
	id3_walk(&ts, &mut coll)?;
	for i in coll {
	  let path = i.path.strip_prefix(&ts).unwrap().to_str().unwrap();
	  let title = i.get_tag("TIT2");
	  let artist = i.get_tag("TPE1");
	  let album = i.get_tag("TALB");
	  let genre = i.get_tag("TCON");
	  let year = i.get_tag("TDRC").map(|y| y.parse::<i16>().unwrap());

	  conn.insert_track(&path)?;
	  let track_id = conn.last_insert_rowid();
	  let tags = TrackTags {
	    artist,
	    title,
	    album,
	    genre,
	    year
	  };
	  conn.insert_track_tags(track_id, &tags)?;
	}
      }
      if samples {
	let _ss = cfg.fs.get_path("samples")?;
      }
      if projects {
	let _ps = cfg.fs.get_path("projects")?;
      }
    },
    Command::Run{runner} => {
      match runner {
	Runner::Metro{bpm, time_sig} => {
	  let sig: Vec<u8> = time_sig.trim().split("/")
	    .map(|x| x.parse().unwrap())
	    .collect();
	  let metro = mpk_audio::gen::Metro::new(bpm, sig[0], sig[1]);
	  metro.start();
	  loop {}
	}
	_ => println!("starting jack server"),
      }
    }
    _ => ppln("Invalid command", 'E'),
  }

  Ok(())
}
