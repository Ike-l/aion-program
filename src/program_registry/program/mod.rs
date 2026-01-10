use std::{fmt::Debug, sync::Arc};

use aion_state::prelude::{AccessMap, Registry, RegistryAccessResult};

use crate::{access::{Access, access_result::AccessResult}, ids::{resource_id::ResourceId, resource_key_id::ResourceKeyId, resource_reserver_id::ResourceReserverId}, program_registry::{injected::Injected, program::{program_results::ProgramResolveResult, resource::resolved_resource::ResolvedResource}}};

pub mod stored_program;
pub mod resource;
pub mod program_results;

use resource::{
    Resource, stored_resource::StoredResource
};

#[derive(Default)]
pub struct Program {
    resource_registry: Registry<ResourceId, ResourceReserverId, Access<StoredResource, Resource>, ResourceId, ResourceKeyId, Box<StoredResource>>
}

impl Program {
    pub unsafe fn deaccess(&self, resource_id: &ResourceId, access: &Access<StoredResource, Resource>, key: Option<&ResourceKeyId>) {
        unsafe { self.resource_registry.deaccess(resource_id, access, key) };
    }

    pub fn resolve<T: Injected>(
        self: &Arc<Self>,
        resource_id: Option<ResourceId>,
        resource_reserver_id: Option<&ResourceReserverId>,
        resource_key_id: Option<&ResourceKeyId>,
    ) -> ProgramResolveResult<'_, T> {
        let resource_id = T::resource_id(resource_id);
        let access = T::access();

        let access_result = self.resource_registry.access(resource_id.clone(), access.clone(), resource_reserver_id, resource_key_id);

        match access_result {
            RegistryAccessResult::Found(access_result) => {
                let resolved_resource = ResolvedResource::new(
                    access_result, 
                    Arc::clone(&self), 
                    access,
                    resource_id,
                    resource_key_id.cloned(),
                );

                let resolve_result = T::resolve(resolved_resource);
                ProgramResolveResult::Found(resolve_result)
            },
            _ => ProgramResolveResult::AccessFailure
        }
    }
}

impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Program>")
    }
}