use std::sync::Arc;

use aion_state::prelude::DeaccessingResult;
use tracing::span;

use crate::prelude::{AccessResult, AutoRegistry, FUNCTION_LEVEL, Program, ProgramAccess, ProgramId, ProgramRegistry, ProgramRegistryReleaseResource, ResourceAccess, ResourceId, StoredProgram, UserId, UserPassword};

pub struct CastedResource<'a, T> {
    access_result: Option<DeaccessingResult<AccessResult<'a, T>, Program>>,
    
    program_registry: Arc<ProgramRegistry>,
    program: Option<DeaccessingResult<AccessResult<'a, StoredProgram>, AutoRegistry<ProgramId, StoredProgram, ProgramAccess>>>,
    program_id: ProgramId,

    resource_access: ResourceAccess,
    resource_id: ResourceId,
    user_details: Option<(UserId, UserPassword)>
}

impl<'a, T> CastedResource<'a, T> {
    pub fn new(
        access_result: DeaccessingResult<AccessResult<'a, T>, Program>,
        program_registry: Arc<ProgramRegistry>,
        program: DeaccessingResult<AccessResult<'a, StoredProgram>, AutoRegistry<ProgramId, StoredProgram, ProgramAccess>>,
        program_id: ProgramId,
        resource_access: ResourceAccess,
        resource_id: ResourceId,
        user_details: Option<(UserId, UserPassword)>
    ) -> Self {
        Self {
            access_result: Some(access_result),
            program_registry,
            program: Some(program),
            program_id,
            resource_access,
            resource_id,
            user_details
        }
    }

    pub fn user_details(&self) -> &Option<(UserId, UserPassword)> {
        &self.user_details
    }

    pub fn as_ref(&self) -> Option<&T> {
        self.access_result.as_ref().unwrap().as_ref().unwrap().as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.access_result.as_mut().unwrap().as_mut().unwrap().as_mut()
    }
}

impl<'a, T> Drop for CastedResource<'a, T> {
    fn drop(&mut self) {
        let span = span!(
            FUNCTION_LEVEL, 
            "CastedResource Drop",
            program_id =? self.program_id,
            resource_id =? self.resource_id,
            resource_access =? self.resource_access,
        );
        let _enter = span.enter();

        drop(self.access_result.take());
        drop(self.program.take());

        // Safety
        // We do not use the resources any further (in the drop)
        unsafe {
            self.program_registry.notify_of_release(&ProgramRegistryReleaseResource {
                program_id: &self.program_id,
                resource_id: &self.resource_id, 
            })
        };
    }
}