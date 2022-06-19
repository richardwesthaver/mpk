//! mpk_parser -- span
use core::ops::Range;
use std::fmt;
use pest::Span as PSpan;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Span {
  start: usize,
  end: usize,
}

impl Span {
  #[inline]
  pub const fn new(start: usize, end: usize) -> Self {
    Self { start, end }
  }

  #[inline]
  pub const fn double(span: usize) -> Self {
    Self {
      start: span,
      end: span,
    }
  }

  #[inline]
  pub const fn start(&self) -> usize {
    self.start
  }

  #[inline]
  pub const fn end(&self) -> usize {
    self.end
  }

  #[inline]
  pub const fn range(&self) -> Range<usize> {
    self.start..self.end
  }

  #[inline]
  pub const fn merge(start: Self, end: Self) -> Span {
    Self::new(start.start, end.end)
  }

  #[inline]
  pub const fn width(&self) -> usize {
    self.end - self.start
  }

  pub fn coalesce_span(spans: &[Span]) -> Span {
    let span = spans.get(0);
    if let Some(span) = span {
      let mut span = *span;
      for s in spans {
        if s.start() < span.start() {
          span = Span::new(s.start(), span.end());
        }
        if s.end() > span.end() {
          span = Span::new(s.start(), s.end());
        }
      }
      span
    } else {
      Span::new(0, 0)
    }
  }
}

impl fmt::Debug for Span {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}..{}", self.start, self.end)
  }
}

impl From<Range<usize>> for Span {
  #[inline]
  fn from(range: Range<usize>) -> Self {
    Self {
      start: range.start,
      end: range.end,
    }
  }
}

impl Into<Range<usize>> for Span {
  #[inline]
  fn into(self) -> Range<usize> {
    self.start..self.end
  }
}

impl From<(usize, usize)> for Span {
  #[inline]
  fn from(range: (usize, usize)) -> Self {
    Self {
      start: range.0,
      end: range.1,
    }
  }
}

impl Into<(usize, usize)> for Span {
  #[inline]
  fn into(self) -> (usize, usize) {
    (self.start, self.end)
  }
}

impl From<[usize; 2]> for Span {
  #[inline]
  fn from(range: [usize; 2]) -> Self {
    Self {
      start: range[0],
      end: range[1],
    }
  }
}

impl Into<[usize; 2]> for Span {
  #[inline]
  fn into(self) -> [usize; 2] {
    [self.start, self.end]
  }
}

impl From<PSpan<'_>> for Span {
  #[inline]
  fn from(s: PSpan) -> Self {
    Self {
      start: s.start(),
      end: s.end(),
    }
  }
}
