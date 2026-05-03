use std::sync::Arc;

use crate::prelude::Program;

pub mod program;

pub struct StoredProgram {
    program: Arc<Program>
}

impl StoredProgram {
    pub fn new(program: Program) -> Self {
        Self {
            program: Arc::new(program)
        }
    }

    pub fn get(&self) -> &Program {
        &self.program
    }
}