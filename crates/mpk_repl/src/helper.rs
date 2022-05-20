//! MPK_REPL SESH HELPER
use rustyline::{
  Context,
  hint::{Hint, Hinter},
  completion::FilenameCompleter,
  highlight::MatchingBracketHighlighter,
  validate::{MatchingBracketValidator, ValidationResult, ValidationContext, Validator},
};
use rustyline_derive::{Completer, Helper, Highlighter, Validator};
use std::collections::HashSet;

#[derive(Completer, Highlighter, Helper)]
pub struct ReplHelper {
  pub completer: FilenameCompleter,
  pub highlighter: MatchingBracketHighlighter,
  pub validator: MatchingBracketValidator,
  pub hinter: RlHinter,
  pub colored_prompt: String,
}

impl Validator for ReplHelper {
  fn validate(
    &self,
    ctx: &mut ValidationContext,
  ) -> rustyline::Result<ValidationResult> {
    self.validator.validate(ctx)
  }

  fn validate_while_typing(&self) -> bool {
    self.validator.validate_while_typing()
  }
}

impl Hinter for ReplHelper {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.hinter.hints
            .iter()
            .filter_map(|hint| {
                // expect hint after word complete, like redis cli, add condition:
                // line.ends_with(" ")
                if hint.display.starts_with(line) {
                    Some(hint.suffix(pos))
                } else {
                    None
                }
            })
            .next()
    }
}

#[derive(Completer, Validator, Highlighter)]
pub struct RlHinter {
    // for better efficiency, please use ** radix trie **
    pub hints: HashSet<CommandHint>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct CommandHint {
    display: String,
    complete_up_to: usize,
}

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display[..self.complete_up_to])
        } else {
            None
        }
    }
}

impl CommandHint {
    fn new(text: &str, complete_up_to: &str) -> CommandHint {
        assert!(text.starts_with(complete_up_to));
        CommandHint {
            display: text.into(),
            complete_up_to: complete_up_to.len(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}


pub fn hints() -> HashSet<CommandHint> {
    let mut set = HashSet::new();
    set.insert(CommandHint::new("\\http", "\\ht"));
    set.insert(CommandHint::new("\\h", "\\h"));
    set.insert(CommandHint::new("\\osc", "\\o"));
    set.insert(CommandHint::new("\\db", "\\d"));
    set
}
