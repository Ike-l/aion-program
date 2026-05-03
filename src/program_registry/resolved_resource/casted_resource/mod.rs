use std::sync::Arc;

use crate::prelude::{AccessResult, ProgramId, ProgramRegistry, ProgramReleaseAccess, ResourceAccess, ResourceId};

pub struct CastedResource<'a, T> {
    access_result: AccessResult<'a, T>,
    
    program_registry: Arc<ProgramRegistry>,
    program_id: ProgramId,

    resource_access: ResourceAccess,
    resource_id: ResourceId,
}

impl<'a, T> CastedResource<'a, T> {
    pub fn new(
        access_result: AccessResult<'a, T>,
        program_registry: Arc<ProgramRegistry>,
        program_id: ProgramId,
        resource_access: ResourceAccess,
        resource_id: ResourceId,
    ) -> Self {
        Self {
            access_result,
            program_registry,
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
        unsafe {
            self.program_registry.release_access(&ProgramReleaseAccess {
                program_id: &self.program_id,
                resource_id: &self.resource_id,
                resource_access: &self.resource_access
            })
        };
    }
}