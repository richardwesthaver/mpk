//! MPK_AST
pub enum Op {
  Add,
  Sub,
  Mul,
  Div,
  Eq,
  Lt,
  Gt,
}

pub enum Cmd {
  Query,
  Open,
  Save,
  Close,
  Exit,
  Play,
  Edit,
  Start,
  Stop,
}
