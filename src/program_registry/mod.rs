use std::collections::HashMap;

use crate::prelude::{AccessBuilder, FinalisedAccess, Injection, Program, ProgramId};

pub mod program;
pub mod program_id;

pub struct ProgramRegistry {
    global_program_id: ProgramId,
    programs: HashMap<ProgramId, Program>
}

impl ProgramRegistry {
    pub fn resolve<T: Injection>(&self, prompted_accesses: Vec<AccessBuilder>) {
        let submitted_accesses = T::submit_access(prompted_accesses);
    
        for FinalisedAccess { 
            program_id, 
            user_details, 
            resource_id, 
            resource_password, 
            access 
        } in submitted_accesses {
            
        }   
    }
}