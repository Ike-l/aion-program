use std::sync::Arc;

use aion_state::prelude::RegistryReleaseAccess;

use crate::prelude::{AccessResult, Program, ResourceAccess, ResourceId};

pub struct CastedResource<'a, T> {
    access_result: AccessResult<'a, T>,
    
    source: Arc<Program>,

    resource_access: ResourceAccess,
    resource_id: ResourceId,
}

impl<'a, T> CastedResource<'a, T> {
    pub fn new(
        access_result: AccessResult<'a, T>,
        source: Arc<Program>,
        resource_access: ResourceAccess,
        resource_id: ResourceId,
    ) -> Self {
        Self {
            access_result,
            source,
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
        todo!("release access for program");

        unsafe { self.source.release_access(RegistryReleaseAccess {
            resource_id: &self.resource_id,
            access: &self.resource_access
        } ) };
    }
}