use std::sync::Arc;

use crate::prelude::Program;

pub mod program;

pub struct StoredProgram {
    program: Arc<Program>
}

impl StoredProgram {
    pub fn new(program: Arc<Program>) -> Self {
        Self {
            program
        }
    }

    pub fn get(&self) -> &Arc<Program> {
        &self.program
    }
}