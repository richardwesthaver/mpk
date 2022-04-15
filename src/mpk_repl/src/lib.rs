use rustyline::{
  completion::FilenameCompleter, highlight::MatchingBracketHighlighter,
  hint::HistoryHinter, validate::MatchingBracketValidator, CompletionType, Config,
  EditMode, Editor, Helper,
};

use rustyline::ExternalPrinter;

mod err;
pub use err::{Error, Result};

mod sesh;
pub use sesh::SeshHelper;

pub fn init_sesh_repl() -> Result<Editor<SeshHelper>> {
  let config = Config::builder()
    .completion_type(CompletionType::Circular)
    .edit_mode(EditMode::Emacs)
    .build();
  let h = SeshHelper {
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

pub fn run_repl<H: Helper>(rl: &mut Editor<H>) -> Result<()> {
  loop {
    let readline = rl.readline("|| ");
    match readline {
      Ok(line) => {
        rl.add_history_entry(line.as_str()); // writes to mem buffer (i think?)
        println!("{}", line);
      }
      Err(rustyline::error::ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break;
      }
      Err(rustyline::error::ReadlineError::Eof) => {
        println!("CTRL-D");
        break;
      }
      Err(err) => {
        println!("Error: {:?}", err);
        break;
      }
    }
  }
  Ok(())
}

pub fn print_external<T: ExternalPrinter>(mut printer: T, msg: String) {
  printer.print(msg).expect("failed to print remotely")
}

#[cfg(test)]
mod tests {
  use super::*;
}
