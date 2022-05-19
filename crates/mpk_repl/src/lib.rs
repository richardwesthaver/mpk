//! MPK_REPL
pub use mpk_parser as parser;
use rustyline::ExternalPrinter;
use rustyline::{
  completion::FilenameCompleter, highlight::MatchingBracketHighlighter,
  hint::HistoryHinter, validate::MatchingBracketValidator, CompletionType, Config,
  EditMode, Editor,
};
use mpk_parser::ast::AstNode;
use tokio::sync::mpsc::Receiver;

mod err;
pub use err::{Error, Result};

mod helper;
pub use helper::ReplHelper;

mod dispatch;
pub use dispatch::Dispatcher;

mod eval;
pub use eval::Repl;

pub fn init_repl() -> Result<Editor<ReplHelper>> {
  let config = Config::builder()
    .completion_type(CompletionType::Circular)
    .edit_mode(EditMode::Emacs)
    .build();
  let h = ReplHelper {
    completer: FilenameCompleter::new(),
    highlighter: MatchingBracketHighlighter::new(),
    hinter: HistoryHinter {},
    colored_prompt: "|| ".to_owned(),
    validator: MatchingBracketValidator::new(),
  };
  let mut rl = Editor::with_config(config);
  rl.set_helper(Some(h));
  Ok(rl)
}

pub async fn init_dispatcher<T: ExternalPrinter>(printer: T, client: &str, server: &str, rx: Receiver<Vec<AstNode>>) -> Result<Dispatcher<T>> {
  Ok(Dispatcher::new(printer, client, server, rx).await)
}

pub fn print_external<'a, T: ExternalPrinter>(printer: &'a mut T, msg: &'a str) {
  printer
    .print(msg.to_string())
    .expect("failed to print remotely")
}
