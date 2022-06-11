use rustyline::{Editor, Helper};
use tokio::sync::mpsc;

use crate::parser::parse;
use crate::parser::Prog;

pub const CH_LEN: usize = 32;

#[derive(Debug)]
pub struct Repl<H: Helper> {
  rl: Editor<H>,
  tx: mpsc::Sender<Prog>,
  rx: mpsc::Receiver<String>,
}

impl<H> Repl<H>
where
  H: Helper,
{
  pub fn new(rl: Editor<H>) -> (Repl<H>, mpsc::Receiver<Prog>, mpsc::Sender<String>) {
    let (tx, d_rx) = mpsc::channel(CH_LEN);
    let (d_tx, rx) = mpsc::channel(1);
    (Repl { rl, tx, rx }, d_rx, d_tx)
  }

  /// Parse a line from stdin and send it over a channel for dispatch.
  pub async fn parse(&mut self, debug: bool) {
    while let Ok(line) = self.rl.readline("|| ") {
      match parse(line.as_str()) {
        Ok(prog) => {
          self.rl.add_history_entry(line);
          if debug {
            println!("{:?}", prog)
          }
          self.tx(prog).await;
          self.rx().await;
        }
        Err(e) => {
          println!("{:?}", e)
        }
      }
    }
  }

  pub async fn tx(&self, p: Prog) {
    self.tx.send(p).await.unwrap()
  }

  pub async fn rx(&mut self) {
    println!("{}", self.rx.recv().await.unwrap().as_str());
  }
}
