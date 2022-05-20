//! MPK_REPL
pub use mpk_parser as parser;
use rustyline::ExternalPrinter;
use rustyline::{
  completion::FilenameCompleter, highlight::MatchingBracketHighlighter,
  validate::MatchingBracketValidator, CompletionType, Config,
  EditMode, Editor,
};
use mpk_parser::ast::AstNode;
use tokio::sync::mpsc::Receiver;

mod err;
pub use err::{Error, Result};

mod helper;
pub use helper::*;

mod dispatch;
pub use dispatch::Dispatcher;

mod eval;
pub use eval::Repl;

pub async fn exec(client: &str, server: &str, timeout: u64, debug: bool) -> Result<()> {
  let mut rl = init_repl().unwrap();
  let printer = rl.create_external_printer().unwrap();
  let (mut evaluator, rx) = Repl::new(rl);
  let mut d = init_dispatcher(
    printer,
    client,
    server,
    rx,
    timeout,
  ).await.unwrap();
  let disp = tokio::spawn(async move {
    d.run().await;
  });
  evaluator.parse(debug).await;
  disp.abort();
  Ok(())
}

pub fn init_repl() -> Result<Editor<ReplHelper>> {
  let config = Config::builder()
    .completion_type(CompletionType::Circular)
    .edit_mode(EditMode::Emacs)
    .build();
  let h = ReplHelper {
    completer: FilenameCompleter::new(),
    highlighter: MatchingBracketHighlighter::new(),
    hinter: RlHinter { hints: hints() },
    colored_prompt: "|| ".to_owned(),
    validator: MatchingBracketValidator::new(),
  };
  let mut rl = Editor::with_config(config);
  rl.set_helper(Some(h));
  Ok(rl)
}

pub async fn init_dispatcher<T: ExternalPrinter>(printer: T, client: &str, server: &str, rx: Receiver<Vec<AstNode>>, timeout: u64) -> Result<Dispatcher<T>> {
  Ok(Dispatcher::new(printer, client, server, rx, timeout).await)
}

pub fn print_external<'a, T: ExternalPrinter>(printer: &'a mut T, msg: &'a str) {
  printer
    .print(msg.to_string())
    .expect("failed to print remotely")
}
