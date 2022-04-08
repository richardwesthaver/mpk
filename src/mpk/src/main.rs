use clap::{AppSettings, Parser, Subcommand};
use mpk::Result;
use mpk_audio::gen::SampleChain;
use mpk_config::{expand_tilde, Config};
use mpk_db::{AudioType, DbValue, Mdb, NaiveDate, QueryBy, QueryFor, QueryType};
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
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
  /// Enable DB tracing
  #[clap(long)]
  trace: bool,
  /// Enable DB profiling
  #[clap(long)]
  profile: bool,
}

#[derive(Subcommand)]
#[clap(setting = AppSettings::DeriveDisplayOrder)]
enum Command {
  /// Initialize MPK
  Init,
  /// Interact with sessions
  Sesh {
    #[clap(subcommand)]
    cmd: SeshCmd,
  },
  /// Play an audio file
  Play {
    file: Option<PathBuf>,
    #[clap(long, short)]
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
    ty: Option<AudioType>,
    query: Option<QueryFor>,
    by: Option<String>,
    #[clap(long)]
    id: Option<u64>,
    #[clap(long)]
    path: Option<PathBuf>,
    #[clap(long)]
    title: Option<String>,
    #[clap(long)]
    artist: Option<String>,
    #[clap(long)]
    album: Option<String>,
    #[clap(long)]
    genre: Option<String>,
    #[clap(long)]
    date: Option<String>,
    #[clap(long)]
    sr: Option<u32>,
    #[clap(long)]
    bpm: Option<f64>,
    #[clap(long)]
    label: Option<String>,
    #[clap(short, long)]
    raw: Option<String>,
    #[clap(long)]
    csv: bool,
  },
  /// Sync resources with DB
  Sync {
    #[clap(short, long)]
    tracks: bool,
    #[clap(short, long)]
    samples: bool,
    #[clap(short, long)]
    projects: bool,
    #[clap(short, long)]
    ext: bool,
    #[clap(short, long)]
    force: bool,
    #[clap(short, long)]
    desc: Option<Vec<String>>,
    #[clap(long)]
    path: Option<PathBuf>,
  },
  Backup {
    output: PathBuf,
  },
  Restore {
    input: PathBuf,
  },
}

#[derive(Subcommand)]
pub enum SeshCmd {
  New {
    name: String,
  },
  List,
  Abort,
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
    Command::Info {
      mut audio,
      mut midi,
      mut db,
    } => {
      if !(audio || midi || db) {
        audio = true;
        midi = true;
        db = true;
      }
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
        println!("sqlite_version: {}", db.version());
        println!("{} tracks", ts);
        println!("{} samples", ss);
      }
    }
    Command::Play {
      file,
      query,
      volume,
      speed,
      device,
    } => {
      let device = if let Some(d) = device {
        mpk_audio::device_from_str(d.as_str())
      } else {
        None
      };
      let file: Result<String> = if let Some(f) = file {
        Ok(f.to_str().unwrap().into())
      } else if let Some(q) = query {
        let db = Mdb::new_with_config(cfg.db)?;
        let path = db.query_track(q.parse().unwrap())?.path;
        let info = db.query_track_tags(q.parse().unwrap())?;
        println!("playing {} - {}", info.artist.unwrap(), info.title.unwrap());
        Ok(path)
      } else {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound).into())
      };
      let rx = mpk_audio::pause_controller_cli();
      mpk_audio::play(file.unwrap(), &device, volume, speed, rx)
    }

    Command::Db { cmd } => {
      let mut conn = Mdb::new_with_config(cfg.db)?;
      if args.trace {
        conn.set_tracer(Some(|x| println!("{}", x)))
      }
      if args.profile {
        conn.set_profiler(Some(|x, y| println!("{} -- {}ms", x, y.as_millis())))
      }

      match cmd {
        DbCmd::Query {
          ty,
          query,
          by,
          id,
          path,
          title,
          artist,
          album,
          genre,
          date,
          sr,
          bpm,
          label,
          raw,
          csv,
        } => {
          let by: Option<QueryBy> = if let Some(n) = id {
            match by.map(|b| QueryType::from_str(&b).unwrap()) {
              Some(QueryType::GreaterThan) => Some(QueryBy::IdGreater(n)),
              Some(QueryType::LessThan) => Some(QueryBy::IdLess(n)),
              _ => Some(QueryBy::Id(n)),
            }
          } else if let Some(p) = path {
            match by.map(|b| QueryType::from_str(&b).unwrap()) {
              Some(QueryType::Like) => Some(QueryBy::PathLike(p)),
              _ => Some(QueryBy::Path(p)),
            }
          } else if let Some(s) = title {
            match by.map(|b| QueryType::from_str(&b).unwrap()) {
              Some(QueryType::Like) => Some(QueryBy::TitleLike(s)),
              _ => Some(QueryBy::Title(s)),
            }
          } else if let Some(s) = artist {
            match by.map(|b| QueryType::from_str(&b).unwrap()) {
              Some(QueryType::Like) => Some(QueryBy::ArtistLike(s)),
              _ => Some(QueryBy::Artist(s)),
            }
          } else if let Some(s) = album {
            match by.map(|b| QueryType::from_str(&b).unwrap()) {
              Some(QueryType::Like) => Some(QueryBy::AlbumLike(s)),
              _ => Some(QueryBy::Album(s)),
            }
          } else if let Some(s) = genre {
            match by.map(|b| QueryType::from_str(&b).unwrap()) {
              Some(QueryType::Like) => Some(QueryBy::GenreLike(s)),
              _ => Some(QueryBy::Genre(s)),
            }
          } else if let Some(d) = date {
            let d = NaiveDate::parse_from_str(&d, "%Y-%m-%d").unwrap();
            match by.map(|b| QueryType::from_str(&b).unwrap()) {
              Some(QueryType::GreaterThan) => Some(QueryBy::DateGreater(d)),
              Some(QueryType::LessThan) => Some(QueryBy::DateLess(d)),
              _ => Some(QueryBy::Date(d)),
            }
          } else if let Some(n) = sr {
            match by.map(|b| QueryType::from_str(&b).unwrap()) {
              _ => Some(QueryBy::SampleRate(n)),
            }
          } else if let Some(n) = bpm {
            match by.map(|b| QueryType::from_str(&b).unwrap()) {
              Some(QueryType::GreaterThan) => Some(QueryBy::BpmGreater(n)),
              Some(QueryType::LessThan) => Some(QueryBy::BpmLess(n)),
              _ => Some(QueryBy::Bpm(n)),
            }
          } else if let Some(s) = label {
            match by.map(|b| QueryType::from_str(&b).unwrap()) {
              Some(QueryType::Like) => Some(QueryBy::LabelLike(s)),
              _ => Some(QueryBy::Label(s)),
            }
          } else {
            None
          };

          if let Some(by) = by {
            let s = by
              .as_query(ty.unwrap_or_default(), query.unwrap_or_default())
              .unwrap();
            let q = conn.query::<DbValue>(
              ty.unwrap_or_default(),
              by,
              query.unwrap_or_default(),
            )?;
            if csv {
              mpk_db::print_csv(q);
            } else {
              println!("{}", s);
              println!("{:#?}", q);
            }
          }
          if let Some(q) = raw {
            println!("{}", conn.query_raw(&q)?);
          }
        }
        DbCmd::Sync {
          tracks,
          samples,
          projects,
          ext,
          force,
          desc,
          path,
        } => {
          let script = if let Some(p) = path {
            p
          } else {
            cfg.extractor.path.unwrap_or_default()
          };

          let descriptors = if let Some(d) = desc {
            d
          } else {
            cfg.extractor.descriptors
          };

          if tracks {
            let mut cmd = std::process::Command::new(&script);
            let tracks = cfg.fs.get_path("tracks")?;
            cmd.arg(tracks.to_str().unwrap());
            if ext {
              if let Some(v) = cfg.fs.get_ext_paths("tracks") {
                cmd.args(v);
              }
            }
            cmd.args(["-t", "track", "-d"]);
            cmd.args(&descriptors);
            if force {
              cmd.arg("--force");
            }
            cmd.status()?;
          }
          if samples {
            let mut cmd = std::process::Command::new(&script);
            let samps = cfg.fs.get_path("samples")?;
            cmd.arg(samps.to_str().unwrap());
            if ext {
              if let Some(v) = cfg.fs.get_ext_paths("samples") {
                cmd.args(v);
              }
            }
            cmd.args(["-t", "sample", "-d"]);
            cmd.args(&descriptors);
            if force {
              cmd.arg("--force");
            }
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
    },
    Command::Sesh { cmd } => {
      let mut client = mpk_osc::nsm::NsmClient::new(
	"mpk",
	"127.0.0.1:0",
	None,
	mpk_osc::nsm::ClientCaps::all()).unwrap();
      match cmd {
	SeshCmd::New { name } => {
	  client.new_project(&name).unwrap()
	},
	SeshCmd::List => {
	  client.list().unwrap()
	}
	SeshCmd::Abort => {
	  client.abort().unwrap()
	},
      }
    },
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
  }

  Ok(())
}
