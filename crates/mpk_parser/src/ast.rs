//! MPK_PARSER -- AST
//!
//! Abstract Syntax Tree objects of the mk language.
pub type Program = Vec<AstNode>;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum AdVerb {
  Each,      // '
  Over,      // /
  Scan,      // \
  EachPrior, // ':
  EachRight, // /:
  EachLeft,  // \:
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SysVerb {
  Sesh,   // \sesh
  Http,   // \http
  Osc,    // \osc
  Db,     // \db
  Vars,   // \v
  Work,   // \w
  Import, // \l
  Timeit, // \t
  Exit,   // \\
}

#[derive(PartialEq, Debug, Clone)]
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
