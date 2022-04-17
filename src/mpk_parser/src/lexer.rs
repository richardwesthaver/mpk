use std::char;
use crate::Location;
use crate::Tok;

pub type Spanned = (Location, Tok, Location);

pub struct Lexer<T: Iterator<Item =  char>> {
  chars: T,
  bol: bool,
  nesting: usize,
  pending: Vec<Spanned>,
  chr0: Option<char>,
  chr1: Option<char>,
  chr2: Option<char>,
  cursor: Location,
}

impl<T> Lexer<T>
where
  T: Iterator<Item = char>,
{
  pub fn new(input: T, start: Location) -> Self {
    let mut lxr = Lexer {
      chars: input,
      bol: true,
      nesting: 0,
      pending: Vec::new(),
      chr0: None,
      cursor: start,
      chr1: None,
      chr2: None,
        };
        lxr.next_char();
        lxr.next_char();
        lxr.next_char();
        // Start at top row (=1) left column (=1)
        lxr.cursor.reset();
        lxr    
  }

  /// Helper function to go to the next character coming up.
  fn next_char(&mut self) -> Option<char> {
    let c = self.chr0;
    let nxt = self.chars.next();
    self.chr0 = self.chr1;
    self.chr1 = self.chr2;
    self.chr2 = nxt;
    if c == Some('\n') {
      self.cursor.newline();
    } else {
      self.cursor.go_right();
    }
    c
  }
}
