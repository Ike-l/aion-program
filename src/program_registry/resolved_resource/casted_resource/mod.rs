use std::sync::Arc;

use aion_state::prelude::{RegistryReleaseAccess, RegistryReleaseAccessResult};

use crate::prelude::{AccessResult, Program, ProgramId, ProgramRegistry, ProgramRegistryReleaseAccess, ResourceAccess, ResourceId};

pub struct CastedResource<'a, T> {
    access_result: AccessResult<'a, T>,
    
    program_registry: Arc<ProgramRegistry>,
    program: Arc<Program>,
    program_id: ProgramId,

    resource_access: ResourceAccess,
    resource_id: ResourceId,
}

impl<'a, T> CastedResource<'a, T> {
    pub fn new(
        access_result: AccessResult<'a, T>,
        program_registry: Arc<ProgramRegistry>,
        program: Arc<Program>,
        program_id: ProgramId,
        resource_access: ResourceAccess,
        resource_id: ResourceId,
    ) -> Self {
        Self {
            access_result,
            program_registry,
            program,
            program_id,
            resource_access,
            resource_id
        }
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
        assert!(
            matches!(
                // Safety
                // We do not use the resources any further (in the drop)
                unsafe {
                    self.program.release_access(&RegistryReleaseAccess {
                        resource_id: &self.resource_id,
                        access: &self.resource_access
                    })
                },
                RegistryReleaseAccessResult::Ok
            )
        );

        assert!(
            matches!(
                // Safety
                // We do not use program any further
                // and we do not store it
                unsafe {
                    self.program_registry.release_access(&ProgramRegistryReleaseAccess {
                        program_id: &self.program_id,
                    })
                },
                RegistryReleaseAccessResult::Ok
            )
        );
    }
}