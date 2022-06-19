//! MPK_VM -- OBJ
use std::vec::Vec;
use std::{
  any::Any,
  cell::RefCell,
  cmp::Ordering,
  fmt,
  fmt::Write,
  hash::{Hash, Hasher},
  ops::Fn,
  rc::{Rc, Weak},
  time::Duration,
};
use mpk_parser::ast::{AstNode, Atom, Float, Integer};
use im_rc::{HashMap, Vector};
use Obj::*;

use crate::{i::Ins, Gc, Result, VmError, stop};

pub type FnSig = fn(&[Obj]) -> Result<Obj>;
pub type BoxFnSig = Rc<dyn Fn(&[Obj]) -> Result<Obj>>;
pub type RcrObj = Rc<RefCell<Obj>>;

pub fn new_rcr_obj(x: Obj) -> RcrObj {
  Rc::new(RefCell::new(x))
}

macro_rules! try_from_impl {
    ($(($o:ident $body:ty))*) => {
        $(
            impl TryFrom<Obj> for $body {
                type Error = VmError;
                fn try_from(value: Obj) -> Result<Self> {
                    match value {
                        Obj::$o(x) => Ok(x.clone() as $body),
                        _ => Err(VmError::Conversion("Expected number".to_string())),
                    }
                }
            }

            impl TryFrom<&Obj> for $body {
                type Error = VmError;
                fn try_from(value: &Obj) -> Result<Self> {
                    match value {
                        Obj::$o(x) => Ok(x.clone() as $body),
                        _ => Err(VmError::Conversion("Expected number".to_string())),
                    }
                }
            }

            impl FromObj for $body {
                fn from_obj(value: Obj) -> Result<Self> {
                    match value {
                        Obj::$o(x) => Ok(x.clone() as $body),
                        _ => Err(VmError::Conversion("Expected number".to_string())),
                    }
                }
            }

        )*
    };
}

macro_rules! from_num {
    ($(($o:ident $body:ty))*) => {
        $(
            impl From<$body> for Obj {
                fn from(v:$body) -> Obj {
                    Obj::$o(v)
                }
            }

            impl IntoObj for $body {
                fn into_obj(self) -> Result<Obj> {
                    Ok(Obj::$o(self))
                }
            }
        )*
    };
}

/// The entry point for turning value into Obj
pub trait IntoObj: Sized {
  fn into_obj(self) -> Result<Obj>;
}

/// The exit point for turning Obj into outside world value
pub trait FromObj: Sized {
  fn from_obj(val: Obj) -> Result<Self>;
}

#[derive(Clone)]
pub enum Obj {
  A,
  B(bool),
  C(char),
  D(Duration),
  E(f32),
  F(f64),
  G(u8),
  H(u16),
  I(u32),
  J(u64),
  K(u128),
  L(Gc<Vector<Obj>>),
  M(Gc<HashMap<Obj, Obj>>),
  Fn(FnSig),
  Str(Gc<String>),
  Sym(Gc<String>),
  Custom(Gc<Box<dyn CustomType>>),
  Closure(Gc<ByteCodeLambda>),
  Vec(Gc<Vector<Obj>>),
  BoxFn(BoxFnSig),
  //  Tbl(Gc<HashMap<Obj, Obj>>),
}

impl From<char> for Obj {
  fn from(val: char) -> Obj {
    Obj::C(val)
  }
}

impl TryInto<AstNode> for Obj {
  type Error = VmError;
  fn try_into(self) -> Result<AstNode> {
    match self {
      A => todo!(),
      B(x) => Ok(Atom::Boolean(x).into()),
      C(x) => Ok(Atom::Char(x).into()),
      D(x) => Ok(Atom::Time(x).into()),
      E(x) => Ok(Atom::Float(x.into()).into()),
      F(x) => Ok(Atom::Float(x.into()).into()),
      G(x) => Ok(Atom::Int(x.into()).into()),
      H(x) => Ok(Atom::Int(x.into()).into()),
      I(x) => Ok(Atom::Int(x.into()).into()),
      J(x) => Ok(Atom::Int(x.into()).into()),
      K(x) => Ok(Atom::Int(x.into()).into()),
      _ => stop!(Conversion => "failed to convert Obj to AstNode")
    }
  }
}

impl IntoObj for char {
  fn into_obj(self) -> Result<Obj> {
    Ok(Obj::C(self))
  }
}

impl FromObj for char {
  fn from_obj(val: Obj) -> Result<Self> {
    if let Obj::C(c) = val {
      Ok(c)
    } else {
      Err(VmError::Conversion("Expected character".to_string()))
    }
  }
}

impl<T: Into<Obj>> From<Option<T>> for Obj {
  fn from(val: Option<T>) -> Obj {
    if let Some(s) = val {
      s.into()
    } else {
      Obj::B(true)
    }
  }
}

impl<T: IntoObj> IntoObj for Option<T> {
  fn into_obj(self) -> Result<Obj> {
    if let Some(s) = self {
      s.into_obj()
    } else {
      Ok(Obj::B(false))
    }
  }
}

impl<T: FromObj> FromObj for Option<T> {
  fn from_obj(val: Obj) -> Result<Self> {
    if val.is_truthy() {
      Ok(Some(T::from_obj(val)?))
    } else {
      Ok(None)
    }
  }
}

// TODO make into_obj return a result type
// This allows errors to propagate
impl<T: IntoObj, E: std::fmt::Debug> IntoObj for std::result::Result<T, E> {
  fn into_obj(self) -> Result<Obj> {
    match self {
      Ok(s) => s.into_obj(),
      Err(e) => crate::stop!(Generic => format!("{:?}", e)),
    }
  }
}

impl FromObj for () {
  fn from_obj(val: Obj) -> Result<Self> {
    if let Obj::A = val {
      Ok(())
    } else {
      crate::stop!(Conversion => "could not convert value to unit type")
    }
  }
}

impl IntoObj for () {
  fn into_obj(self) -> Result<Obj> {
    Ok(Obj::A)
  }
}

impl From<()> for Obj {
  fn from(_: ()) -> Obj {
    Obj::A
  }
}

from_num!(
  (E f32)
    (F f64)
    (G u8)
    (H u16)
    (I u32)
    (J u64)
    (K u128));

try_from_impl!(
  (E f32)
    (F f64)
    (G u8)
    (H u16)
    (I u32)
    (J u64)
    (K u128)
    (J usize));

impl TryFrom<Obj> for String {
  type Error = VmError;
  fn try_from(value: Obj) -> Result<Self> {
    match value {
      Obj::Str(ref x) => Ok(x.unwrap()),
      Obj::Sym(ref x) => Ok(x.unwrap()),
      _ => Err(VmError::Conversion("Expected string".to_string())),
    }
  }
}

impl From<Obj> for Gc<Obj> {
  fn from(val: Obj) -> Self {
    Gc::new(val)
  }
}

impl From<Gc<Obj>> for Obj {
  fn from(val: Gc<Obj>) -> Self {
    (*val).clone()
  }
}

impl FromObj for String {
  fn from_obj(val: Obj) -> Result<Self> {
    if let Obj::Str(s) = val {
      Ok(s.unwrap())
    } else {
      Err(VmError::Conversion("Expected string".to_string()))
    }
  }
}

impl TryFrom<&Obj> for String {
  type Error = VmError;
  fn try_from(value: &Obj) -> Result<Self> {
    match value {
      Obj::Str(x) => Ok(x.unwrap()),
      Obj::Sym(x) => Ok(x.unwrap()),
      _ => Err(VmError::Conversion("Expected string".to_string())),
    }
  }
}

impl From<String> for Obj {
  fn from(val: String) -> Obj {
    Obj::Str(val.into())
  }
}

impl IntoObj for String {
  fn into_obj(self) -> Result<Obj> {
    Ok(Obj::Str(Gc::new(self)))
  }
}

impl From<String> for Gc<Obj> {
  fn from(val: String) -> Gc<Obj> {
    Gc::new(val.into())
  }
}

impl From<bool> for Obj {
  fn from(val: bool) -> Obj {
    Obj::B(val)
  }
}

impl IntoObj for bool {
  fn into_obj(self) -> Result<Obj> {
    Ok(Obj::B(self))
  }
}

impl From<Vector<Obj>> for Obj {
  fn from(val: Vector<Obj>) -> Obj {
    Obj::Vec(Gc::new(val))
  }
}

impl From<FnSig> for Obj {
  fn from(val: FnSig) -> Obj {
    Obj::Fn(val)
  }
}

// TODO requires list ops
impl<T: IntoObj> IntoObj for Vec<T> {
  fn into_obj(self) -> Result<Obj> {
    let vec_vals: Result<Vec<Obj>> = self.into_iter().map(|x| x.into_obj()).collect();

    match vec_vals {
      // TODO
      Ok(_l) => Ok(Obj::L(Gc::new(Vector::new()))),
      _ => Err(VmError::Conversion(
        "Could not convert vector of values to SteelVal list".to_string(),
      )),
    }
  }
}

impl<T: FromObj> FromObj for Vec<T> {
  fn from_obj(val: Obj) -> Result<Self> {
    match val {
      Vec(v) => {
        let result_vec_vals: Result<Self> =
          v.iter().map(|x| FromObj::from_obj(x.clone())).collect();
        match result_vec_vals {
          Ok(x) => Ok(x),
          _ => Err(VmError::Conversion(
            "Could not convert SteelVal list to Vector of values".to_string(),
          )),
        }
      } // TODO
      _ => Err(VmError::Conversion(
        "Could not convert SteelVal list to Vector of values".to_string(),
      )),
    }
  }
}

// HashMap
impl<K: IntoObj, V: IntoObj> IntoObj for std::collections::HashMap<K, V> {
  fn into_obj(mut self) -> Result<Obj> {
    let mut hm = im_rc::HashMap::new();
    for (key, val) in self.drain() {
      hm.insert(key.into_obj()?, val.into_obj()?);
    }
    Ok(Obj::M(Gc::new(hm)))
  }
}

impl<M: FromObj + Eq + std::hash::Hash, V: FromObj> FromObj
  for std::collections::HashMap<M, V>
{
  fn from_obj(val: Obj) -> Result<Self> {
    // todo!()
    if let M(hm) = val {
      let mut h = std::collections::HashMap::new();
      for (key, value) in hm.unwrap().into_iter() {
        h.insert(M::from_obj(key)?, V::from_obj(value)?);
      }
      Ok(h)
    } else {
      Err(VmError::Conversion(
        "Could not convert SteelVal to HashMap".to_string(),
      ))
    }
  }
}

impl Obj {
  pub fn is_truthy(&self) -> bool {
    match &self {
      Obj::B(false) => false,
      Obj::A => false,
      Obj::Vec(v) => !v.is_empty(),
      _ => true,
    }
  }
  pub fn is_hashable(&self) -> bool {
    matches!(
      self,
      B(_)|C(_)|D(_)|E(_)|F(_)|G(_)|H(_)|I(_)|J(_)|K(_)|L(_)
        | Vec(_)
//	| Tbl(_)
        | Str(_)
        | Sym(_)
        | M(_) //        | Closure(_)
    )
  }
  pub fn is_function(&self) -> bool {
    matches!(
      self,
      Fn(_) // | Closure(_)
    )
  }
}

impl Eq for Obj {}

impl PartialEq for Obj {
  fn eq(&self, b: &Self) -> bool {
    match (self, b) {
      (B(x), B(y)) => x == y,
      (C(x), C(y)) => x == y,
      (D(x), D(y)) => x == y,
      (E(x), E(y)) => x == y,
      (F(x), F(y)) => x == y,
      (G(x), G(y)) => x == y,
      (H(x), H(y)) => x == y,
      (I(x), I(y)) => x == y,
      (J(x), J(y)) => x == y,
      (K(x), K(y)) => x == y,
      (L(x), L(y)) => x == y,
      (Str(x), Str(y)) => x == y,
      (Sym(x), Sym(y)) => x == y,
      (Vec(x), Vec(y)) => x == y,
      (M(x), M(y)) => x == y,
      //      (Tbl(x),Tbl(y)) => x==y,
      (_, _) => false,
    }
  }
}

impl Hash for Obj {
  fn hash<H: Hasher>(&self, s: &mut H) {
    match self {
      A => unimplemented!(),
      B(x) => x.hash(s),
      C(x) => x.hash(s),
      D(x) => x.hash(s),
      E(_) => unimplemented!(),
      F(_) => unimplemented!(),
      G(x) => x.hash(s),
      H(x) => x.hash(s),
      I(x) => x.hash(s),
      J(x) => x.hash(s),
      K(x) => x.hash(s),
      L(x) => x.hash(s),
      Str(x) => x.hash(s),
      Sym(x) => {
        "`".hash(s);
        x.hash(s)
      }
      M(x) => x.hash(s),
      Vec(x) => x.hash(s),
      //      Tbl(x) => x.hash(s),
      Fn(_) => unimplemented!(),
      BoxFn(_) => unimplemented!(),
      Closure(_) => unimplemented!(),
      Custom(_) => unimplemented!(),
    }
  }
}

impl PartialOrd for Obj {
  fn partial_cmp(&self, b: &Self) -> Option<core::cmp::Ordering> {
    match (self, b) {
      (B(x), B(y)) => x.partial_cmp(y),
      (C(x), C(y)) => x.partial_cmp(y),
      (D(x), D(y)) => x.partial_cmp(y),
      (E(x), E(y)) => x.partial_cmp(y),
      (F(x), F(y)) => x.partial_cmp(y),
      (G(x), G(y)) => x.partial_cmp(y),
      (H(x), H(y)) => x.partial_cmp(y),
      (I(x), I(y)) => x.partial_cmp(y),
      (J(x), J(y)) => x.partial_cmp(y),
      (K(x), K(y)) => x.partial_cmp(y),
      (Str(x), Str(y)) => x.partial_cmp(y),
      (_, _) => None,
    }
  }
}

impl fmt::Display for Obj {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Sym(_) | L(_) => write!(f, "`")?,
      _ => (),
    }
    obj_display(self, f)
  }
}

impl fmt::Debug for Obj {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Sym(_) | L(_) => write!(f, "`")?,
      _ => (),
    }
    obj_display(self, f)
  }
}

fn obj_display(o: &Obj, f: &mut fmt::Formatter) -> fmt::Result {
  match o {
    A => write!(f, "0V"),
    B(x) => {
      if *x {
        write!(f, "1b")
      } else {
        write!(f, "0b")
      }
    }
    C(x) => write!(f, "{}", x),
    D(x) => write!(f, "{}", x.as_secs_f64()),
    E(x) => write!(f, "{}", x),
    F(x) => write!(f, "{}", x),
    G(x) => write!(f, "{}", x),
    H(x) => write!(f, "{}", x),
    I(x) => write!(f, "{}", x),
    J(x) => write!(f, "{}", x),
    K(x) => write!(f, "{}", x),
    L(x) => {
      let mut it = x.iter();
      write!(f, "(")?;
      if let Some(e) = it.next_back() {
        for i in it {
          obj_display(i, f)?;
          write!(f, " ")?;
        }
        obj_display(e, f)?;
      }
      write!(f, ")")
    }
    Fn(_) => write!(f, "{{fn}}"),
    Str(x) => write!(f, "{}", x),
    Sym(x) => write!(f, "{}", x),
    M(x) => {
      let mut str: Vec<String> = vec![];
      for (k, v) in x.iter() {
        str.push(format!("{}|{}", k, v.to_string()));
      }
      f.write_fmt(format_args!(
        "{}",
        str
          .into_iter()
          .intersperse("\n".to_string())
          .collect::<String>()
      ))
    }
    Vec(x) => {
      let mut it = x.iter();
      write!(f, "[")?;
      if let Some(e) = it.next_back() {
        for i in it {
          obj_display(i, f)?;
          write!(f, " ")?;
        }
        obj_display(e, f)?;
      }
      write!(f, "]")
    }
    // Tbl(x) => {
    //   let mut str: Vec<String> = vec![];
    //   for (k, v) in x.iter() {
    //     str.push(format!("{}|{}", k, v.to_string()));
    //   }
    //   f.write_fmt(format_args!(
    //     "{}",
    //     str
    //       .into_iter()
    //       .intersperse("\n".to_string())
    //       .collect::<String>()
    //   ))
    // },
    Custom(x) => write!(f, "{}", x.display()?),
    Closure(_) => write!(f, "{{cl}}"),
    BoxFn(_) => write!(f, "{{fn}}"),
  }
}

/// marker trait for custom types
pub trait Custom {}

/// custom type impl
pub trait CustomType {
  fn box_clone(&self) -> Box<dyn CustomType>;
  fn as_any(&self) -> Box<dyn Any>;
  fn name(&self) -> String {
    (std::any::type_name::<Self>()).to_string()
  }
  fn new_obj(&self) -> Obj;
  fn display(&self) -> std::result::Result<String, std::fmt::Error>;
}

impl Clone for Box<dyn CustomType> {
  fn clone(&self) -> Box<dyn CustomType> {
    self.box_clone()
  }
}

impl From<Box<dyn CustomType>> for Obj {
  fn from(val: Box<dyn CustomType>) -> Obj {
    val.new_obj()
  }
}

impl<T: Custom + Clone + 'static + std::fmt::Debug> CustomType for T {
  fn box_clone(&self) -> Box<dyn CustomType> {
    Box::new((*self).clone())
  }
  fn as_any(&self) -> Box<dyn Any> {
    Box::new((*self).clone())
  }
  fn new_obj(&self) -> Obj {
    Obj::Custom(Gc::new(Box::new(self.clone())))
  }
  fn display(&self) -> std::result::Result<String, std::fmt::Error> {
    let mut buf = String::new();
    write!(buf, "{:?}", &self)?;
    Ok(buf)
  }
}

impl<T: CustomType> IntoObj for T {
  fn into_obj(self) -> Result<Obj> {
    Ok(self.new_obj())
  }
}

impl<T: CustomType + Clone + 'static> FromObj for T {
  fn from_obj(val: Obj) -> Result<Self> {
    if let Obj::Custom(v) = val {
      let left_type = v.as_any();
      let left: Option<T> = left_type.downcast_ref::<T>().cloned();
      left.ok_or_else(|| {
        let error_message = format!(
          "Type Mismatch: Type of Obj did not match the given type: {}",
          std::any::type_name::<Self>()
        );
        VmError::Conversion(error_message)
      })
    } else {
      let error_message = format!(
        "Type Mismatch: Type of Obj did not match the given type: {}",
        std::any::type_name::<Self>()
      );
      Err(VmError::Conversion(error_message))
    }
  }
}

// Upvalues themselves need to be stored on the heap
// Consider a separate section for them on the heap, or wrap them in a wrapper
// before allocating on the heap
#[derive(Clone, Debug)]
pub struct UpValue {
  // Either points to a stack location, or the value
  pub(crate) location: Location,
  // The next upvalue in the sequence
  pub(crate) next: Option<Weak<RefCell<UpValue>>>,
  // Reachable
  pub(crate) reachable: bool,
}

impl PartialEq for UpValue {
  fn eq(&self, other: &Self) -> bool {
    self.location == other.location
  }
}

impl PartialOrd for UpValue {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (&self.location, &other.location) {
      (Location::Stack(l), Location::Stack(r)) => Some(l.cmp(r)),
      _ => panic!("Cannot compare two values on the heap"),
    }
  }
}

impl UpValue {
  // Given a reference to the stack, either get the value from the stack index
  // Or snag the obj stored inside the upvalue
  pub(crate) fn get_value(&self, stack: &[Obj]) -> Obj {
    match self.location {
      Location::Stack(idx) => stack[idx].clone(),
      Location::Closed(ref v) => v.clone(),
    }
  }

  pub(crate) fn is_reachable(&self) -> bool {
    self.reachable
  }

  // Given a reference to the stack, either get the value from the stack index
  // Or snag the Obj stored inside the upvalue
  pub(crate) fn mutate_value(&mut self, stack: &mut [Obj], value: Obj) -> Obj {
    match self.location {
      Location::Stack(idx) => {
        let old = stack[idx].clone();
        stack[idx] = value;
        old
      }
      Location::Closed(ref v) => {
        let old = v.clone();
        self.location = Location::Closed(value);
        old
      }
    }
  }

  pub(crate) fn get_value_if_closed(&self) -> Option<&Obj> {
    if let Location::Closed(ref v) = self.location {
      Some(v)
    } else {
      None
    }
  }

  pub(crate) fn set_value(&mut self, val: Obj) {
    self.location = Location::Closed(val);
  }

  pub(crate) fn mark_reachable(&mut self) {
    self.reachable = true;
  }

  pub(crate) fn reset(&mut self) {
    self.reachable = false;
  }

  pub(crate) fn is_open(&self) -> bool {
    matches!(self.location, Location::Stack(_))
  }

  pub(crate) fn index(&self) -> Option<usize> {
    if let Location::Stack(idx) = &self.location {
      Some(*idx)
    } else {
      None
    }
  }

  pub(crate) fn new(stack_index: usize, next: Option<Weak<RefCell<UpValue>>>) -> Self {
    UpValue {
      location: Location::Stack(stack_index),
      next,
      reachable: false,
    }
  }

  pub(crate) fn set_next(&mut self, next: Weak<RefCell<UpValue>>) {
    self.next = Some(next);
  }
}

// Either points to a stack index or an Obj directly When performing
// an OPCODE::GET_UPVALUE, index into the array in the current
// function being executed in the stack frame, and pull it in
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum Location {
  Stack(usize),
  Closed(Obj),
}

#[derive(Clone, Debug)]
pub struct ByteCodeLambda {
  /// body of the function with identifiers yet to be bound
  body_exp: Rc<[Ins]>,
  arity: usize,
  upvalues: Vec<Weak<RefCell<UpValue>>>,
}

impl PartialEq for ByteCodeLambda {
  fn eq(&self, other: &Self) -> bool {
    self.body_exp == other.body_exp && self.arity == other.arity
  }
}

impl Hash for ByteCodeLambda {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.body_exp.as_ptr().hash(state);
    // self.sub_expression_env.as_ptr().hash(state);
  }
}

impl ByteCodeLambda {
  pub fn new(
    body_exp: Vec<Ins>,
    arity: usize,
    upvalues: Vec<Weak<RefCell<UpValue>>>,
  ) -> ByteCodeLambda {
    ByteCodeLambda {
      body_exp: Rc::from(body_exp.into_boxed_slice()),
      arity,
      upvalues,
    }
  }

  pub fn body_exp(&self) -> Rc<[Ins]> {
    Rc::clone(&self.body_exp)
  }

  // pub fn sub_expression_env(&self) -> &Weak<RefCell<Env>> {
  //     &self.sub_expression_env
  // }

  // pub fn offset(&self) -> usize {
  //     self.offset
  // }

  pub fn arity(&self) -> usize {
    self.arity
  }

  // pub fn ndef_body(&self) -> usize {
  //     self.ndef_body
  // }

  pub fn upvalues(&self) -> &[Weak<RefCell<UpValue>>] {
    &self.upvalues
  }
}
