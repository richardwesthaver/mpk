use crate::parser::ast::AstNode;
use crate::parser::parse;
use rustyline::{Editor, Helper};
use tokio::sync::mpsc;

pub fn split_eval(size: usize) -> (mpsc::Sender<AstNode>, mpsc::Receiver<AstNode>) {
  mpsc::channel(size)
}

#[derive(Debug)]
pub struct Evaluator<H: Helper> {
  rl: Editor<H>,
  tx: mpsc::Sender<AstNode>,
}

impl<H> Evaluator<H>
where
  H: Helper,
{
  pub fn new(rl: Editor<H>, tx: mpsc::Sender<AstNode>) -> Self {
    Evaluator { rl, tx }
  }

  pub async fn parse(&mut self, debug: bool) {
    while let Ok(line) = self.rl.readline("|| ") {
      match parse(line.as_str()) {
        Ok(prog) => {
          if debug {
            println!("{:?}", prog)
          }
          for n in prog {
            match n {
              AstNode::SysOp { verb: _, expr: _ } => {
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
