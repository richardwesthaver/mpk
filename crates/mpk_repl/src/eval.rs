use rustyline::{Editor, Helper};
use tokio::sync::mpsc;

use crate::parser::ast::AstNode;
use crate::parser::parse;

pub const CH_LEN: usize = 32;

#[derive(Debug)]
pub struct Evaluator<H: Helper> {
  rl: Editor<H>,
  tx: mpsc::Sender<AstNode>,
}

impl<H> Evaluator<H>
where
  H: Helper,
{
  pub fn new(rl: Editor<H>) -> (Self, mpsc::Receiver<AstNode>) {
    let (tx, rx) = mpsc::channel(CH_LEN);
    (Evaluator { rl, tx }, rx)
  }

  pub async fn parse(&mut self, debug: bool) {
    while let Ok(line) = self.rl.readline("|| ") {
      match parse(line.as_str()) {
        Ok(prog) => {
          self.rl.add_history_entry(line);
          if debug {
            println!("{:?}", prog)
          }
          for n in prog {
            match n {
              AstNode::SysFn { verb: _, args: _ } => {
                self.tx(n).await;
              }
              _ => (),
            }
          }
        }
        Err(e) => {
          println!("{:?}", e)
        }
      }
    }
  }

  pub async fn tx(&self, node: AstNode) {
    self.tx.send(node).await.unwrap()
  }
}
