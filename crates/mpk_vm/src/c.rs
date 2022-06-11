//! MPK_VM -- c
//! compiler
use mpk_parser::Prog;
pub trait Compile {
  type Output;
  fn compile(ast: Prog) -> Self::Output;
}
