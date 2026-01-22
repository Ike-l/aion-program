use std::sync::Arc;

use crate::prelude::{AccessResult, ResourceAccess, ResourceId, ResourceKeyId, Program, Resource, CastedResource};

pub struct ResolvedResource<'a> {
    access_result: AccessResult<'a, Resource>,

    source: Arc<Program>,
    access: ResourceAccess,
    access_resource_id: ResourceId, 
    access_key_id: Option<ResourceKeyId>,
}


// Cant put even a manual drop on ResolvedResource because in Injected::resolve the user could drop
// and then if errors it would cause double free
impl<'a> ResolvedResource<'a> {
    pub fn new(
        access_result: AccessResult<'a, Resource>,
        source: Arc<Program>,
        access: ResourceAccess,
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

    pub unsafe fn cast<Y: 'static>(self) -> Result<CastedResource<'a, Y>, Self> {
        let Self {
            access_result,
            source,
            access,
            access_resource_id,
            access_key_id
        } = self;

        match unsafe { access_result.cast::<Y>() } {
            Ok(access_result) => Ok(CastedResource::new(access_result, source, access, access_resource_id, access_key_id)),
            Err(access_result) => Err(Self::new(access_result, source, access, access_resource_id, access_key_id)),
        }
    }
}
