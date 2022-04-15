use rodio::{Decoder, Source};
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::{Duration, Instant};

pub fn calc_beat_delta(bpm: u16, lower: u8) -> Duration {
  let quarter_note_sec: f64 = 60f64 / bpm as f64;
  let factor: f64 = 4f64 / lower as f64;

  Duration::from_secs_f64(quarter_note_sec * factor)
}

#[derive(Debug)]
pub enum TicToc<'a> {
  Tic(&'a Path),
  Toc(&'a Path),
}

#[derive(Debug, Copy, Clone)]
pub struct TimeSig {
  pub upper: u8,
  pub lower: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct MetroCfg {
  pub bpm: u16,
  pub time_sig: TimeSig,
}

#[derive(Debug)]
pub enum MetroMsg {
  Stop,
}

#[derive(Debug, Copy, Clone)]
pub struct Metro {
  cfg: MetroCfg,
  current_beat: u8,
  last_time_run: Instant,
  beat_delta: Duration,
}

impl Metro {
  pub fn new(bpm: u16, upper: u8, lower: u8) -> Metro {
    Metro {
      cfg: MetroCfg {
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
    self.last_time_run = Instant::now();
  }

  // TODO calculate duration of audio sample AOT, subtract from sleep
  // time. current bpm isn't right.
  pub fn play(self, t: TicToc) {
    let now = Instant::now();
    let st = match t {
      TicToc::Tic(p) => p,
      TicToc::Toc(p) => p,
    };
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let src = Decoder::new(BufReader::new(std::fs::File::open(st).unwrap()))
      .unwrap()
      .take_duration(self.beat_delta);
    eprintln!("metro: {}", self.current_beat + 1);
    sink.append(src);
    sink.play();
    sink.detach();
    std::thread::sleep(self.beat_delta - now.elapsed());
  }

  pub fn start<P: 'static + AsRef<Path> + std::marker::Send>(
    mut self,
    tic: P,
    toc: P,
  ) -> Sender<MetroMsg> {
    eprintln!("Metro started!");
    let (tx, rx) = channel();
    thread::spawn(move || loop {
      let msg = rx.try_recv();
      match msg {
        Ok(MetroMsg::Stop) => break,
        Err(_) => (),
      }
      match self.current_beat {
        0 => self.play(TicToc::Tic(tic.as_ref())),
        _ => self.play(TicToc::Toc(toc.as_ref())),
      }
      self.next()
    });
    tx
  }
}
