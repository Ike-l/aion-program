use std::sync::Arc;

use aion_state::prelude::{Registry, RegistryAcquireAccess, RegistryAcquireAccessResult};

use crate::prelude::{AccessResult, AccessStorage, BlacklistStorage, ControlStorage, CredentialStorage, FinalisedAccess, Injection, ProgramAccess, ProgramId, PromptedProgramAccess, RegistryStorage, ReservationStorage, ResolvedResource, StoredProgram, WhitelistStorage};

pub mod prompted_program_access;

pub mod program_id;
pub mod stored_program;
pub mod program_access;
pub mod resolved_resource;

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
    
        let resource_results = for FinalisedAccess { 
            program_id, 
            program_password,
            user_details, 
            resource_id, 
            resource_password, 
            resource_access 
        } in submitted_accesses {
            let user_details = user_details.as_ref().map(|(user_id, user_password)| (user_id, user_password));

            let program = self.programs.acquire_access(RegistryAcquireAccess {
                user_details,
                resource_id: program_id,
                access: ProgramAccess::Shared(1),
                password: program_password.as_ref(),
            });  

            if let RegistryAcquireAccessResult::Found(access_result) = program {
                let AccessResult::Shared(program) = access_result else { unreachable!() };
                
                let resource = program.acquire_access(RegistryAcquireAccess {
                    user_details,
                    resource_id: resource_id.clone(),
                    access: resource_access.clone(),
                    password: resource_password.as_ref()
                });

                if let RegistryAcquireAccessResult::Found(access_result) = resource {
                    let resolved_resource = ResolvedResource::new(
                        access_result,
                        Arc::clone(program),
                        resource_access,
                        resource_id,
                    );
                }
            } else {

            }
        };
    }
}