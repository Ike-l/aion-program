use crate::prelude::Program;

pub mod program;

pub struct StoredProgram {
    program: Program
}

impl StoredProgram {
    pub fn new(program: Program) -> Self {
        Self {
            program
        }
    }

    pub fn get(&self) -> &Program {
        &self.program
    }
}