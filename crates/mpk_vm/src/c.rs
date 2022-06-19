//! c.rs --- compiler
pub mod gen;
pub mod cons;
pub mod map;
pub mod p;

use mpk_parser::Prog;
use map::SymbolMap;
use cons::ConstantMap;
use gen::{Gen, loop_condition_local_const_arity_two, convert_call_globals};
use p::Program;
use crate::Result;
use crate::i::{Instruction, Ins, compact};
use crate::vm::const_eval::ConstantEvalManager;
use crate::Obj;
use mpk_parser::{parse, ast::AstNode};
use log::debug;
use im_rc::HashMap as ImHashMap;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum OptLevel {
    Zero = 0,
    One,
    Two,
    Three,
}

pub struct Compiler {
  pub symbol_map: SymbolMap,
  pub constant_map: ConstantMap,
  pub opt_level: OptLevel,
}

impl Compiler {
  pub fn new(symbol_map: SymbolMap,
             constant_map: ConstantMap,
  ) -> Compiler {
    Compiler {
      symbol_map,
      constant_map,
      opt_level: OptLevel::Three
    }
  }
  pub fn default() -> Self {
    Compiler::new(
      SymbolMap::new(),
      ConstantMap::new(),
    )
  }  
    /// Registers a name in the underlying symbol map and returns the
    /// idx that it maps to
    pub fn register(&mut self, name: &str) -> usize {
        self.symbol_map.get_or_add(name)
    }

    /// Get the index associated with a name in the underlying symbol
    /// map If the name hasn't been registered, this will return
    /// `None`
    pub fn get_idx(&self, name: &str) -> Option<usize> {
        self.symbol_map.get(name).ok()
    }

    /// compile and emit the program
    pub fn compile_program(
        &mut self,
        expr_str: &str,
        constants: ImHashMap<String, Obj>,
    ) -> Result<Program> {
        let instructions = self.emit_instructions(expr_str, constants)?;

        // TODO Perhaps use a different representation for the constant map
        let program = Program::new(instructions, self.constant_map.clone());
        Ok(program)
    }
  pub fn emit_instructions(
    &mut self,
    expr_str: &str,
    constants: ImHashMap<String, Obj>,
    ) -> Result<Vec<Vec<Ins>>> {
    let parsed = parse(expr_str)?;
    self.emit_instructions_from_exprs(parsed, constants)
    }

    pub fn emit_debug_instructions(
        &mut self,
        expr_str: &str,
        constants: ImHashMap<String, Obj>,
    ) -> Result<Vec<Vec<Instruction>>> {
      let parsed = parse(expr_str)?;
      self.emit_debug_instructions_from_exprs(parsed, constants)
    }  

    pub fn emit_expanded_ast(
        &mut self,
        expr_str: &str,
        constants: ImHashMap<String, Obj>,
    ) -> Result<Vec<AstNode>> {
        let parsed = parse(expr_str)?;

        let expanded_statements = self.expand_expressions(parsed)?;

        let mut expanded_statements = expanded_statements;

        match self.opt_level {
            OptLevel::Three => loop {
                let mut manager = ConstantEvalManager::new(constants.clone(), self.opt_level);
                expanded_statements = manager.run(expanded_statements)?;
                if !manager.changed {
                    break;
                }
            },
            OptLevel::Two => {
                expanded_statements =
                    ConstantEvalManager::new(constants.clone(), self.opt_level)
                        .run(expanded_statements)?;
            }
            _ => {}
        }

        // let expanded_statements =
        //     ConstantEvalManager::new(constants).run(expanded_statements)?;

        Ok(flatten_begins_and_expand_defines(expanded_statements))

        // self.emit_debug_instructions_from_exprs(parsed)
    }

    pub fn expand_expressions(
        &mut self,
        exprs: Vec<AstNode>,
    ) -> Result<Vec<AstNode>> {
        #[cfg(feature = "modules")]
        return self
            .module_manager
            .compile_main(&mut self.macro_env, exprs, path);

        #[cfg(not(feature = "modules"))]
        self.module_manager
            .expand_expressions(&mut self.macro_env, exprs)
    }

    pub fn generate_dense_instructions(
        &mut self,
        expanded_statements: Vec<AstNode>,
        results: Vec<Vec<Ins>>,
    ) -> Result<Vec<Vec<Ins>>> {
        let mut results = results;
        let mut instruction_buffer = Vec::new();
        let mut index_buffer = Vec::new();

        for expr in expanded_statements {
            // TODO add printing out the expression as its own special function
            // println!("{:?}", expr.to_string());
            // let mut instructions: Vec<Instruction> = Vec::new();

            let mut instructions =
                Gen::new(&mut self.constant_map, &mut self.symbol_map).compile(&expr)?;

            // TODO double check that arity map doesn't exist anymore
            // emit_loop(&expr, &mut instructions, None, &mut self.constant_map)?;

            instructions.push(Instruction::new_pop());
            inject_heap_save_to_pop(&mut instructions);
            index_buffer.push(instructions.len());
            instruction_buffer.append(&mut instructions);
        }

        convert_call_globals(&mut instruction_buffer);
        replace_defines_with_debruijn_indices(&mut instruction_buffer, &mut self.symbol_map)?;

        // TODO
        loop_condition_local_const_arity_two(&mut instruction_buffer);

        for idx in index_buffer {
            let extracted: Vec<Instruction> = instruction_buffer.drain(0..idx).collect();
            // pretty_print_instructions(extracted.as_slice());
            results.push(compact(extracted));
        }

        Ok(results)
    }

    fn generate_debug_instructions(
        &mut self,
        expanded_statements: Vec<AstNode>,
        results: Vec<Vec<Instruction>>,
    ) -> Result<Vec<Vec<Instruction>>> {
        let mut results = results;
        let mut instruction_buffer = Vec::new();
        let mut index_buffer = Vec::new();

        for expr in expanded_statements {
            // TODO add printing out the expression as its own special function

            let mut instructions =
                Gen::new(&mut self.constant_map, &mut self.symbol_map).compile(&expr)?;

            instructions.push(Instruction::new_pop());
            inject_heap_save_to_pop(&mut instructions);
            index_buffer.push(instructions.len());
            instruction_buffer.append(&mut instructions);
        }

        convert_call_globals(&mut instruction_buffer);
        replace_defines_with_debruijn_indices(&mut instruction_buffer, &mut self.symbol_map)?;

        // TODO
        loop_condition_local_const_arity_two(&mut instruction_buffer);

        for idx in index_buffer {
            let extracted: Vec<Instruction> = instruction_buffer.drain(0..idx).collect();
            // pretty_print_instructions(extracted.as_slice());
            results.push(extracted);
        }

        Ok(results)
    }

    fn emit_debug_instructions_from_exprs(
        &mut self,
        exprs: Vec<AstNode>,
        constants: ImHashMap<String, Obj>,
    ) -> Result<Vec<Vec<Instruction>>> {
        let mut results = Vec::new();

        let expanded_statements = self.expand_expressions(exprs)?;

        debug!(
            "Generating instructions for the expression: {:?}",
            expanded_statements
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        );

        debug!("About to expand defines");

        let mut expanded_statements = expanded_statements;

        match self.opt_level {
            OptLevel::Three => loop {
                let mut manager = ConstantEvalManager::new(constants.clone(), self.opt_level);
                expanded_statements = manager.run(expanded_statements)?;
                if !manager.changed {
                    break;
                }
            },
            OptLevel::Two => {
                expanded_statements =
                    ConstantEvalManager::new(constants.clone(), self.opt_level)
                        .run(expanded_statements)?;
            }
            _ => {}
        }

        let expanded_statements = flatten_begins_and_expand_defines(expanded_statements);

        debug!(
            "Successfully expanded defines: {:?}",
            expanded_statements
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        );

        let statements_without_structs =
            self.debug_extract_structs(expanded_statements, &mut results)?;

        self.generate_debug_instructions(statements_without_structs, results)
    }

    pub fn emit_instructions_from_exprs(
        &mut self,
        exprs: Vec<AstNode>,
        constants: ImHashMap<String, Obj>,
    ) -> Result<Vec<Vec<Ins>>> {
        let mut results = Vec::new();

        let expanded_statements = self.expand_expressions(exprs)?;

        debug!(
            "Generating instructions for the expression: {:?}",
            expanded_statements
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        );

        let mut expanded_statements = expanded_statements;

        match self.opt_level {
            OptLevel::Three => loop {
                let mut manager = ConstantEvalManager::new(constants.clone(), self.opt_level);
                expanded_statements = manager.run(expanded_statements)?;

                if !manager.changed {
                    break;
                }
            },
            OptLevel::Two => {
                expanded_statements =
                    ConstantEvalManager::new(constants.clone(), self.opt_level)
                        .run(expanded_statements)?;
            }
            _ => {}
        }

        debug!("About to expand defines");
        let expanded_statements = flatten_begins_and_expand_defines(expanded_statements);

        debug!(
            "Successfully expanded defines: {:?}",
            expanded_statements
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        );

        let statements_without_structs = self.extract_structs(expanded_statements, &mut results)?;

        // println!(
        //     "{}",
        //     statements_without_structs
        //         .iter()
        //         .map(|x| x.to_pretty(60))
        //         .join("\n\n")
        // );

        self.generate_dense_instructions(statements_without_structs, results)
    }
}
