use std::sync::Arc;

use tracing::span;

use aion_state::prelude::ReleasingResult;
use crate::prelude::{Access, AccessResult, FUNCTION_LEVEL, Program, ProgramId, ProgramRegistry, ProgramRegistryReleaseResource, Programs, Resource, ResourceId, StoredProgram, UserId, UserPassword};

pub struct CastedResource<'a, T> {
    access_result: Option<ReleasingResult<Resource, AccessResult<'a, T>, Program>>,
    
    program_registry: Arc<ProgramRegistry>,
    program: Option<ReleasingResult<StoredProgram, AccessResult<'a, StoredProgram>, Programs>>,
    program_id: ProgramId,

    resource_access: Access,
    resource_id: ResourceId,
    user_details: Option<(UserId, UserPassword)>
}

impl<'a, T> CastedResource<'a, T> {
    pub fn new(
        access_result: ReleasingResult<Resource, AccessResult<'a, T>, Program>,
        program_registry: Arc<ProgramRegistry>,
        program: ReleasingResult<StoredProgram, AccessResult<'a, StoredProgram>, Programs>,
        program_id: ProgramId,
        resource_access: Access,
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