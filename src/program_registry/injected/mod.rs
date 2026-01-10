use crate::{access::Access, ids::{resource_id::ResourceId}, program_registry::program::resource::{Resource, resolved_resource::ResolvedResource, stored_resource::StoredResource}};

pub mod injected_primitives;
pub mod injection_error;

use injection_error::InjectionError;

pub trait Injected {
    type Item<'new>;

    fn access() -> Access<StoredResource, Resource>;
    fn resource_id(resource_id: Option<ResourceId>) -> ResourceId;

    fn resolve<'a>(
        resolved_resource: ResolvedResource<'a>
    ) -> Result<Self::Item<'a>, InjectionError>;
}