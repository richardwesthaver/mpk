use clap::{AppSettings, Parser, Subcommand};
use mpk::Result;
use mpk_audio::gen::SampleChain;
use mpk_config::{expand_tilde, Config};
use mpk_db::{Mdb, QueryType};
use std::io;
use std::path::PathBuf;
use std::sync::mpsc::sync_channel;
use std::thread;

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
  #[clap(short,long, default_value_t = String::from("~/mpk/mpk.toml"))]
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
  /// Play an audio file
  Play {
    file: Option<PathBuf>,
    query: Option<String>,
    #[clap(short)]
    volume: Option<f32>,
    #[clap(short)]
    speed: Option<f32>,
    #[clap(short)]
    device: Option<String>,
  },
  /// Run a service
  Run {
    #[clap(subcommand)]
    runner: Runner,
  },
  /// Save a session
  Save,
  /// Interact with the database
  Db {
    #[clap(subcommand)]
    cmd: DbCmd,
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
  /// start a JACK service
  Jack {
    name: String,
  },
  /// start a network service
  Net,
  /// create a sample chain
  Chain {
    #[clap(parse(from_os_str))]
    input: Vec<PathBuf>,
    #[clap(short, long, parse(from_os_str))]
    output: PathBuf,
    #[clap(short, long)]
    even: bool,
    /// Generate an octatrack data file (.ot)
    #[clap(long)]
    ot: bool,
  },
  Plot,
  /// start the metronome
  Metro {
    bpm: Option<u16>,
    time_sig: Option<String>,
  },
  Monitor,
}

#[derive(Subcommand)]
enum DbCmd {
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
  Backup {
    output: PathBuf,
  },
  Restore {
    input: PathBuf,
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
  let cfg_path = expand_tilde(&args.cfg).unwrap();
  let cfg = if cfg_path.exists() {
    Config::load(&cfg_path)?
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
    Command::Play {
      file,
      query: _,
      volume,
      speed,
      device,
    } => {
      let device = if let Some(d) = device {
        mpk_audio::device_from_str(d.as_str())
      } else {
        None
      };
      let rx = mpk_audio::pause_controller_cli();
      mpk_audio::play(file.unwrap(), device, volume, speed, rx)
    }

    Command::Db { cmd } => {
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

      match cmd {
        DbCmd::Query {
          query,
          track,
          sample,
          raw,
        } => match query {
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
        },
        DbCmd::Sync {
          tracks,
          samples,
          projects,
        } => {
          let script = cfg.extractor.path.unwrap_or_default();
          let descriptors = cfg.extractor.descriptors;
          if tracks {
            let mut cmd = std::process::Command::new(&script);
            let tracks = cfg.fs.get_path("tracks")?;
            cmd.args([tracks.to_str().unwrap(), "-t", "track", "-d"]);
            cmd.args(&descriptors);
            cmd.status()?;
          }
          if samples {
            let mut cmd = std::process::Command::new(&script);
            let samps = cfg.fs.get_path("samples")?;
            cmd.args([samps.to_str().unwrap(), "-t", "sample", "-d"]);
            cmd.args(&descriptors);
            cmd.status()?;
          }
          if projects {
            let _projs = cfg.fs.get_path("projects")?;
          }
        }
        DbCmd::Backup { output } => {
          conn.backup(output, Some(|p| mpk_db::print_progress(p)))?
        }
        DbCmd::Restore { input } => conn.restore(
          mpk_db::DatabaseName::Main,
          input,
          Some(|p| mpk_db::print_progress(p)),
        )?,
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
      Runner::Jack { name } => {
        // Wait for user input to quit
        let (tx, rx) = sync_channel(1);
        println!("Press enter to quit...");
        thread::spawn(move || {
          let mut input = String::new();
          io::stdin().read_line(&mut input).ok();
          tx.send(()).unwrap();
        });
        mpk_jack::internal_client(&name, rx);
      }
      Runner::Chain {
        input,
        output,
        even,
        ot,
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
        if ot {
          mpk_gear::octatrack::generate_ot_file(&mut chain)?;
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
      Runner::Monitor => mpk_midi::monitor()?,
      _ => println!("starting jack server"),
    },
    _ => ppln("Invalid command", 'E'),
  }

  Ok(())
}
