use chrono::naive::{NaiveDate, NaiveTime};
use std::ffi::CString;
pub type Program = Vec<AstNode>;

#[derive(PartialEq, Eq, Debug, Clone)]
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

#[derive(PartialEq, Eq, Debug, Clone)]
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AdVerb {
  Each,      // '
  Over,      // /
  Scan,      // \
  EachPrior, // ':
  EachRight, // /:
  EachLeft,  // \:
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SysVerb {
  Sesh, // 0:sesh
  Http, // 0:http
  Osc,  // 0:osc
  Db,   // 0:db
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
  Int(i32),
  Float(f64),
  Date(NaiveDate),
  Time(NaiveTime),
  Name(String),
  Str(CString),
  Symbol(String),
  Nouns(Vec<AstNode>),
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
  IsGlobal {
    name: String,
    expr: Box<AstNode>,
  },
}
