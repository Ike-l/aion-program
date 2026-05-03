use std::collections::HashMap;

use crate::prelude::{Program, ProgramId};

pub mod program;
pub mod program_id;

pub struct ProgramRegistry {
    global_program_id: ProgramId,
    programs: HashMap<ProgramId, Program>
}