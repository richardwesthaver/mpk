use mpk_gc::{Trace, Finalize};

pub type Program = Vec<AstNode>;


#[derive(PartialEq, Eq, Debug, Clone, Finalize, Trace)]
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

#[derive(PartialEq, Eq, Debug, Clone, Finalize, Trace)]
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

#[derive(PartialEq, Eq, Debug, Clone, Finalize, Trace)]
pub enum AdVerb {
  Each,      // '
  Over,      // /
  Scan,      // \
  EachPrior, // ':
  EachRight, // /:
  EachLeft,  // \:
}

#[derive(PartialEq, Eq, Debug, Clone, Finalize, Trace)]
pub enum SysVerb {
  Sesh, // \sesh
  Http, // \http
  Osc,  // \osc
  Db,   // \db
}

#[derive(PartialEq, Debug, Clone, Finalize, Trace)]
pub enum AstNode {
  Int(i64),
  Float(f64),
  Date(u128),
  Time(u128),
  Name(String),
  Str(String),
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
