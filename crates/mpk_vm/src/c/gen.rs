//! mpk_vm/c -- gen
//! codegen
use crate::i::{Instruction, Op};
use crate::{Result, Obj, stop, VmError};
use super::{cons::{ConstantMap, ConstantTable}, map::SymbolMap};
use mpk_parser::{Node, ast::*, visitor::VisitorMut};
use std::{cell::RefCell, rc::Rc};
use log::info;

#[derive(Clone, Debug)]
struct LocalVar {
    depth: u32,
    name: String,
    is_captured: bool,
    struct_offset: usize,
    syntax_object: Node,
}

impl LocalVar {
    pub fn new(depth: u32, name: String, syntax_object: Node) -> Self {
        LocalVar {
            depth,
            name,
            is_captured: false,
            struct_offset: 0,
            syntax_object,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct UpValue {
    // The slot that the upvalue is capturing
    index: usize,
    // Whether or not this is a local variable at all
    is_local: bool,
}

impl UpValue {
    pub fn new(index: usize, is_local: bool) -> Self {
        UpValue { index, is_local }
    }
}

#[derive(Clone, Debug)]
pub struct VarData {
    locals: Vec<LocalVar>,
    upvalues: Vec<UpValue>,
    enclosing: Option<Rc<RefCell<VarData>>>,
}

impl VarData {
    fn new(
        locals: Vec<LocalVar>,
        upvalues: Vec<UpValue>,
        enclosing: Option<Rc<RefCell<VarData>>>,
    ) -> Self {
        VarData {
            locals,
            upvalues,
            enclosing,
        }
    }

    fn push_local(&mut self, local: LocalVar) {
        self.locals.push(local)
    }

    // Set a local to be captured for later code generation
    fn mark_captured(&mut self, index: usize) {
        self.locals[index].is_captured = true;
    }

    // Go backwards and attempt to find the index in which a local variable will live on the stack
    // returns (actual, stack)
    fn resolve_local(&self, ident: &str) -> Option<usize> {
        let idx = self
            .locals
            .iter()
            .rev()
            .position(|x| &x.name == ident)
            .map(|x| self.locals.len() - 1 - x)?;

        let var = self.locals.iter().rev().find(|x| &x.name == ident)?;
        Some(idx + var.struct_offset)
    }

    // Resolve the upvalue with some recursion shenanigans
    fn resolve_upvalue(&mut self, ident: &str) -> Option<usize> {
        if self.enclosing.is_none() {
            return None;
        }

        // Check local first
        let local = self
            .enclosing
            .as_ref()
            .map(|x| x.borrow().resolve_local(ident))
            .flatten();

        if let Some(local) = local {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .mark_captured(local);

            return Some(self.add_upvalue(local, true));
        }

        // Check upvalues afterwards
        let upvalue = self
            .enclosing
            .as_ref()
            .map(|x| x.borrow_mut().resolve_upvalue(ident))
            .flatten();
        if let Some(upvalue) = upvalue {
            return Some(self.add_upvalue(upvalue, false));
        }

        // Otherwise we're a global and we should move on
        None
    }

    // Add the upvalue to the upvalue list, returning the index in the list
    fn add_upvalue(&mut self, index: usize, is_local: bool) -> usize {
        // If the upvalue has already been captured, don't capture it again
        if let Some(i) = self
            .upvalues
            .iter()
            .position(|x| x.index == index && x.is_local == is_local)
        {
            return i;
        }

        self.upvalues.push(UpValue::new(index, is_local));
        self.upvalues.len() - 1
    }
}

pub struct Gen<'a> {
  buf: Vec<Instruction>,
  cons: &'a mut ConstantMap,
  def_ctx: Option<String>,
  syms: &'a mut SymbolMap,
  depth: u32,
  vars: Option<Rc<RefCell<VarData>>>,
  let_ctx: bool,
  stack_offset: usize,
}

impl<'a> Gen<'a> {
    pub fn new(cons: &'a mut ConstantMap, syms: &'a mut SymbolMap) -> Self {
        Gen {
            buf: Vec::new(),
	  cons,
            def_ctx: None,
	  syms,
            depth: 0,
            vars: None,
            let_ctx: false,
            stack_offset: 0,
            // enclosing: None,
        }
    }
    fn new_from_body_instructions(
        cons: &'a mut ConstantMap,
        syms: &'a mut SymbolMap,
        buf: Vec<Instruction>,
        depth: u32,
        vars: Option<Rc<RefCell<VarData>>>,
    ) -> Self {
        Gen {
	  buf,
            cons,
            def_ctx: None,
	  syms,
            depth,
            vars,
            let_ctx: false,
            stack_offset: 0,
            // enclosing,
        }
    }

    pub fn compile(mut self, expr: &AstNode) -> Result<Vec<Instruction>> {
        self.visit(expr)?;
        Ok(self.buf)
    }

    #[inline]
    fn push(&mut self, instr: Instruction) {
        self.buf.push(instr);
    }

    #[inline]
    fn len(&self) -> usize {
        self.buf.len()
    }
  fn specialize_constant(&mut self, a: &AstNode) -> Result<()> {
    let val = eval_atom(a)?;
    let op = Op::PUSHCONST;
    let idx = self.cons.add_or_get(val);
    self.push(Instruction::new(op, idx, a.clone(), true));
    Ok(())
  }
}

// TODO
impl <'a> VisitorMut for Gen<'a> {
  type Output = Result<()>;
  fn visit_atom(&mut self, a: &Atom) -> Self::Output {
    let ident = if let Atom::Symbol(i) = &a {
      i
    } else {
      self.specialize_constant(&a)?;
      return Ok(());
    };
    // Attempt to resolve this as a local variable
    if let Some(idx) = self
      .vars
      .as_ref()
      .map(|x| x.borrow().resolve_local(ident))
      .flatten()
    {
      self.push(Instruction::new_local(idx, a.clone()));
      
      // Otherwise attempt to resolve this as an upvalue
    } else if let Some(idx) = self
      .vars
      .as_ref()
      .map(|x| x.borrow_mut().resolve_upvalue(ident))
      .flatten()
    {
      self.push(Instruction::new_read_upvalue(idx, a.clone()));
      
      // Otherwise we resort to it being a global variable for now
    } else {
      self.push(Instruction::new(Op::PUSH, 0, a.clone(), true));
    }
    
    Ok(())    
  }
}

fn transform_tail_call(instructions: &mut [Instruction], defining_context: &str) -> bool {
    let last_idx = instructions.len() - 1;

    let mut indices = vec![last_idx];

    let mut transformed = false;

    for (idx, instruction) in instructions.iter().enumerate() {
        if instruction.op == Op::JMP && instruction.payload_size == last_idx {
            indices.push(idx);
        }
    }

    for index in &indices {
        if *index < 2 {
            continue;
        }
        let prev_instruction = instructions.get(index - 1);
        let prev_func_push = instructions.get(index - 2);

        match (prev_instruction, prev_func_push) {
            (
                Some(Instruction {
                    op: Op::FUNC,
                    payload_size: arity,
                    ..
                }),
                Some(Instruction {
                    op: Op::PUSH,
                    contents:
                        Some(Node(AstNode::Atom(Atom::Symbol(s)),
                            ..
                        )),
                    ..
                }),
            ) => {
                let arity = *arity;
                if s == defining_context {
                    let new_jmp = Instruction::new_tco_jmp();
                    // inject tail call jump
                    instructions[index - 2] = new_jmp;
                    instructions[index - 1] = Instruction::new_pass(arity);
                    transformed = true;

                    info!("Tail call optimization performed for: {}", defining_context);
                    // println!("Tail call optimization performed for: {}", defining_context);
                }
            }
            _ => {}
        }
    }

    transformed
}

// Find if this function has a valid TCO able situation
fn identify_letrec_tailcall(instructions: &[Instruction], ident: &str) -> bool {
    for i in 0..instructions.len() - 1 {
        let read = instructions.get(i);
        let set = instructions.get(i + 1);

        match (read, set) {
            (
                Some(Instruction {
                    op: Op::READUPVALUE,
                    contents:
                        Some(Node(AstNode::Atom(Atom::Symbol(local_value)),
                            ..
                        )),
                    ..
                }),
                Some(Instruction {
                    op: Op::TAILCALL,
                    ..
                }),
            ) => {
                // println!("FOUND LOCAL VALUE: {}", local_value);
                if local_value == ident {
                    return true;
                }
            }
            _ => {}
        }
    }

    false
}

// If the upvalue func has been used before the set, we can't TCO it
fn upvalue_func_used_before_set(instructions: &[Instruction], upvalue: &str, idx: usize) -> bool {
    // Iterate up to the set index
    // If the upvalue is used prior to that, don't use it
    for i in 0..idx {
        if let Some(Instruction {
            contents:
                Some(Node(AstNode::Atom(Atom::Symbol(s)),
                    ..
                )),
            ..
        }) = instructions.get(i)
        {
            if upvalue == s {
                return true;
            }
        }
    }

    false
}

// Use this to flatten calls to globals such that its just one instruction instead of two
pub fn convert_call_globals(instructions: &mut [Instruction]) {
    if instructions.is_empty() {
        return;
    }

    for i in 0..instructions.len() - 1 {
        let push = instructions.get(i);
        let func = instructions.get(i + 1);

        match (push, func) {
            (
                Some(Instruction {
                    op: Op::PUSH,
                    ..
                }),
                Some(Instruction {
                    op: Op::FUNC,
                    ..
                }),
            ) => {
                if let Some(x) = instructions.get_mut(i) {
                    x.op = Op::CALLGLOBAL;
                }

                if let Some(x) = instructions.get_mut(i + 1) {
                    x.op = Op::PASS;
                }
            }
            (
                Some(Instruction {
                    op: Op::PUSH,
                    ..
                }),
                Some(Instruction {
                    op: Op::TAILCALL,
                    ..
                }),
            ) => {
                if let Some(x) = instructions.get_mut(i) {
                    x.op = Op::CALLGLOBALTAIL;
                }

                if let Some(x) = instructions.get_mut(i + 1) {
                    x.op = Op::PASS;
                }
            }
            _ => {}
        }
    }
}

// 0    READLOCAL : 0
// 1    LOADINT2 : 12
// 2    CALLGLOBAL : 6
// 3    PASS : 2

// Often, there may be a loop condition with something like (= x 10000)
// this identifies these and lazily applies the function, only pushing on to the stack
// until it absolutely needs to
pub fn loop_condition_local_const_arity_two(instructions: &mut [Instruction]) {
    for i in 0..instructions.len() {
        let read_local = instructions.get(i);
        let push_const = instructions.get(i + 1);
        let call_global = instructions.get(i + 2);
        let pass = instructions.get(i + 3);

        match (read_local, push_const, call_global, pass) {
            (
                Some(Instruction {
                    op: Op::READLOCAL,
                    payload_size: local_idx,
                    ..
                }),
                Some(Instruction {
                    op: Op::PUSHCONST,
                    payload_size: const_idx,
                    ..
                }),
                Some(Instruction {
                    op: Op::CALLGLOBAL,
                    payload_size: ident,
                    contents: identifier,
                    ..
                }),
                // HAS to be arity 2 in this case
                Some(Instruction {
                    op: Op::PASS,
                    payload_size: 2,
                    ..
                }),
            ) => {
                let local_idx = *local_idx;
                let const_idx = *const_idx;
                let ident = *ident;
                let identifier = identifier.clone();

                if let Some(x) = instructions.get_mut(i) {
                    x.op = Op::CGLOCALCONST;
                    x.payload_size = ident;
                    x.contents = identifier;
                }

                if let Some(x) = instructions.get_mut(i + 1) {
                    x.op = Op::READLOCAL;
                    x.payload_size = local_idx;
                }

                if let Some(x) = instructions.get_mut(i + 2) {
                    x.op = Op::PUSHCONST;
                    x.payload_size = const_idx;
                }
            }
            _ => {}
        }
    }
}

// attempt to find if this is a TCO valid let rec situation
fn identify_let_rec(
    instructions: &[Instruction],
    context: &str,
) -> Option<(String, String, usize)> {
    // println!("Identifying let rec...");
    for i in 0..instructions.len() - 1 {
        let read = instructions.get(i);
        let set = instructions.get(i + 1);

        match (read, set) {
            (
                Some(Instruction {
                    op: Op::READLOCAL,
                    contents:
                        Some(Node(AstNode::Atom(Atom::Symbol(local_value)),
                            ..
                        )),
                    ..
                }),
                Some(Instruction {
                    op: Op::SETUPVALUE,
                    contents:
                        Some(Node(AstNode::Atom(Atom::Symbol(ident_being_set)),
                            ..
                        )),
                    ..
                }),
            ) => {
                // println!(
                //     "FOUND LOCAL_VALUE: {} AND IDENT: {}",
                //     local_value, ident_being_set
                // );

                if context == local_value {
                    return Some((ident_being_set.clone(), local_value.clone(), i));
                }
            }
            _ => {}
        }
    }

    None
}

// Note, this should be called AFTER `transform_tail_call`
fn check_and_transform_mutual_recursion(instructions: &mut [Instruction]) -> bool {
    let last_idx = instructions.len() - 1;

    // could panic
    let mut indices = vec![last_idx];

    let mut transformed = false;

    for (idx, instruction) in instructions.iter().enumerate() {
        if instruction.op == Op::JMP && instruction.payload_size == last_idx {
            indices.push(idx);
        }
    }

    for index in &indices {
        if *index < 2 {
            continue;
        }
        let prev_instruction = instructions.get(index - 1);
        let prev_func_push = instructions.get(index - 2);

        match (prev_instruction, prev_func_push) {
            (
                Some(Instruction {
                    op: Op::FUNC,
                    ..
                }),
                Some(Instruction {
                    op: Op::PUSH,
                    contents:
                  Some(Node(AstNode::Atom(Atom::Symbol(_s)),
                            ..
                        )),
                    ..
                }),
            )
            | (
                Some(Instruction {
                    op: Op::FUNC,
                    ..
                }),
                Some(Instruction {
                    op: Op::READUPVALUE,
                    contents:
                        Some(Node(AstNode::Atom(Atom::Symbol(_s)),
                            ..
                        )),
                    ..
                }),
            ) => {
                // let s = s.clone();
                if let Some(x) = instructions.get_mut(index - 1) {
                    x.op = Op::TAILCALL;
                    transformed = true;
                    // println!("Found tail call with: {}", &s);
                }
            }
            _ => {}
        }
    }

    transformed
}

// TODO
/// evaluates an atom expression in given environment
fn eval_atom(t: &AstNode) -> Result<Obj> {
    match &t {
      AstNode::Atom(atom) => {
	match atom {
	  Atom::Boolean(x) => Ok((*x).into()),
	  Atom::Char(x) => Ok(Obj::C(x.0 as char)),
	  Atom::Float(flt) => {
	    match flt {
	      Float::E(x) => Ok(Obj::E(*x)),
	      Float::F(x) => Ok(Obj::F(*x)),
	    }
	  },
	  Atom::Int(int) => {
	    match int {
	      Integer::G(x) => Ok(Obj::G(*x)),
	      Integer::H(x) => Ok(Obj::H(*x)),
	      Integer::I(x) => Ok(Obj::I(*x)),
	      Integer::J(x) => Ok(Obj::J(*x)),
	      Integer::K(x) => Ok(Obj::K(*x)),
	    }
	  },
	  _ => todo!(),
//	  Atom::Time
//	  Atom::Symbol
	}
      },
        AstNode::Str(s) => Ok(Obj::Str(s.clone().into())),
        what => {
            // println!("getting here in the eval_atom");
          stop!(UnexpectedToken => what)
        }
    }
}
