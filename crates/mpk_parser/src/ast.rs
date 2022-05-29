//! MPK_PARSER -- AST
//!
//! Abstract Syntax Tree objects of the mk language.
use std::fmt::Write;
use std::iter::{ExactSizeIterator, Iterator};
use std::ops::{Add, Deref, Sub};
use std::time::Duration;

use mpk_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};

use crate::EvalError;
pub type Program = Vec<AstNode>;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Atom {
  Boolean(bool),
  Char(Char),
  Int(Integer),
  Float(Float),
  Name(Name), // 8 char max
  Time(Duration),
}

impl Add for Atom {
  type Output = Atom;
  fn add(self, rhs: Self) -> Self::Output {
    use Atom::*;
    match (self, rhs) {
      (Boolean(x), Boolean(y)) => Boolean(x | y),
      (Char(x), Char(y)) => Char(x + y),
      (Char(x), Int(y)) => Int(Integer::G(*x) + y),
      (Int(x), Int(y)) => Int(x + y),
      (Float(x), Float(y)) => Float(x + y),
      (Name(x), Name(y)) => Name(x + y),
      (Time(x), Time(y)) => Time(x + y),
      _ => todo!(),
    }
  }
}

impl Sub for Atom {
  type Output = Atom;
  fn sub(self, rhs: Self) -> Self::Output {
    use Atom::*;
    match (self, rhs) {
      (Boolean(x), Boolean(y)) => Boolean(x ^ y),
      (Char(x), Char(y)) => Char(x - y),
      (Char(x), Int(y)) => Int(Integer::G(*x) - y),
      (Int(x), Int(y)) => Int(x - y),
      (Float(x), Float(y)) => Float(x - y),
      (Name(x), Name(y)) => Name(x - y),
      (Time(x), Time(y)) => Time(x - y),
      _ => todo!(),
    }
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
      Atom::Name(x) => x.fmt(f),
      Atom::Time(x) => x.as_millis().fmt(f),
    }
  }
}

#[derive(
  PartialEq, PartialOrd, Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq,
)]
pub enum Name {
  N1(u8),
  N2([u8; 2]),
  N4([u8; 4]),
  N8([u8; 8]),
}

impl Add for Name {
  type Output = Name;
  fn add(self, rhs: Self) -> Self::Output {
    use Name::*;
    match (self, rhs) {
      (N1(x), N1(y)) => N2([x, y]),
      (N1(x), N2(y)) => N4([x, y[0], y[1], 0]),
      (N2(x), N1(y)) => N4([x[0], x[1], y, 0]),
      (N2(x), N2(y)) => N4([x[0], x[1], y[0], y[1]]),
      (N1(x), N4(y)) => N8([x, y[0], y[1], y[2], y[3], 0, 0, 0]),
      (N2(x), N4(y)) => N8([x[0], x[1], y[0], y[1], y[2], y[3], 0, 0]),
      (N4(x), N4(y)) => N8([x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3]]),
      _ => todo!(),
    }
  }
}

impl Sub for Name {
  type Output = Name;
  fn sub(self, rhs: Self) -> Self::Output {
    use Name::*;
    match (self, rhs) {
      (N1(x), N1(y)) => N1(x - y),
      (N1(x), N2(y)) => N1(x - y[0]),
      (N1(x), N4(y)) => N1(x - y[0]),
      (N1(x), N8(y)) => N1(x - y[0]),
      (N2(x), N1(y)) => N2([x[0], x[1] - y]),
      (N2(x), N2(y)) => N2([x[0] - y[0], x[1] - y[1]]),
      (N2(x), N4(y)) => N2([x[0] - y[0], x[1] - y[1]]),
      (N2(x), N8(y)) => N2([x[0] - y[0], x[1] - y[1]]),
      (N4(x), N1(y)) => N4([x[0] - y, x[1], x[2], x[3]]),
      (N4(x), N2(y)) => N4([x[0] - y[0], x[1] - y[1], x[2], x[3]]),
      (N4(x), N4(y)) => N4([x[0] - y[0], x[1] - y[1], x[2] - y[2], x[3] - y[3]]),
      (N4(x), N8(y)) => N4([x[0] - y[0], x[1] - y[1], x[2] - y[2], x[3] - y[3]]),
      (N8(x), N1(y)) => N8([x[0] - y, x[1], x[2], x[3], x[4], x[5], x[6], x[7]]),
      (N8(x), N2(y)) => {
        N8([x[0] - y[0], x[1] - y[1], x[2], x[3], x[4], x[5], x[6], x[7]])
      }
      (N8(x), N4(y)) => N8([
        x[0] - y[0],
        x[1] - y[1],
        x[2] - y[2],
        x[3] - y[3],
        x[4],
        x[5],
        x[6],
        x[7],
      ]),
      (N8(x), N8(y)) => N8([
        x[0] - y[0],
        x[1] - y[1],
        x[2] - y[2],
        x[3] - y[3],
        x[4] - y[4],
        x[5] - y[5],
        x[6] - y[6],
        x[7] - y[7],
      ]),
    }
  }
}

impl From<Integer> for Name {
  fn from(i: Integer) -> Self {
    use Name::*;
    match i {
      Integer::G(x) => N1(x),
      Integer::H(x) => N2(x.to_be_bytes()),
      Integer::I(x) => N4(x.to_be_bytes()),
      Integer::J(x) => N8(x.to_be_bytes()),
    }
  }
}

impl<'a> From<&'a str> for Name {
  fn from(s: &'a str) -> Self {
    use Name::*;
    let s = s.as_bytes();
    match s.len() {
      1 => N1(s[0]),
      2 => N2([s[0], s[1]]),
      3 => N4([s[0], s[1], s[2], 0]),
      4 => N4([s[0], s[1], s[2], s[3]]),
      5 => N8([s[0], s[1], s[2], s[3], s[4], 0, 0, 0]),
      6 => N8([s[0], s[1], s[2], s[3], s[4], s[5], 0, 0]),
      7 => N8([s[0], s[1], s[2], s[3], s[4], s[5], s[6], 0]),
      8 => N8([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]),
      // if n.len>8, truncate
      _ => N8([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]),
    }
  }
}
impl std::fmt::Display for Name {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Name::N1(x) => f.write_str(&(*x as char).to_string()),
      Name::N2(x) => f.write_str(std::str::from_utf8(x).unwrap()), // no padding is possible so no trim
      Name::N4(x) => f.write_str(std::str::from_utf8(x.trim_ascii()).unwrap()),
      Name::N8(x) => f.write_str(std::str::from_utf8(x.trim_ascii()).unwrap()),
    }
  }
}

impl Iterator for Name {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    match self {
      // TODO test if this is necessary else just return None for N1
      Name::N1(x) => x.to_be_bytes().iter().next().map(|x| *x),
      Name::N2(x) => x.iter().next().map(|x| *x),
      Name::N4(x) => x.iter().next().map(|x| *x),
      Name::N8(x) => x.iter().next().map(|x| *x),
    }
  }
}

impl ExactSizeIterator for Name {
  fn len(&self) -> usize {
    match self {
      // TODO handle odd cases where 0-padding is added n.len=(3,5,6,7)
      Name::N1(_) => 1,
      Name::N2(_) => 2,
      Name::N4(_) => 4,
      Name::N8(_) => 8,
    }
  }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Float {
  E(f32),
  F(f64),
}

impl Add for Float {
  type Output = Float;
  fn add(self, rhs: Self) -> Self::Output {
    use Float::*;
    match (self, rhs) {
      (E(x), F(y)) => F(x as f64 + y),
      (F(x), E(y)) => F(x + y as f64),
      (F(x), F(y)) => F(x + y),
      (E(x), E(y)) => E(x + y),
    }
  }
}

impl Sub for Float {
  type Output = Float;
  fn sub(self, rhs: Self) -> Self::Output {
    use Float::*;
    match (self, rhs) {
      (E(x), F(y)) => F(x as f64 - y),
      (F(x), E(y)) => F(x - y as f64),
      (F(x), F(y)) => F(x - y),
      (E(x), E(y)) => E(x - y),
    }
  }
}

impl From<Integer> for Float {
  fn from(i: Integer) -> Self {
    match i {
      Integer::G(x) => Float::E(x as f32),
      Integer::H(x) => Float::E(x as f32),
      Integer::I(x) => Float::F(x as f64),
      Integer::J(x) => Float::F(x as f64),
    }
  }
}

impl std::fmt::Display for Float {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Float::E(x) => x.fmt(f),
      Float::F(x) => x.fmt(f),
    }
  }
}

#[derive(
  PartialEq, PartialOrd, Debug, Clone, Copy, Serialize, Deserialize, Eq, Hash,
)]
pub enum Integer {
  G(u8),
  H(u16),
  I(u32),
  J(i64),
}

impl Add for Integer {
  type Output = Integer;
  fn add(self, rhs: Self) -> Self::Output {
    use Integer::*;
    match (self, rhs) {
      (G(x), G(y)) => G(x + y),
      (G(x), H(y)) => H(x as u16 + y),
      (G(x), I(y)) => I(x as u32 + y),
      (G(x), J(y)) => J(x as i64 + y),
      (H(x), G(y)) => H(x + y as u16),
      (H(x), H(y)) => H(x + y),
      (H(x), I(y)) => I(x as u32 + y),
      (H(x), J(y)) => J(x as i64 + y),
      (I(x), G(y)) => I(x + y as u32),
      (I(x), H(y)) => I(x + y as u32),
      (I(x), I(y)) => I(x + y),
      (I(x), J(y)) => J(x as i64 + y),
      (J(x), G(y)) => J(x + y as i64),
      (J(x), H(y)) => J(x + y as i64),
      (J(x), I(y)) => J(x + y as i64),
      (J(x), J(y)) => J(x + y),
    }
  }
}

impl Sub for Integer {
  type Output = Integer;
  fn sub(self, rhs: Self) -> Self::Output {
    use Integer::*;
    match (self, rhs) {
      (G(x), G(y)) => G(x - y),
      (G(x), H(y)) => H(x as u16 - y),
      (G(x), I(y)) => I(x as u32 - y),
      (G(x), J(y)) => J(x as i64 - y),
      (H(x), G(y)) => H(x - y as u16),
      (H(x), H(y)) => H(x - y),
      (H(x), I(y)) => I(x as u32 - y),
      (H(x), J(y)) => J(x as i64 - y),
      (I(x), G(y)) => I(x - y as u32),
      (I(x), H(y)) => I(x - y as u32),
      (I(x), I(y)) => I(x - y),
      (I(x), J(y)) => J(x as i64 - y),
      (J(x), G(y)) => J(x - y as i64),
      (J(x), H(y)) => J(x - y as i64),
      (J(x), I(y)) => J(x - y as i64),
      (J(x), J(y)) => J(x - y),
    }
  }
}

impl From<Char> for Integer {
  fn from(i: Char) -> Self {
    Integer::G(i.0)
  }
}

impl std::fmt::Display for Integer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Integer::G(x) => x.fmt(f),
      Integer::H(x) => x.fmt(f),
      Integer::I(x) => x.fmt(f),
      Integer::J(x) => x.fmt(f),
    }
  }
}

#[derive(
  PartialEq, PartialOrd, Debug, Clone, Copy, Serialize, Deserialize, Eq, Hash,
)]
pub struct Char(u8);

impl Add for Char {
  type Output = Char;
  fn add(self, rhs: Self) -> Self::Output {
    Char(self.0 + rhs.0)
  }
}

impl Sub for Char {
  type Output = Char;
  fn sub(self, rhs: Self) -> Self::Output {
    Char(self.0 - rhs.0)
  }
}

impl Deref for Char {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<Integer> for Char {
  fn from(i: Integer) -> Self {
    match i {
      Integer::G(x) => Char(x),
      Integer::H(x) => Char(x as u8),
      Integer::I(x) => Char(x as u8),
      Integer::J(x) => Char(x as u8),
    }
  }
}

impl From<u8> for Char {
  fn from(c: u8) -> Self {
    Char(c)
  }
}

impl std::fmt::Display for Char {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char(self.0 as char)
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
  Symbol(Name),
  Str(Vec<Char>),
  List(Vec<AstNode>),
  Dict(HashMap<Name, AstNode>),
  Table(HashMap<Name, AstNode>),
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
    args: Option<Vec<Name>>,
    expr: Box<AstNode>,
  },
  FnCall {
    name: Name,
    args: Option<Vec<AstNode>>,
  },
  Var {
    name: Name,
    expr: Box<AstNode>,
  },
}

impl std::fmt::Display for AstNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AstNode::Atom(x) => x.fmt(f),
      AstNode::Str(x) => {
        f.write_str(&x.into_iter().map(|c| c.0 as char).collect::<String>())
      }
      AstNode::Symbol(x) => f.write_str(&x.to_string()),
      AstNode::List(x) => f.write_fmt(format_args!(
        "{}",
        x.into_iter()
          .map(|l| l.to_string())
          .intersperse(" ".to_string())
          .collect::<String>()
      )),
      AstNode::Dict(x) => {
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

impl Add for AstNode {
  type Output = Result<AstNode, EvalError>;
  fn add(self, rhs: Self) -> Self::Output {
    use AstNode::*;
    match (self, rhs) {
      (Atom(x), Atom(y)) => Ok(Atom(x + y)),
      (Atom(x), List(y)) => {
        Ok(List(y.iter().map(|y| Atom(x) + y.clone()).try_collect()?))
      }
      (List(x), List(y)) => {
        if x.len() == y.len() {
          Ok(List(
            y.iter().zip(x).map(|(x, y)| x.clone() + y).try_collect()?,
          ))
        } else {
          Err(EvalError::Length)
        }
      }
      (List(x), Atom(y)) => {
        Ok(List(x.iter().map(|x| x.clone() + Atom(y)).try_collect()?))
      }
      _ => todo!(),
    }
  }
}

impl Sub for AstNode {
  type Output = Result<AstNode, EvalError>;
  fn sub(self, rhs: Self) -> Self::Output {
    use AstNode::*;
    match (self, rhs) {
      (Atom(x), Atom(y)) => Ok(Atom(x - y)),
      (Atom(x), List(y)) => {
        Ok(List(y.iter().map(|y| Atom(x) - y.clone()).try_collect()?))
      }
      (List(x), List(y)) => {
        if x.len() == y.len() {
          Ok(List(
            y.iter().zip(x).map(|(x, y)| x.clone() - y).try_collect()?,
          ))
        } else {
          Err(EvalError::Length)
        }
      }
      (List(x), Atom(y)) => {
        Ok(List(x.iter().map(|x| x.clone() - Atom(y)).try_collect()?))
      }
      _ => todo!(),
    }
  }
}
