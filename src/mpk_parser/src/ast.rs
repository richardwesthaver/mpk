use std::ffi::CString;

pub type Program = Vec<AstNode>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MonadicVerb {
  Increment,
  Square,
  Negate,
  Reciprocal,
  Tally,
  Ceiling,
  ShapeOf,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DyadicVerb {
  Plus,
  Times,
  LessThan,
  LargerThan,
  Equal,
  Minus,
  Divide,
  Power,
  Residue,
  Copy,
  LargerOf,
  LargerOrEqual,
  Shape,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SysVerb {
  Http,
  Osc,
  Sql,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
  Integer(i32),
  Float(f64),
  MonadicOp {
    verb: MonadicVerb,
    expr: Box<AstNode>,
  },
  DyadicOp {
    verb: DyadicVerb,
    lhs: Box<AstNode>,
    rhs: Box<AstNode>,
  },
  SysOp {
    verb: SysVerb,
    expr: Option<Box<AstNode>>,
  },
  Nouns(Vec<AstNode>),
  IsGlobal {
    ident: String,
    expr: Box<AstNode>,
  },
  Ident(String),
  Str(CString),
}
