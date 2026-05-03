use std::sync::Arc;

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
}