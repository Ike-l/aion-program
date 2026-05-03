use aion_state::prelude::Registry;

use crate::prelude::{FinalisedAccess, Injection, ProgramId, PromptedAccesses};

pub mod prompted_accesses;

pub struct ProgramRegistry {
    global_program_id: ProgramId,
    programs: Registry<ProgramStorage>
}

impl ProgramRegistry {
    pub fn resolve<T: Injection>(&self, prompted_accesses: Vec<PromptedAccesses>) {

        let access_builders = prompted_accesses.into_iter().map(|prompted_accesses| prompted_accesses.with(self.global_program_id, false)).collect();

        let submitted_accesses = T::submit_access(access_builders);
    
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