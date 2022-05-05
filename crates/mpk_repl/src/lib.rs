//! MPK_REPL
use rustyline::ExternalPrinter;
use rustyline::{
  completion::FilenameCompleter, highlight::MatchingBracketHighlighter,
  hint::HistoryHinter, validate::MatchingBracketValidator, CompletionType, Config,
  EditMode, Editor,
};

pub use mpk_parser as parser;

mod err;
pub use err::{Error, Result};

mod helper;
pub use helper::ReplHelper;

mod dispatch;
pub use dispatch::Dispatcher;

mod eval;
pub use eval::Evaluator;

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

pub fn print_external<'a, T: ExternalPrinter>(printer: &'a mut T, msg: &'a str) {
  printer
    .print(msg.to_string())
    .expect("failed to print remotely")
}
