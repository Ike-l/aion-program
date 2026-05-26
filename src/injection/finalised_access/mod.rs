use std::sync::Arc;

use aion_state::prelude::{RegistryAcquireAccess, RegistryAcquireAccessResult, RegistryReleaseAccessResult};
use tracing::{Level, event, span};

use crate::prelude::{AccessResult, DerivedResult, FUNCTION_LEVEL, ProgramId, ProgramRegistry, ProgramRegistryAcquireProgram, ProgramRegistryReleaseProgram, ResolvedResource, ResourceAccess, ResourceId, UserId, UserPassword, ValuePassword, trace_function};

#[derive(Clone)]
pub struct FinalisedAccess {
    pub program_id: Option<ProgramId>,
    pub program_password: Option<ValuePassword>,

    pub user_details: Option<(UserId, UserPassword)>,

    pub resource_id: ResourceId,
    pub resource_access: ResourceAccess,
    pub resource_password: Option<ValuePassword>,
}

impl FinalisedAccess {
    pub fn derive<'a>(&self, program_registry: &'a Arc<ProgramRegistry>) -> DerivedResult<'a> {
        trace_function!("FinalisedAccess Derive");

        let program_id = match self.program_id.clone() {
            Some(program_id) => program_id,
            None => program_registry.global_program_id().clone(),
        };

        let user_details = self.user_details.as_ref().map(|(u, p)| (u, p));

        let span = span!(
            FUNCTION_LEVEL, 
            "Program", 
            user_id =? user_details.as_ref().map(|details| details.0), 
            program_id =? program_id, 
            program_password_is_some = self.program_password.is_some()
        );
        let _enter = span.enter();

        let program_access_result = program_registry.acquire_program(ProgramRegistryAcquireProgram { 
            user_details, 
            program_id: program_id.clone(), 
            program_password: self.program_password.as_ref()
        });

        match program_access_result {
            RegistryAcquireAccessResult::Found(access_result) => {
                let AccessResult::Shared(program) = access_result else { unreachable!() };
                let span = span!(
                    FUNCTION_LEVEL, 
                    "Access",
                    resource_id =? self.resource_id,
                    access =? self.resource_access,
                    password_is_some = self.resource_password.is_some()
                );
                let _enter = span.enter();
        
                let resource_access_result = program.acquire_access(RegistryAcquireAccess {
                    user_details,
                    resource_id: self.resource_id.clone(),
                    access: self.resource_access.clone(),
                    password: self.resource_password.as_ref()
                });
                
                match resource_access_result {
                    RegistryAcquireAccessResult::Found(access_result) => {
                        DerivedResult::Complete(ResolvedResource::new(
                            access_result,
                            Arc::clone(program_registry),
                            Arc::clone(program),
                            program_id,
                            self.resource_access.clone(),
                            self.resource_id.clone(),
                            user_details.map(|(user_id, user_password)| (user_id.clone(), user_password.clone())),
                        ))
                    },
                    error @ _ => {
                        event!(Level::WARN, "{}", error);
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
            
                        DerivedResult::ResourceAccessNotFound(error)
                    },
                }
            },
            error @ _ => {
                event!(Level::WARN, "{}", error);
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
    
                DerivedResult::ProgramAccessNotFound(error)
            }
        }
    }
}