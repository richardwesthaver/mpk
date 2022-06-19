use super::cons::ConstantMap;
use crate::Result;
use crate::i::Ins;
use serde::{Deserialize, Serialize};

pub struct ProgramBuilder(Vec<Vec<Ins>>);

impl ProgramBuilder {
    pub fn new() -> Self {
        ProgramBuilder(Vec::new())
    }

    pub fn push(&mut self, val: Vec<Ins>) {
        self.0.push(val);
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableProgram {
    pub instructions: Vec<Vec<Ins>>,
    pub constant_map: Vec<u8>,
}

impl SerializableProgram {
    pub fn write_to_file(&self, filename: &str) -> Result<()> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::create(format!("{}.txt", filename)).unwrap();

        let buffer = bincode::serialize(self).unwrap();

        file.write_all(&buffer)?;
        Ok(())
    }

    pub fn read_from_file(&self, filename: &str) -> Result<Self> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open(format!("{}.txt", filename)).unwrap();

        let mut buffer = Vec::new();

        let _ = file.read(&mut buffer).unwrap();

        let program: SerializableProgram = bincode::deserialize(&buffer).unwrap();

        Ok(program)
    }
}

/// an mk program
///
/// The program holds the instructions and the constant map, serialized to bytes
pub struct Program {
    pub instructions: Vec<Vec<Ins>>,
    pub constant_map: ConstantMap,
}

impl Program {
    pub fn new(instructions: Vec<Vec<Ins>>, constant_map: ConstantMap) -> Self {
        Program {
            instructions,
            constant_map,
        }
    }

    pub fn into_serializable_program(self) -> Result<SerializableProgram> {
        Ok(SerializableProgram {
            instructions: self.instructions,
            constant_map: self.constant_map.to_bytes()?,
        })
    }
}
