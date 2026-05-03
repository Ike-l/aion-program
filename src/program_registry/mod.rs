use aion_state::prelude::{Registry, RegistryAcquireAccess};

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
            program_password,
            user_details, 
            resource_id, 
            resource_password, 
            resource_access 
        } in submitted_accesses {
            let user_details = user_details.as_ref().map(|(user_id, user_password)| (user_id, user_password));

            let result = self.programs.acquire_access(RegistryAcquireAccess {
                user_details,
                resource_id: program_id,
                access: ProgramAccess::Shared(1),
                password: program_password.as_ref(),
            });

            
        }   
    }
}