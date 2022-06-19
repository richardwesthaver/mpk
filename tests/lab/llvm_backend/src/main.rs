use mpk::parser::{ast::*, parse};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, BasicValue, FloatValue, FunctionValue, PointerValue};
use inkwell::{FloatPredicate, OptimizationLevel};
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::{self, Write};
use std::iter::Peekable;
use std::ops::DerefMut;
use std::str::Chars;

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,
    pub expr: &'a AstNode,

    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Gets a defined function given its name.
    #[inline]
    fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    /// Returns the `FunctionValue` representing the function being compiled.
    #[inline]
    fn fn_value(&self) -> FunctionValue<'ctx> {
        self.fn_value_opt.unwrap()
    }
    /// Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();

        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(self.context.f64_type(), name)
    }

    /// Compiles the specified `Expr` into an LLVM `FloatValue`.
  fn compile_expr(&mut self, expr: &AstNode) -> Result<FloatValue<'ctx>, &'static str> {
    match *expr {
          }
  }
}
fn main() {
  let input = "2+2";
  dbg!(parse(input).unwrap());
  let context = Context::create();
  let module = context.create_module("repl");
  let builder = context.create_builder();  
  Target::initialize_all(&InitializationConfig::default());
  // use the host machine as the compilation target
  let target_triple = TargetMachine::get_default_triple();
  let cpu = TargetMachine::get_host_cpu_name().to_string();
  let features = TargetMachine::get_host_cpu_features().to_string();

  // make a target from the triple
  let target = Target::from_triple(&target_triple)
    .map_err(|e| format!("{:?}", e))?;

  // make a machine from the target
  let target_machine = target
    .create_target_machine(
      &target_triple,
      &cpu,
      &features,
      OptimizationLevel::Default,
      RelocMode::Default,
      CodeModel::Default,
    )
    .ok_or_else(|| "Unable to create compiler!".to_string())?;

// use the machine to convert our module to machine code and write the result to a file
let output_filename = matches.value_of("output").unwrap();
target_machine
    .write_to_file(&module, FileType::Object, output_filename.as_ref())
    .map_err(|e| format!("{:?}", e))?;
}
