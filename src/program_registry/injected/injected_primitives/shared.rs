use crate::{access::Access, ids::resource_id::ResourceId, program_registry::{injected::{Injected, injection_error::InjectionError}, program::{resource::{Resource, casted_resource::CastedResource, resolved_resource::ResolvedResource, stored_resource::StoredResource}}}};

pub struct Shared<'a, T> {
    resource: CastedResource<'a, T>
}

impl<'a, T> Shared<'a, T> {
    pub fn as_ref(&self) -> &T {
        self.resource.as_ref().expect("Expected Shared Resource")
    }
}

impl<'a, T: 'static> Injected for Shared<'a, T> {
    type Item<'new> = Shared<'new, T>;

    fn access() -> Access<StoredResource, Resource> {
        Access::Shared(1)
    }

    fn resource_id(resource_id: Option<ResourceId>) -> ResourceId {
        resource_id.unwrap_or(ResourceId::raw::<T>())
    }

    fn resolve<'new>(
            resolved_resource: ResolvedResource<'new>
        ) -> Result<Self::Item<'new>, InjectionError> {
        let resource = unsafe { resolved_resource.resolve::<T>().map_err(|_| InjectionError::Resolving) }?;
        
        Ok(Shared {
            resource
        })
    }
}
