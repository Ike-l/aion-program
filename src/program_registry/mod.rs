use std::sync::Arc;

use aion_state::prelude::{Registry, RegistryAcquireAccess, RegistryAcquireAccessResult};

use crate::prelude::{AccessResult, AccessStorage, BlacklistStorage, ControlStorage, CredentialStorage, DerivedResult, FinalisedAccess, Injection, ProgramAccess, ProgramId, PromptedProgramAccess, RegistryStorage, ReservationStorage, ResolveResourceError, ResolvedResource, StoredProgram, WhitelistStorage};

pub mod prompted_program_access;

pub mod program_id;
pub mod stored_program;
pub mod program_access;
pub mod resolved_resource;
pub mod derived_result;

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
    pub fn resolve<T: Injection>(&self, prompted_program_accesses: Vec<PromptedProgramAccess>) -> Result<<T as Injection>::Item<'_>, ResolveResourceError> {
        let access_builders = prompted_program_accesses.into_iter().map(|prompted_accesses| prompted_accesses.with(self.global_program_id.clone(), false)).collect();

        let submitted_accesses = T::submit_access(access_builders);
    
        let derived_results = submitted_accesses.into_iter().map(|FinalisedAccess { 
            program_id, 
            program_password,
            user_details, 
            resource_id, 
            resource_password, 
            resource_access 
        }| {
            let user_details = user_details.as_ref().map(|(user_id, user_password)| (user_id, user_password));

            let program_access_result = self.programs.acquire_access(RegistryAcquireAccess {
                user_details,
                resource_id: program_id,
                access: ProgramAccess::Shared(1),
                password: program_password.as_ref(),
            });  

            if let RegistryAcquireAccessResult::Found(access_result) = program_access_result {
                let AccessResult::Shared(program) = access_result else { unreachable!() };
                
                let resource_access_result = program.acquire_access(RegistryAcquireAccess {
                    user_details,
                    resource_id: resource_id.clone(),
                    access: resource_access.clone(),
                    password: resource_password.as_ref()
                });

                if let RegistryAcquireAccessResult::Found(access_result) = resource_access_result {
                    DerivedResult::Complete(ResolvedResource::new(
                        access_result,
                        Arc::clone(program),
                        resource_access,
                        resource_id,
                    ))
                } else {
                    DerivedResult::ResourceAccessNotFound(resource_access_result)
                }
            } else {
                DerivedResult::ProgramAccessNotFound(program_access_result)
            }
        }).collect::<Vec<_>>();

        // deaccess all programs
        let resolve_result = T::resolve_access(derived_results);
        match resolve_result {
            Ok(item) => Ok(item),
            Err(err) => {
                // deaccess all accessed resources
                Err(err)
            },
        }
    }
}