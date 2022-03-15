use std::io::BufReader;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::{Duration, Instant};

pub fn calc_beat_delta(bpm: u16, lower: u8) -> Duration {
  let quarter_note_sec: f64 = 60f64 / bpm as f64;
  let factor: f64 = 4f64 / lower as f64;

  Duration::from_secs_f64(quarter_note_sec * factor)
}

#[derive(Debug)]
pub enum TicToc {
  Tic,
  Toc,
}

#[derive(Debug, Copy, Clone)]
pub struct TimeSig {
  pub upper: u8,
  pub lower: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct MetroConfig {
  pub bpm: u16,
  pub time_sig: TimeSig,
}

#[derive(Debug)]
pub enum MetroMsg {
  Stop,
}

#[derive(Debug, Copy, Clone)]
pub struct Metro {
  cfg: MetroConfig,
  current_beat: u8,
  last_time_run: Instant,
  beat_delta: Duration,
}

impl Metro {
  pub fn new(bpm: u16, upper: u8, lower: u8) -> Metro {
    Metro {
      cfg: MetroConfig {
        bpm,
        time_sig: TimeSig { upper, lower },
      },
      current_beat: 0,
      last_time_run: Instant::now(),
      beat_delta: calc_beat_delta(bpm, lower),
    }
  }
  pub fn next(&mut self) {
    self.current_beat = (self.current_beat + 1) % self.cfg.time_sig.upper;
    self.last_time_run = std::time::Instant::now();
  }

  pub fn play(self, t: TicToc) {
    let st = match t {
      TicToc::Tic => "tic.wav",
      TicToc::Toc => "toc.wav",
    };
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = std::fs::File::open(st).unwrap();
    eprintln!("metro: {}", self.current_beat + 1);
    stream_handle
      .play_once(BufReader::new(file))
      .unwrap()
      .detach();
    std::thread::sleep(self.beat_delta);
  }

  pub fn start(mut self) -> Sender<MetroMsg> {
    eprintln!("Metro started!");
    let (tx, rx) = channel();
    thread::spawn(move || loop {
      let msg = rx.recv();
      match msg {
        Ok(MetroMsg::Stop) => break,
        Err(_) => (),
      }
      match self.current_beat {
        0 => self.play(TicToc::Tic),
        _ => self.play(TicToc::Toc),
      }
      self.next()
    });
    tx
  }
}
