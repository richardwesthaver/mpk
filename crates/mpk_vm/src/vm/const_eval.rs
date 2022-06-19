//! mpk_vm -- vm/const_eval
use crate::{Obj, c::OptLevel, Result, VmError};
use std::{
    cell::RefCell,
    collections::HashSet,
    convert::TryFrom,
    rc::{Rc, Weak},
};
use mpk_parser::visitor::{VisitorMut, ConsumingVisitor};
use mpk_parser::Node;
use mpk_parser::ast::*;
use mpk_hash::FxHashMap as Map;
use im_rc::HashMap;

use log::debug;

type SharedEnv = Rc<RefCell<ConstantEnv>>;

struct ConstantEnv {
    bindings: HashMap<String, Obj>,
    used_bindings: HashSet<String>,
    non_constant_bound: HashSet<String>,
    parent: Option<Weak<RefCell<ConstantEnv>>>,
}

impl ConstantEnv {
    fn root(bindings: HashMap<String, Obj>) -> Self {
        Self {
            bindings,
            used_bindings: HashSet::new(),
            non_constant_bound: HashSet::new(),
            parent: None,
        }
    }

    fn new_subexpression(parent: Weak<RefCell<ConstantEnv>>) -> Self {
        Self {
            bindings: HashMap::new(),
            used_bindings: HashSet::new(),
            non_constant_bound: HashSet::new(),
            parent: Some(parent),
        }
    }

    fn bind(&mut self, ident: &str, value: Obj) {
        self.bindings.insert(ident.to_owned(), value);
    }

    fn bind_non_constant(&mut self, ident: &str) {
        self.non_constant_bound.insert(ident.to_owned());
    }

    fn get(&mut self, ident: &str) -> Option<Obj> {
        if self.non_constant_bound.get(ident).is_some() {
            return None;
        }

        let value = self.bindings.get(ident);
        if value.is_none() {
            self.parent
                .as_ref()?
                .upgrade()
                .expect("Constant environment freed early")
                .borrow_mut()
                .get(ident)
        } else {
            self.used_bindings.insert(ident.to_string());
            value.cloned()
        }
    }

    fn _set(&mut self, ident: &str, value: Obj) -> Option<Obj> {
        let output = self.bindings.get(ident);
        if output.is_none() {
            self.parent
                .as_ref()?
                .upgrade()
                .expect("Constant environment freed early")
                .borrow_mut()
                ._set(ident, value)
        } else {
            self.bindings.insert(ident.to_string(), value)
        }
    }

    fn unbind(&mut self, ident: &str) -> Option<()> {
        if self.bindings.get(ident).is_some() {
            self.bindings.remove(ident);
            self.used_bindings.insert(ident.to_string());
        } else {
            self.parent
                .as_ref()?
                .upgrade()
                .expect("Constant environment freed early")
                .borrow_mut()
                .unbind(ident);
        }
        Some(())
    }
}

// Holds the global env that will eventually get passed down
//
// Holds the arena for all environments to eventually be dropped together
pub struct ConstantEvalManager {
    global_env: SharedEnv,
    set_idents: HashSet<String>,
    pub(crate) changed: bool,
    opt_level: OptLevel,
}

impl ConstantEvalManager {

}

fn obj_to_atom(value: &Obj) -> Option<Atom> {
    match value {
      Obj::B(x) => Some(Atom::Boolean(*x)),
      Obj::C(x) => Some(Atom::Char(*x)),
      Obj::D(x) => Some(Atom::Time(*x)),
      Obj::E(x) => Some(Atom::Float(Float::E(*x))),      
      Obj::F(x) => Some(Atom::Float(Float::F(*x))),      
      Obj::G(x) => Some(Atom::Int(Integer::G(*x))),      
      Obj::H(x) => Some(Atom::Int(Integer::H(*x))),      
      Obj::I(x) => Some(Atom::Int(Integer::I(*x))),      
      Obj::J(x) => Some(Atom::Int(Integer::J(*x))),      
      _ => None,
    }
}

struct ConstantEvaluator<'a> {
    bindings: SharedEnv,
    set_idents: &'a HashSet<String>,
    changed: bool,
    opt_level: OptLevel,
}

impl<'a> ConstantEvaluator<'a> {
    fn new(
        bindings: Rc<RefCell<ConstantEnv>>,
        set_idents: &'a HashSet<String>,
        opt_level: OptLevel,
    ) -> Self {
        Self {
            bindings,
            set_idents,
            changed: false,
            opt_level,
        }
    }  
    fn to_constant(&self, expr: &AstNode) -> Option<Obj> {
        match expr {
            AstNode::Atom(x) => self.eval_atom(x),
            _ => None,
        }
    }

    fn eval_atom(&self, t: &Atom) -> Option<Obj> {
        match t {
	  Atom::Boolean(x) => Some((*x).into()),
	  Atom::Symbol(x) => {
            // If we found a set identifier, skip it
            if self.set_idents.get(x).is_some() {
              return None;
            };
            self.bindings.borrow_mut().get(x.as_str())
	  }
	  Atom::Char(x) => {},
	  Atom::Int(x) => {},
	  Atom::Float(x) => {},
	  Atom::Time(x) => {},
	}
    }

    fn all_to_constant(&self, exprs: &[AstNode]) -> Option<Vec<Obj>> {
        exprs.iter().map(|x| self.to_constant(x)).collect()
    }

    fn eval_function(
        &mut self,
        evaluated_func: Obj,
        func: AstNode,
        mut raw_args: Vec<AstNode>,
        args: &[Obj],
    ) -> Result<AstNode> {
        if evaluated_func.is_function() {
            match evaluated_func {
                Obj::Fn(x) => {
                    let output = x(args)?;

                    if let Some(atom) = obj_to_atom(&output) {
                        debug!(
                            "Const evaluation of a function resulted in an atom: {}",
                            atom
                        );
                        self.changed = true;
                        Ok(AstNode::Atom(atom))
                    } else if let Ok(output) = AstNode::try_from(&output) {
                        self.changed = true;
                        debug!(
                            "Const evaluation of a function resulted in a value: {}",
                            output
                        );
                        Ok(output)
                    } else {
                        debug!(
                            "Unable to convert constant-evalutable function output to value: {}",
                            evaluated_func
                        );
                        // Something went wrong
                        raw_args.insert(0, func);
                        Ok(AstNode::List(raw_args))
                    }
                }
                _ => {
                    debug!(
                        "Found a non-constant evaluatable function: {}",
                        evaluated_func
                    );
                    raw_args.insert(0, func);
                    // Not a constant evaluatable function, just return the original input
                    Ok(AstNode::List(raw_args))
                }
            }
        } else {
            raw_args.insert(0, func);
            Ok(AstNode::List(raw_args))
        }
    }  
}

impl<'a> ConsumingVisitor for ConstantEvaluator<'a> {
  type Output = Result<AstNode>;
  fn visit_atom(&mut self, x: Atom) -> Self::Output {
    if let Some(inner) = self.eval_atom(&x) {
      if let Some(new_token) = obj_to_atom(&inner) {
	return Ok(AstNode::Atom(x));
      }
    }
    Ok(AstNode::Atom(x))
  }
  fn visit_str(&self, x: String) -> Self::Output {
    Ok(AstNode::Str(x))
  }
  fn visit_list(&mut self, x: Vec<AstNode>) -> Self::Output {
    Ok(AstNode::List(x))
  }
  fn visit_map(&mut self, x: Map<String, AstNode>) -> Self::Output {
    Ok(AstNode::Map(x))
  }
  fn visit_table(&mut self, x: Map<String, AstNode>) -> Self::Output {
    Ok(AstNode::Table(x))
  }
  fn visit_monad(&mut self, x: MonadicVerb, y: Option<AdVerb>, z: Box<AstNode>) -> Self::Output {
    Ok(AstNode::Monad{expr:z,verb:x,adverb:y})
  }
  fn visit_dyad(&mut self, w: Box<AstNode>, x: DyadicVerb, y: Option<AdVerb>, z: Box<AstNode>) -> Self::Output {
    Ok(AstNode::Dyad{lhs:w,verb:x,adverb:y,rhs:z})
  }
  fn visit_sysfn(&mut self, x: SysVerb, y: Option<Box<AstNode>>) -> Self::Output {
    Ok(AstNode::SysFn{verb:x,args:y})
  }
  fn visit_userfn(&mut self, x: Option<Vec<String>>, y: Box<AstNode>) -> Self::Output {
    Ok(AstNode::UserFn{args:x,expr:y})
  }
  fn visit_fncall(&mut self, x: String, y: Option<Vec<AstNode>>) -> Self::Output {
    Ok(AstNode::FnCall{name:x,args:y})
  }
  fn visit_var(&mut self, x: String, y: Box<AstNode>) -> Self::Output {
    Ok(AstNode::Var{name:x,expr:y})
  }
}

struct CollectSet<'a> {
    set_idents: &'a mut HashSet<String>,
}

impl<'a> CollectSet<'a> {
    fn new(set_idents: &'a mut HashSet<String>) -> Self {
        Self { set_idents }
    }
}

impl<'a> VisitorMut for CollectSet<'a> {
  type Output = ();
  fn visit_atom(&mut self, _x: &Atom) -> Self::Output {}
  fn visit_str(&self, x: &str) -> Self::Output {}
  fn visit_list(&mut self, x: &[AstNode]) -> Self::Output {
    for i in x {
      self.visit(i);
    }
  }
  fn visit_map(&mut self, x: &Map<String, AstNode>) -> Self::Output {}
  fn visit_table(&mut self, x: &Map<String, AstNode>) -> Self::Output {}
  fn visit_monad(&mut self, x: &MonadicVerb, y: &Option<AdVerb>, z: &Box<AstNode>) -> Self::Output {}
  fn visit_dyad(&mut self, w: &Box<AstNode>, x: &DyadicVerb, y: &Option<AdVerb>, z: &Box<AstNode>) -> Self::Output {}
  fn visit_sysfn(&mut self, x: &SysVerb, y: &Option<Box<AstNode>>) -> Self::Output {}
  fn visit_userfn(&mut self, x: &Option<Vec<String>>, y: &Box<AstNode>) -> Self::Output {}
  fn visit_fncall(&mut self, x: &String, y: &Option<Vec<AstNode>>) -> Self::Output {}
  fn visit_var(&mut self, x: &String, y: &Box<AstNode>) -> Self::Output {}
}
