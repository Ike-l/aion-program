use std::sync::Arc;

use crate::prelude::{Program};

pub mod program;

pub type StoredProgram = Arc<Program>;

pub trait StoredProgramTrait {
    type Program;

    fn new(program: Self::Program) -> Self;

    fn get(&self) -> &Self;
}

impl StoredProgramTrait for StoredProgram {
    type Program = Arc<Program>;

    fn new(program: Self::Program) -> Self {
        program
    }

    fn get(&self) -> &Self {
        self
    }
}