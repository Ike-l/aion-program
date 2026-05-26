use std::sync::Arc;

use aion_state::prelude::RegistryReleaseAccessResult;
use tracing::span;

use crate::prelude::{AccessResult, FUNCTION_LEVEL, Program, ProgramId, ProgramRegistry, ProgramRegistryReleaseResource, ResourceAccess, ResourceId, UserId, UserPassword};

pub struct CastedResource<'a, T> {
    access_result: AccessResult<'a, T>,
    
    program_registry: Arc<ProgramRegistry>,
    program: Arc<Program>,
    program_id: ProgramId,

    resource_access: ResourceAccess,
    resource_id: ResourceId,
    user_details: Option<(UserId, UserPassword)>
}

impl<'a, T> CastedResource<'a, T> {
    pub fn new(
        access_result: AccessResult<'a, T>,
        program_registry: Arc<ProgramRegistry>,
        program: Arc<Program>,
        program_id: ProgramId,
        resource_access: ResourceAccess,
        resource_id: ResourceId,
        user_details: Option<(UserId, UserPassword)>
    ) -> Self {
        Self {
            access_result,
            program_registry,
            program,
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
        self.access_result.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.access_result.as_mut()
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

        assert!(
            matches!(
                // Safety
                // We do not use the resources any further (in the drop)
                unsafe {
                    self.program_registry.release_resource(&ProgramRegistryReleaseResource {
                        program: self.program.as_ref(), 
                        program_id: &self.program_id,
                        resource_id: &self.resource_id, 
                        resource_access: &self.resource_access
                    })
                },
                (RegistryReleaseAccessResult::Ok, RegistryReleaseAccessResult::Ok)
            )
        );
    }
}