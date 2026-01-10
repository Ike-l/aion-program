use std::sync::Arc;

use crate::{access::{Access, access_result::AccessResult}, ids::{resource_id::ResourceId, resource_key_id::ResourceKeyId}, program_registry::program::{Program, resource::{Resource, stored_resource::StoredResource}}};

pub struct CastedResource<'a, T> {
    access_result: AccessResult<'a, T>,

    source: Arc<Program>,
    access: Access<StoredResource, Resource>,
    access_resource_id: ResourceId, 
    access_key_id: Option<ResourceKeyId>,
}

impl<'a, T> CastedResource<'a, T> {
    pub fn new(
        access_result: AccessResult<'a, T>,
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

    pub fn as_ref(&self) -> Option<&T> {
        self.access_result.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.access_result.as_mut()
    }

    pub fn take(&mut self) -> Option<Option<T>> {
        self.access_result.take()
    }
}

impl<'a, T> Drop for CastedResource<'a, T> {
    fn drop(&mut self) {
        unsafe { self.source.deaccess(&self.access_resource_id, &self.access, self.access_key_id.as_ref()) };
    }
}