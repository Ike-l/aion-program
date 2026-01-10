use std::sync::Arc;

use crate::{access::{Access, access_result::AccessResult}, ids::{resource_id::ResourceId, resource_key_id::ResourceKeyId}, program_registry::program::{Program, resource::{Resource, casted_resource::CastedResource, stored_resource::StoredResource}}};

pub struct ResolvedResource<'a> {
    access_result: AccessResult<'a, Resource>,

    source: Arc<Program>,
    access: Access<StoredResource, Resource>,
    access_resource_id: ResourceId, 
    access_key_id: Option<ResourceKeyId>,
}

impl<'a> ResolvedResource<'a> {
    pub fn new(
        access_result: AccessResult<'a, Resource>,
        source: Arc<Program>,
        access: Access<StoredResource, Resource>,
        access_resource_id: ResourceId, 
        access_key_id: Option<ResourceKeyId>,
    ) -> Self {
        Self {
            access_result,
            source,
            access,
            access_resource_id,
            access_key_id
        }
    }

    pub unsafe fn resolve<Y: 'static>(self) -> Result<CastedResource<'a, Y>, Self> {
        let Self {
            access_result,
            source,
            access,
            access_resource_id,
            access_key_id
        } = self;

        match unsafe { access_result.resolve() } {
            Ok(access_result) => Ok(CastedResource::new(access_result, source, access, access_resource_id, access_key_id)),
            Err(access_result) => Err(Self::new(access_result, source, access, access_resource_id, access_key_id)),
        }
    }
}

