use std::sync::Arc;

use aion_state::prelude::{RegistryAcquireAccess, RegistryAcquireAccessResult, RegistryReleaseAccessResult};

use crate::prelude::{AccessResult, DerivedResult, ProgramId, ProgramRegistry, ProgramRegistryAcquireProgram, ProgramRegistryReleaseProgram, ResolvedResource, ResourceAccess, ResourceId, UserId, UserPassword, ValuePassword};

pub struct FinalisedAccess {
    pub program_id: Option<ProgramId>,
    pub program_password: Option<ValuePassword>,

    pub user_details: Option<(UserId, UserPassword)>,

    pub resource_id: ResourceId,
    pub resource_access: ResourceAccess,
    pub resource_password: Option<ValuePassword>,
}

impl FinalisedAccess {
    pub fn derive<'a>(self, program_registry: &'a Arc<ProgramRegistry>) -> DerivedResult<'a> {
        let program_id = match self.program_id {
            Some(program_id) => program_id,
            None => program_registry.global_program_id().clone(),
        };

        let user_details = self.user_details.as_ref().map(|(u, p)| (u, p));

        let program_access_result = program_registry.acquire_program(ProgramRegistryAcquireProgram { 
            user_details, 
            program_id: program_id.clone(), 
            program_password: self.program_password.as_ref()
        });

        if let RegistryAcquireAccessResult::Found(access_result) = program_access_result {
            let AccessResult::Shared(program) = access_result else { unreachable!() };
            
            let resource_access_result = program.acquire_access(RegistryAcquireAccess {
                user_details,
                resource_id: self.resource_id.clone(),
                access: self.resource_access.clone(),
                password: self.resource_password.as_ref()
            });
            
            if let RegistryAcquireAccessResult::Found(access_result) = resource_access_result {
                DerivedResult::Complete(ResolvedResource::new(
                    access_result,
                    Arc::clone(program_registry),
                    Arc::clone(program),
                    program_id,
                    self.resource_access,
                    self.resource_id,
                    user_details.map(|(user_id, user_password)| (user_id.clone(), user_password.clone())),
                ))
            } else {
                assert!(
                    matches!(
                        // Safety
                        // We do not use program any further
                        // and we do not store it
                        unsafe {
                            program_registry.release_program(&ProgramRegistryReleaseProgram {
                                program_id: &program_id,
                            })
                        },
                        RegistryReleaseAccessResult::Ok
                    )
                );
                DerivedResult::ResourceAccessNotFound(resource_access_result)
            }
        } else {
            assert!(
                matches!(
                    // Safety
                    // We do not use program any further
                    // and we do not store it
                    unsafe {
                        program_registry.release_program(&ProgramRegistryReleaseProgram {
                            program_id: &program_id,
                        })
                    },
                    RegistryReleaseAccessResult::Ok
                )
            );

            DerivedResult::ProgramAccessNotFound(program_access_result)
        }
    }
}