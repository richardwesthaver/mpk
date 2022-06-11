//! mpk_vm/c -- gen
//! codegen
use crate::i::Instruction;

pub struct Gen<'a> {
  buf: Vec<Instruction>,
//  cons: &'a mut ConstantMap,
  def_ctx: Option<String>,
//  syms: &'a mut SymbolMap,
  depth: u32,
//  vars: Option<Rc<RefCell<VarData>>>,
  let_ctx: bool,
  ip: usize,
}
