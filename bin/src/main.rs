use mpk::{Error,
	  midi,
	  osc,
	  http,
	  repl,
	  flate,
	  jack,
	  gear,
	  audio::{self, gen::SampleChain},
	  config::Config,
	  db::Db,
	  http::freesound::{write_sound, FreeSoundRequest, FreeSoundResponse},
	  util::expand_tilde};

use clap::{AppSettings, Parser, Subcommand};

use std::io;
use std::path::PathBuf;
use std::sync::mpsc::sync_channel;

#[derive(Parser)]
#[clap(name = "mpk")]
#[clap(about = "media programming kit")]
#[clap(author = "ellis <ellis@rwest.io>")]
#[clap(version = option_env!("CARGO_PKG_VERSION").unwrap_or("NULL"))]
/// MPK -- Media Programming Kit
///
/// Tools for building and managing creative workflows on UNIX systems.
struct Args {
  /// Command to execute
  #[clap(subcommand)]
  cmd: Command,
  /// Use specified config file
  #[clap(short,long, default_value_t = String::from("~/mpk/mpk.toml"))]
  cfg: String,
}

#[derive(Subcommand)]
#[clap(setting = AppSettings::DeriveDisplayOrder)]
enum Command {
  /// Initialize MPK
  Init,
  /// Sessions
  Sesh {
    #[clap(subcommand)]
    cmd: SeshCmd,
  },
  /// Web APIs
  Net {
    #[clap(subcommand)]
    cmd: NetCmd,
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
  /// Run services
  Run {
    #[clap(subcommand)]
    runner: Runner,
  },
  /// MPK DB
  Db {
    #[clap(subcommand)]
    cmd: DbCmd,
  },
  /// Start the REPL
  Repl,
  /// Print info
  Status {
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
  /// start the metronome
  Metro {
    bpm: Option<u16>,
    time_sig: Option<String>,
  },
  /// Monitor MIDI messages from input
  Monitor {
    input: Option<usize>,
  },
}

#[derive(Subcommand)]
enum DbCmd {
}

#[derive(Subcommand)]
pub enum SeshCmd {
  /// Create a new session
  New { name: String },
  /// Add a client to current session
  Add { exe: String },
  /// Open a session
  Open { name: String },
  /// Save current session
  Save,
  /// Duplicate current session
  Duplicate { name: String },
  /// Register client with nsmd
  Announce,
  /// List sessions
  List,
  /// Close current session
  Close,
  /// Close current session without saving
  Abort,
  /// Close current session and nsmd
  Quit,
}

#[derive(Subcommand)]
pub enum NetCmd {
  /// Freesound API Client
  Freesound {
    /// API command
    cmd: String,
    /// Automatically open browser during authentication
    #[clap(short, long)]
    auto: bool,
    /// Query for API requests
    #[clap(short, long)]
    query: Option<String>,
    /// Output path for downloads
    #[clap(short, long)]
    out: Option<PathBuf>,
  },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let args = Args::parse();
  let cfg_path = expand_tilde(&args.cfg).unwrap();
  let mut cfg = if cfg_path.exists() {
    Config::load(&cfg_path)?
  } else {
    Config::default()
  };

  match args.cmd {
    Command::Init => {
      print!("Initializing MPK... ");
      cfg.build()?;
      cfg.write(cfg_path)?;
      let db_path = cfg.db.path;
      Db::open(expand_tilde(db_path))?;
      println!("\x1b[1;32mDONE\x1b[0m");
    }
    Command::Status {
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
        audio::info();
      }
      if midi {
        println!("\x1b[1mMIDI INFO\x1b[0m");
        midi::list_midi_ports()?;
      }
      if db {
        println!("\x1b[1mDB INFO\x1b[0m");
        let db = Db::with_config(cfg.db)?;
	db.info()?;
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
        audio::device_from_str(d.as_str())
      } else {
        None
      };
      let file: Result<String, Error> = if let Some(f) = file {
        Ok(f.to_str().unwrap().into())
      } // else if let Some(q) = query {
//        let db = Db::with_config(cfg.db)?;
//        let path = db.query_track(q.parse().unwrap())?.path;
//        let info = db.query_track_tags(q.parse().unwrap())?;
//        println!("playing {} - {}", info.artist.unwrap(), info.title.unwrap());
//        Ok(path)
//    }
      else {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound).into())
      };
      let rx = audio::pause_controller_cli();
      audio::play(file.unwrap(), &device, volume, speed, rx)
    }

    Command::Db {
      cmd: _,
    } => {
      let _conn = Db::with_config(cfg.db)?;
    }
    Command::Sesh { cmd } => {
      let mut client = osc::nsm::NsmClient::new(
        "mpk",
        "127.0.0.1:0",
        None,
        osc::nsm::ClientCaps::all(),
      )
      .unwrap();
      match cmd {
        SeshCmd::New { name } => client.new_project(&name).unwrap(),
        SeshCmd::Add { exe } => client.add(&exe).unwrap(),
        SeshCmd::Open { name } => client.open(&name).unwrap(),
        SeshCmd::Save => client.save().unwrap(),
        SeshCmd::Duplicate { name } => client.duplicate(&name).unwrap(),
        SeshCmd::List => {
          let ps = client.list().unwrap();
          for l in ps {
            println!("{}", l);
          }
        }
        SeshCmd::Close => client.close().unwrap(),
        SeshCmd::Abort => client.abort().unwrap(),
        SeshCmd::Quit => client.quit().unwrap(),
        SeshCmd::Announce => {
          let res = client.announce().unwrap();
          println!(
            "opening project ::\n  name: {}\n  path: {}\n  client_id: {}",
            res.0.to_str().unwrap(),
            res.1,
            res.2
          );
        }
      }
    }
    Command::Net { cmd } => match cmd {
      NetCmd::Freesound {
        cmd,
        auto,
        query,
        out,
      } => {
        tokio::spawn(async move {
          let mut client = http::freesound::FreeSoundClient::new_with_config(
            cfg.net.freesound.as_ref().unwrap(),
          );
          if cmd.eq("auth") {
            client.auth(auto).await.unwrap();
            client.save_to_config(&mut cfg);
            cfg.write(cfg_path).unwrap();
          } else if cmd.eq("search") {
            let req = FreeSoundRequest::SearchText {
              query: &query.unwrap(),
              filter: "tag:guitar",
              sort: "",
              group_by_pack: false,
              weights: "",
              page: 1,
              page_size: 150,
              fields: &["id", "name"],
              descriptors: &[],
              normalized: false,
            };
            let res = client.request(req).await.unwrap();
            let response = FreeSoundResponse::parse(res).await;
            println!("{}", response);
          } else if cmd.eq("raw") {
            let res = client
              .get_raw(query.unwrap())
              .await
              .unwrap()
              .text()
              .await
              .unwrap();
            println!("{}", res);
          } else if cmd.eq("dl") || cmd.eq("download") {
            let query = query.unwrap();
            let out = if let Some(p) = out {
              p
            } else {
              let mut path = cfg.fs.get_path("samples").unwrap();
              path.push(&query);
              path
            };
            let req = FreeSoundRequest::SoundDownload {
              id: query.parse().unwrap(),
            };
            let res = client.request(req).await.unwrap();
            write_sound(res, &out, true).await.unwrap();
            println!("sound_id {} downloaded to {}", query, out.to_str().unwrap());
          }
        })
        .await
        .unwrap();
      }
    },
    Command::Repl => {
      let mut rl = repl::init_repl().unwrap();
      let printer = rl.create_external_printer().unwrap();
      let (mut evaluator, rx) = repl::Evaluator::new(rl);
      let disp = tokio::spawn(async move {
        let mut dispatcher =
          repl::Dispatcher::new(printer, "127.0.0.1:0", "127.0.0.1:57813", rx)
            .await;
        dispatcher.run().await;
      });
      evaluator.parse(true).await;
      disp.abort();
      std::process::exit(0);
    }
    Command::Pack {
      input,
      output,
      level,
    } => flate::pack(input, output, level.map(|x| x.into())),
    Command::Unpack {
      input,
      output,
      replace,
    } => {
      if replace {
        flate::unpack_replace(input, output)
      } else {
        flate::unpack(input, output)
      }
    }
    Command::Run { runner } => match runner {
      Runner::Jack { name } => {
        // Wait for user input to quit
        let (tx, rx) = sync_channel(1);
        println!("Press enter to quit...");
        tokio::spawn(async move {
          let mut input = String::new();
          io::stdin().read_line(&mut input).ok();
          tx.send(()).unwrap();
        });
        jack::internal_client(&name, rx);
      }
      Runner::Chain {
        input,
        output,
        even,
        ot,
      } => {
        let mut chain = SampleChain {
          output_file: output.with_extension(""),
          output_ext: output
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap(),
          ..Default::default()
        };
        for i in &input {
          chain.add_file(i)?;
        }
        for i in &input {
          chain.process_file(i, even)?;
        }
        if ot {
          gear::octatrack::generate_ot_file(&mut chain)?;
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
              t.trim().split('/').map(|x| x.parse().unwrap()).collect();
            (tsig[0], tsig[1])
          }
          None => cfg.metro.time_sig,
        };

        let metro = audio::gen::Metro::new(bpm, sig.0, sig.1);
        let tx = metro.start(cfg.metro.tic.unwrap(), cfg.metro.toc.unwrap());
        println!("Press enter to stop...");
        tokio::spawn(async move {
          let mut input = String::new();
          io::stdin().read_line(&mut input).unwrap();
          tx.send(audio::gen::metro::MetroMsg::Stop).unwrap();
          std::process::exit(1);
        });
      },
      Runner::Monitor { input } => midi::monitor(input)?,
    },
  }

  Ok(())
}
