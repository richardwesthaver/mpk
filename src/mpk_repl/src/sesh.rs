use rustyline::{
  completion::FilenameCompleter,
  highlight::MatchingBracketHighlighter,
  hint::HistoryHinter,
  validate::{self, MatchingBracketValidator, Validator},
};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};

#[derive(Completer, Helper, Highlighter, Hinter)]
pub struct SeshHelper {
  pub completer: FilenameCompleter,
  pub highlighter: MatchingBracketHighlighter,
  pub validator: MatchingBracketValidator,
  pub hinter: HistoryHinter,
  pub colored_prompt: String,
}

impl Validator for SeshHelper {
  fn validate(
    &self,
    ctx: &mut validate::ValidationContext,
  ) -> rustyline::Result<validate::ValidationResult> {
    self.validator.validate(ctx)
  }

  fn validate_while_typing(&self) -> bool {
    self.validator.validate_while_typing()
  }
}
