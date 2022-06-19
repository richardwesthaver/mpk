//! MPK_PARSER -- AST
//!
//! Abstract Syntax Tree objects of the mk language.
use std::iter::Iterator;
use std::time::Duration;

use mpk_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Debug, Clone, Serialize, Deserialize)]
pub enum Atom {
  Boolean(bool),
  Char(char),
  Int(Integer),
  Float(Float),
  Symbol(String),
  Time(Duration),
}

impl From<bool> for Atom {
  fn from(b: bool) -> Self {
    Atom::Boolean(b)
  }
}

impl From<char> for Atom {
  fn from(c: char) -> Self {
    Atom::Char(c)
  }
}

impl From<Integer> for Atom {
  fn from(i: Integer) -> Self {
    Atom::Int(i)
  }
}

impl From<Float> for Atom {
  fn from(f: Float) -> Self {
    Atom::Float(f)
  }
}

impl From<String> for Atom {
  fn from(s: String) -> Self {
    Atom::Symbol(s)
  }
}

impl From<Duration> for Atom {
  fn from(d: Duration) -> Self {
    Atom::Time(d)
  }
}

impl std::fmt::Display for Atom {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Atom::Boolean(x) => {
        if *x {
          f.write_str("1b")
        } else {
          f.write_str("0b")
        }
      }
      Atom::Char(x) => x.fmt(f),
      Atom::Int(x) => x.fmt(f),
      Atom::Float(x) => x.fmt(f),
      Atom::Symbol(x) => x.fmt(f),
      Atom::Time(x) => x.as_millis().fmt(f),
    }
  }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Float {
  E(f32),
  F(f64),
}

impl std::fmt::Display for Float {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Float::E(x) => x.fmt(f),
      Float::F(x) => x.fmt(f),
    }
  }
}

impl From<f32> for Float {
  fn from(e: f32) -> Self {
    Float::E(e)
  }
}

impl From<f64> for Float {
  fn from(f: f64) -> Self {
    Float::F(f)
  }
}

#[derive(
  PartialEq, PartialOrd, Debug, Clone, Copy, Serialize, Deserialize, Eq, Hash,
)]
pub enum Integer {
  G(u8),
  H(u16),
  I(u32),
  J(u64),
  K(u128),
}

impl From<char> for Integer {
  fn from(i: char) -> Self {
    Integer::G(i as u8)
  }
}

impl From<u8> for Integer {
  fn from(g: u8) -> Self {
    Integer::G(g)
  }
}

impl From<u16> for Integer {
  fn from(h: u16) -> Self {
    Integer::H(h)
  }
}

impl From<u32> for Integer {
  fn from(i: u32) -> Self {
    Integer::I(i)
  }
}

impl From<u64> for Integer {
  fn from(j: u64) -> Self {
    Integer::J(j)
  }
}

impl From<u128> for Integer {
  fn from(k: u128) -> Self {
    Integer::K(k)
  }
}

impl std::fmt::Display for Integer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Integer::G(x) => x.fmt(f),
      Integer::H(x) => x.fmt(f),
      Integer::I(x) => x.fmt(f),
      Integer::J(x) => x.fmt(f),
      Integer::K(x) => x.fmt(f),
    }
  }
}

impl From<Integer> for char {
  fn from(i: Integer) -> Self {
    match i {
      Integer::G(x) => x as char,
      Integer::H(x) => x as u8 as char,
      Integer::I(x) => x as u8 as char,
      Integer::J(x) => x as u8 as char,
      Integer::K(x) => x as u8 as char,
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum MonadicVerb {
  Flip,     // +
  Negate,   // -
  First,    // *
  Sqrt,     // %
  Enum,     // !
  Where,    // &
  Reverse,  // |
  Asc,      // <
  Desc,     // >
  Group,    // =
  Not,      // ~
  Enlist,   // ,
  Null,     // ^
  Count,    // #
  Floor,    // _
  String,   // $
  Distinct, // ?
  Type,     // @
  Eval,     // .
}

impl std::fmt::Display for MonadicVerb {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      MonadicVerb::Flip => f.write_str("+"),
      MonadicVerb::Negate => f.write_str("-"),
      MonadicVerb::First => f.write_str("*"),
      MonadicVerb::Sqrt => f.write_str("%"),
      MonadicVerb::Enum => f.write_str("!"),
      MonadicVerb::Where => f.write_str("&"),
      MonadicVerb::Reverse => f.write_str("|"),
      MonadicVerb::Asc => f.write_str("<"),
      MonadicVerb::Desc => f.write_str(">"),
      MonadicVerb::Group => f.write_str("="),
      MonadicVerb::Not => f.write_str("~"),
      MonadicVerb::Enlist => f.write_str(","),
      MonadicVerb::Null => f.write_str("^"),
      MonadicVerb::Count => f.write_str("#"),
      MonadicVerb::Floor => f.write_str("_"),
      MonadicVerb::String => f.write_str("$"),
      MonadicVerb::Distinct => f.write_str("?"),
      MonadicVerb::Type => f.write_str("@"),
      MonadicVerb::Eval => f.write_str("."),
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum DyadicVerb {
  Plus,   // +
  Minus,  // -
  Times,  // *
  Divide, // %
  Mod,    // !
  Min,    // &
  Max,    // |
  Less,   // <
  More,   // >
  Equal,  // =
  Match,  // ~
  Concat, // ,
  Except, // ^
  Take,   // #
  Drop,   // _
  Cast,   // $
  Find,   // ?
  At,     // @
  Dot,    // .
}

impl std::fmt::Display for DyadicVerb {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      DyadicVerb::Plus => f.write_str("+"),
      DyadicVerb::Minus => f.write_str("-"),
      DyadicVerb::Times => f.write_str("*"),
      DyadicVerb::Divide => f.write_str("%"),
      DyadicVerb::Mod => f.write_str("!"),
      DyadicVerb::Min => f.write_str("&"),
      DyadicVerb::Max => f.write_str("|"),
      DyadicVerb::Less => f.write_str("<"),
      DyadicVerb::More => f.write_str(">"),
      DyadicVerb::Equal => f.write_str("="),
      DyadicVerb::Match => f.write_str("~"),
      DyadicVerb::Concat => f.write_str(","),
      DyadicVerb::Except => f.write_str("^"),
      DyadicVerb::Take => f.write_str("#"),
      DyadicVerb::Drop => f.write_str("_"),
      DyadicVerb::Cast => f.write_str("$"),
      DyadicVerb::Find => f.write_str("?"),
      DyadicVerb::At => f.write_str("@"),
      DyadicVerb::Dot => f.write_str("."),
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum AdVerb {
  Each,      // '
  Over,      // /
  Scan,      // \
  EachPrior, // ':
  EachRight, // /:
  EachLeft,  // \:
}

impl std::fmt::Display for AdVerb {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      AdVerb::Each => f.write_str("'"),
      AdVerb::Over => f.write_str("/"),
      AdVerb::Scan => f.write_str("\\"),
      AdVerb::EachPrior => f.write_str("':"),
      AdVerb::EachRight => f.write_str("/:"),
      AdVerb::EachLeft => f.write_str("\\:"),
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum SysVerb {
  Sesh,   // \sesh
  Proxy,  // \proxy
  Db,     // \db
  Vars,   // \v
  Work,   // \w
  Import, // \l
  Timeit, // \t
  Exit,   // \\
}

impl std::fmt::Display for SysVerb {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      SysVerb::Sesh => f.write_str("\\sesh"),
      SysVerb::Proxy => f.write_str("\\proxy"),
      SysVerb::Db => f.write_str("\\db"),
      SysVerb::Vars => f.write_str("\\v"),
      SysVerb::Work => f.write_str("\\w"),
      SysVerb::Import => f.write_str("\\l"),
      SysVerb::Timeit => f.write_str("\\t"),
      SysVerb::Exit => f.write_str("\\\\"),
    }
  }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum AstNode {
  Atom(Atom),
  Str(String),
  List(Vec<AstNode>),
  Map(HashMap<String, AstNode>),
  Table(HashMap<String, AstNode>),
  Monad {
    verb: MonadicVerb,
    adverb: Option<AdVerb>,
    expr: Box<AstNode>,
  },
  Dyad {
    lhs: Box<AstNode>,
    verb: DyadicVerb,
    adverb: Option<AdVerb>,
    rhs: Box<AstNode>,
  },
  SysFn {
    verb: SysVerb,
    args: Option<Box<AstNode>>,
  },
  UserFn {
    args: Option<Vec<String>>,
    expr: Box<AstNode>,
  },
  FnCall {
    name: String,
    args: Option<Vec<AstNode>>,
  },
  Var {
    name: String,
    expr: Box<AstNode>,
  },
}

impl From<Vec<AstNode>> for AstNode {
  fn from(lst: Vec<AstNode>) -> Self {
    AstNode::List(lst)
  }
}

impl From<Atom> for AstNode {
  fn from(a: Atom) -> Self {
    AstNode::Atom(a)
  }
}

impl From<String> for AstNode {
  fn from(s: String) -> Self {
    AstNode::Str(s)
  }
}

impl std::fmt::Display for AstNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AstNode::Atom(x) => x.fmt(f),
      AstNode::Str(x) => x.fmt(f),
      AstNode::List(x) => f.write_fmt(format_args!(
        "{}",
        x.into_iter()
          .map(|l| l.to_string())
          .intersperse(" ".to_string())
          .collect::<String>()
      )),
      AstNode::Map(x) => {
        // no keys? no problem. max_klen=0
        let klen: usize = x
          .keys()
          .into_iter()
          .map(|k| k.len())
          .max()
          .unwrap_or_default();
        let mut str: Vec<String> = vec![];
        for (k, v) in x {
          str.push(format!("{:klen$}|{}", k, v.to_string()));
        }
        f.write_fmt(format_args!(
          "{}",
          str
            .into_iter()
            .intersperse("\n".to_string())
            .collect::<String>()
        ))
      }
      AstNode::Table(x) => {
        let klen: usize = x.keys().into_iter().map(|k| k.len()).max().unwrap();
        let mut str: Vec<String> = vec![];
        for (k, v) in x {
          str.push(format!("{:klen$}|{}", k, v.to_string()));
        }
        f.write_fmt(format_args!(
          "{}",
          str
            .into_iter()
            .intersperse("\n".to_string())
            .collect::<String>()
        ))
      }
      AstNode::Monad { verb, adverb, expr } => {
        if let Some(ad) = adverb {
          f.write_fmt(format_args!("{}{}{}", verb, ad, *expr))
        } else {
          f.write_fmt(format_args!("{}{}", verb, *expr))
        }
      }
      AstNode::Dyad {
        lhs,
        verb,
        adverb,
        rhs,
      } => {
        if let Some(ad) = adverb {
          f.write_fmt(format_args!("{}{}{}{}", *lhs, verb, ad, *rhs))
        } else {
          f.write_fmt(format_args!("{}{}{}", *lhs, verb, *rhs))
        }
      }
      AstNode::SysFn { verb, args } => {
        if let Some(ar) = args {
          f.write_fmt(format_args!("{} {}", verb, *ar))
        } else {
          verb.fmt(f)
        }
      }
      AstNode::UserFn { args, expr } => {
        if let Some(ar) = args {
          f.write_fmt(format_args!(
            "{{[{}]{}}}",
            ar.iter()
              .map(|a| a.to_string())
              .intersperse(";".to_string())
              .collect::<String>(),
            *expr
          ))
        } else {
          f.write_fmt(format_args!("{{{}}}", *expr))
        }
      }
      AstNode::FnCall { name, args } => {
        if let Some(ar) = args {
          f.write_fmt(format_args!(
            "{}[{}]",
            name,
            ar.iter()
              .map(|a| a.to_string())
              .intersperse(";".to_string())
              .collect::<String>()
          ))
        } else {
          f.write_str(&name.to_string())
        }
      }
      AstNode::Var { name, expr } => f.write_fmt(format_args!("{}:{}", name, *expr)),
    }
  }
}
