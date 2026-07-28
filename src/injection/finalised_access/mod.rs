use std::sync::Arc;

use aion_state::prelude::{Releaser, RegistryReleasingAcquireAccess};
use tracing::{Level, event, span};

use crate::prelude::{Access, AccessResult, DerivedError, FUNCTION_LEVEL, Program, ProgramId, ProgramRegistry, ProgramRegistryAcquireProgram, ResolvedResource, Resource, ResourceId, UserId, UserPassword, ValuePassword, trace_function};

#[derive(Clone)]
pub struct FinalisedAccess {
    pub program_id: Option<ProgramId>,
    pub program_password: Option<ValuePassword>,

    pub user_details: Option<(UserId, UserPassword)>,

    pub resource_id: ResourceId,
    pub resource_access: Access,
    pub resource_password: Option<ValuePassword>,
}

impl FinalisedAccess {
    pub fn derive<'a>(&self, program_registry: &'a Arc<ProgramRegistry>) -> Result<ResolvedResource<'a>, DerivedError> {
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
            user_details: self.user_details.clone(), 
            program_id: program_id.clone(), 
            program_password: self.program_password.clone()
        });

        match program_access_result {
            Ok(program_access_result) => {
                let AccessResult::Shared(program) = program_access_result.as_ref().unwrap() else { unreachable!() };
                let span = span!(
                    FUNCTION_LEVEL, 
                    "Access",
                    resource_id =? self.resource_id,
                    access =? self.resource_access,
                    password_is_some = self.resource_password.is_some()
                );
                let _enter = span.enter();
        
                let resource_access_result = <Program as Releaser<Resource>>::acquire_access(&program, RegistryReleasingAcquireAccess {
                    user_details: self.user_details.clone(),
                    resource_id: self.resource_id.clone(),
                    access: self.resource_access.clone(),
                    password: self.resource_password.clone()
                });
                
                match resource_access_result {
                    Ok(access_result) => {
                        // store program access_result
                        Ok(ResolvedResource::new(
                            access_result,
                            Arc::clone(program_registry),
                            program_access_result,
                            program_id,
                            self.resource_access.clone(),
                            self.resource_id.clone(),
                            user_details.map(|(user_id, user_password)| (user_id.clone(), user_password.clone())),
                        ))
                    },
                    Err(err) => {
                        event!(Level::WARN, "{}", err);
                        Err(DerivedError::ResourceAccessNotFound(err))
                    },
                }
            },
            Err(err) => {
                event!(Level::WARN, "{}", err);    
                Err(DerivedError::ProgramAccessNotFound(err))
            }
        }
    }
}