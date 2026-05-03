use std::sync::Arc;

use crate::prelude::Program;

pub mod program;

pub struct StoredProgram {
    program: Arc<Program>
}