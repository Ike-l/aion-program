use std::sync::Arc;

use crate::prelude::{AccessResult, CastedResource, Program, Resource, ResourceAccess, ResourceId};

pub mod casted_resource;

pub struct ResolvedResource<'a> {
    access_result: AccessResult<'a, Resource>,
    
    source: Arc<Program>,
    resource_access: ResourceAccess,
    resource_id: ResourceId,
}

impl<'a> ResolvedResource<'a> {
    pub fn new(
        access_result: AccessResult<'a, Resource>,
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

    pub fn cast<Y: 'static>(self) -> Result<CastedResource<'a, Y>, Self> {
        let Self {
            access_result,
            source,
            resource_access,
            resource_id
        } = self;

        match access_result.cast::<Y>() {
            Ok(access_result) => Ok(CastedResource::new(access_result, source, resource_access, resource_id)),
            Err(access_result) => Err(Self::new(access_result, source, resource_access, resource_id))
        }
    }
}