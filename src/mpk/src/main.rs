use clap::{AppSettings, Parser, Subcommand};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use mpk::Result;
use mpk_audio::gen::SampleChain;
use mpk_config::{Config, CONFIG_FILE, DEFAULT_PATH};
use mpk_db::{Mdb, QueryType};

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
  /// enable DB tracing
  #[clap(long)]
  db_trace: bool,
  /// enable DB profiling
  #[clap(long)]
  db_profile: bool,
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
  Pack {
    #[clap(parse(from_os_str))]
    input: PathBuf,
    #[clap(parse(from_os_str))]
    output: PathBuf,
    #[clap(short, long)]
    level: Option<u8>,
  },
  /// Unpackage resources [.tar.zst]
  Unpack {
    #[clap(parse(from_os_str))]
    input: PathBuf,
    #[clap(parse(from_os_str))]
    output: PathBuf,
    #[clap(short, long)]
    replace: bool,
  },
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
    #[clap(short, long)]
    even: bool,
  },
  Plot,
  /// start the metronome
  Metro {
    bpm: Option<u16>,
    time_sig: Option<String>,
  },
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
      let (trace, profile) = (
        &cfg.db.trace | args.db_trace,
        &cfg.db.profile | args.db_profile,
      );
      let mut conn = Mdb::new_with_config(cfg.db)?;
      if trace {
        conn.set_tracer(Some(|x| println!("{}", x)))
      }
      if profile {
        conn.set_profiler(Some(|x, y| println!("{} -- {}ms", x, y.as_millis())))
      }
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
    Command::Pack {
      input,
      output,
      level,
    } => mpk_flate::pack(input, output, level.map(|x| x.into())),
    Command::Unpack {
      input,
      output,
      replace,
    } => {
      if replace {
        mpk_flate::unpack_replace(input, output)
      } else {
        mpk_flate::unpack(input, output)
      }
    }
    Command::Run { runner } => match runner {
      Runner::Chain {
        input,
        output,
        even,
      } => {
        let mut chain = SampleChain::default();
        chain.output_file = output.with_extension("");
        chain.output_ext = output
          .extension()
          .unwrap()
          .to_str()
          .unwrap()
          .parse()
          .unwrap();
        for i in &input {
          chain.add_file(i)?;
        }
        for i in &input {
          chain.process_file(i, even)?;
        }
      }
      Runner::Metro { bpm, time_sig } => {
        let bpm = match bpm {
          Some(b) => b,
          None => cfg.metro.bpm,
        };
        let sig: (u8, u8) = match time_sig {
          Some(t) => {
            let tsig: Vec<u8> =
              t.trim().split("/").map(|x| x.parse().unwrap()).collect();
            (tsig[0], tsig[1])
          }
          None => cfg.metro.time_sig,
        };

        let metro = mpk_audio::gen::Metro::new(bpm, sig.0, sig.1);
        metro.start(cfg.metro.tic.unwrap(), cfg.metro.toc.unwrap());
        loop {}
      }
      Runner::Plot => {
        let _data = mpk_db::Mdb::new_with_config(cfg.db)?
          .query_sample_features_rhythm(1)?
          .histogram;
      }
      _ => println!("starting jack server"),
    },
    _ => ppln("Invalid command", 'E'),
  }

  Ok(())
}
