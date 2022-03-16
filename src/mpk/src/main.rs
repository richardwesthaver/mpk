use clap::{AppSettings, Parser, Subcommand};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use mpk_config::{Config, CONFIG_FILE, DEFAULT_PATH};
use mpk_db::{Mdb, QueryType};

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
  /// Query DB
  Query {
    query: QueryType,
    #[clap(short, long)]
    track: Option<i64>,
    #[clap(short, long)]
    sample: Option<i64>,
    raw: Option<String>,
  },
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
    #[clap(short, long)]
    midi: bool,
    #[clap(short, long)]
    db: bool,
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
  /// start the jack server
  Jack,
  /// start a network server
  Net,
  /// create a sample chain
  Chain {
    #[clap(parse(from_os_str))]
    input: Vec<PathBuf>,
    #[clap(parse(from_os_str))]
    output: PathBuf,
  },
  /// start the metronome
  Metro { bpm: u16, time_sig: String },
}

fn ppln(i: &str, s: char) {
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
    }
    Command::Info { audio, midi, db } => {
      if audio {
        println!("\x1b[1mAUDIO INFO\x1b[0m");
        mpk_audio::info();
      }
      if midi {
        println!("\x1b[1mMIDI INFO\x1b[0m");
        mpk_midi::list_midi_ports()?;
      }
      if db {
        println!("\x1b[1mDB INFO\x1b[0m");
        let db = Mdb::new_with_config(cfg.db.to_owned())?;
        let ts = db.track_count()?;
        let ss = db.sample_count()?;
        println!("{} tracks", ts);
        println!("{} samples", ss);
      }
      if !(audio || midi || db) {
        println!("\x1b[1mDB INFO\x1b[0m");
        let db = Mdb::new_with_config(cfg.db)?;
        let ts = db.track_count()?;
        let ss = db.sample_count()?;
        println!("{} tracks", ts);
        println!("{} samples", ss);
        println!("\x1b[1mAUDIO INFO\x1b[0m");
        mpk_audio::info();
        println!("\x1b[1mMIDI INFO\x1b[0m");
        mpk_midi::list_midi_ports()?;
      }
    }
    Command::Query {
      query,
      track,
      sample,
      raw,
    } => {
      let conn = Mdb::new_with_config(cfg.db)?;
      match query {
        QueryType::Info => {
          if track.is_some() {
            println!("{}", conn.query_track(track.unwrap())?)
          } else if sample.is_some() {
            println!("{}", conn.query_sample(sample.unwrap())?)
          }
        }
        QueryType::Tags => {
          if track.is_some() {
            println!("{}", conn.query_track_tags(track.unwrap())?)
          } else {
            eprintln!("query type not supported")
          }
        }
        QueryType::Musicbrainz => {
          if track.is_some() {
            println!("{}", conn.query_track_tags_musicbrainz(track.unwrap())?)
          } else {
            eprintln!("query type not supported");
          }
        }
        QueryType::Lowlevel => {
          if track.is_some() {
            println!("{}", conn.query_track_features_lowlevel(track.unwrap())?)
          } else if sample.is_some() {
            println!("{}", conn.query_sample_features_lowlevel(sample.unwrap())?)
          }
        }
        QueryType::Rhythm => {
          if track.is_some() {
            println!("{}", conn.query_track_features_rhythm(track.unwrap())?)
          }
          if sample.is_some() {
            println!("{}", conn.query_sample_features_rhythm(sample.unwrap())?)
          }
        }
        QueryType::Spectrograms => {
          if track.is_some() {
            println!("{}", conn.query_track_images(track.unwrap())?)
          } else if sample.is_some() {
            println!("{}", conn.query_sample_images(sample.unwrap())?)
          }
        }
        QueryType::Raw => {
          println!("{}", conn.query_raw(&raw.unwrap())?)
        }
        _ => eprintln!("query type not supported"),
      }
    }
    Command::Sync {
      tracks,
      samples,
      projects,
    } => {
      //      let conn = Mdb::new_with_config(cfg.db)?;

      if tracks {
        let _ts = cfg.fs.get_path("tracks")?;
      }
      if samples {
        let _ss = cfg.fs.get_path("samples")?;
      }
      if projects {
        let _ps = cfg.fs.get_path("projects")?;
      }
    }
    Command::Run { runner } => match runner {
      Runner::Metro { bpm, time_sig } => {
        let sig: Vec<u8> = time_sig
          .trim()
          .split("/")
          .map(|x| x.parse().unwrap())
          .collect();
        let metro = mpk_audio::gen::Metro::new(bpm, sig[0], sig[1]);
        metro.start();
        loop {}
      }
      _ => println!("starting jack server"),
    },
    _ => ppln("Invalid command", 'E'),
  }

  Ok(())
}
