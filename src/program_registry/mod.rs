use aion_state::prelude::Registry;

use crate::prelude::{AccessStorage, BlacklistStorage, ControlStorage, CredentialStorage, FinalisedAccess, Injection, ProgramAccess, ProgramId, PromptedProgramAccess, RegistryStorage, ReservationStorage, StoredProgram, WhitelistStorage};

pub mod prompted_program_access;

pub mod program_id;
pub mod stored_program;
pub mod program_access;

pub struct ProgramRegistry {
    global_program_id: ProgramId,
    programs: Registry<
        RegistryStorage<ProgramId, StoredProgram>,
        ReservationStorage<ProgramId, ProgramAccess>,
        AccessStorage<ProgramId, ProgramAccess>,
        CredentialStorage,
        WhitelistStorage<ProgramId, ProgramAccess>,
        BlacklistStorage<ProgramId, ProgramAccess>,
        ControlStorage<ProgramId>
    >
}

impl ProgramRegistry {
    pub fn resolve<T: Injection>(&self, prompted_program_accesses: Vec<PromptedProgramAccess>) {

        let access_builders = prompted_program_accesses.into_iter().map(|prompted_accesses| prompted_accesses.with(self.global_program_id.clone(), false)).collect();

        let submitted_accesses = T::submit_access(access_builders);
    
        for FinalisedAccess { 
            program_id, 
            user_details, 
            resource_id, 
            resource_password, 
            resource_access 
        } in submitted_accesses {
            // self.programs.acquire_access()
        }   
    }
}