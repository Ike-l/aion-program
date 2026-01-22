use std::{fmt::Debug, sync::Arc};

use aion_state::prelude::{Registry, RegistryAccessPermission, RegistryAccessResult};

use crate::prelude::{AccessResult, Resource, ResourceAccess, ResourceId, ResourceKeyId, ResourceReserverId, StoredResource};

pub mod resource;

#[derive(Debug, Default, small_derive_deref::Deref, small_derive_deref::DerefMut)]
pub struct StoredProgram {
    program: Arc<Program>
}

#[derive(Default)]
pub struct Program {
    resource_registry: Registry<ResourceId, ResourceReserverId, ResourceAccess, ResourceId, ResourceKeyId, Box<StoredResource>>
}

impl Program {
    pub fn contains_resources(&self, resources: &Vec<ResourceId>) -> bool {
        !resources.iter().any(|resource| !self.resource_registry.contains_resource(resource))
    }

    pub fn permits_accesses(&self, accesses: &Vec<(&ResourceId, &ResourceAccess, Option<&ResourceReserverId>, Option<&ResourceKeyId>)>) -> bool {
        !accesses
            .iter()
            .any(|(
                resource_id, 
                access, 
                reserver_id, 
                key
            )| 
                !matches!(self.resource_registry.permits_access(resource_id, access, *reserver_id, *key), RegistryAccessPermission::Ok))
    }

    pub unsafe fn deaccess(&self, resource_id: &ResourceId, access: &ResourceAccess, key: Option<&ResourceKeyId>) {
        unsafe { self.resource_registry.deaccess(resource_id, access, key) };
    }

    pub fn access(
        &self,
        resource_id: ResourceId,
        access: ResourceAccess,
        resource_reserver_id: Option<&ResourceReserverId>,
        resource_key: Option<&ResourceKeyId>,
    ) -> RegistryAccessResult<AccessResult<'_, Resource>> {
        self.resource_registry.access(
            resource_id, 
            access, 
            resource_reserver_id, 
            resource_key
        )
    }

    // pub fn resolve<T: Injected>(
    //     self: &Arc<Self>,
    //     resource_ids: Vec<Option<ResourceId>>,
    //     resource_reserver_id: Option<&ResourceReserverId>,
    //     resource_key_ids: Vec<Option<&ResourceKeyId>>,
    // ) -> ProgramResolveResult<'_, T> {
    //     let accesses = T::accesses(resource_ids);

    //     // Attempt all accesses
    //     let access_results = accesses
    //         .iter()
    //         .enumerate()
    //         .map(|(i, (resource_id, access))| {
    //             let resource_key = resource_key_ids.get(i).unwrap_or(&None);
    //             self.resource_registry
    //                 .access(
    //                     resource_id.clone(), 
    //                     access.clone(), 
    //                     resource_reserver_id, 
    //                     *resource_key
    //                 )
    //     }).collect::<Vec<_>>();

    //     // If any access fails:
    //     //  Deaccess
    //     //  return Failure
    //     if access_results.iter().any(|access_result| {
    //         !matches!(access_result, RegistryAccessResult::Found(_))
    //     }) {
    //         for (i, (resource_id, access)) in accesses.into_iter().enumerate() {
    //             let resource_key = resource_key_ids.get(i).unwrap_or(&None);

    //             unsafe { self.deaccess(&resource_id, &access, *resource_key) };
    //         }

    //         return ProgramResolveResult::AccessFailure;
    //     }

    //     let resolved_resources = access_results
    //         .into_iter()
    //         .zip(accesses.iter())
    //         .enumerate()
    //         .map(|(i, (access_result, (resource_id, access)))| {
    //             let RegistryAccessResult::Found(access_result) = access_result else { unreachable!() };

    //             let resource_key = resource_key_ids.get(i).unwrap_or(&None);

    //             ResolvedResource::new(access_result, Arc::clone(&self), access.clone(), resource_id.clone(), resource_key.cloned())
    //         });

    //     let resolve_result = T::resolve(resolved_resources.collect());

    //     if resolve_result.is_err() {
    //         for (i, (resource_id, access)) in accesses.into_iter().enumerate() {
    //             let resource_key = resource_key_ids.get(i).unwrap_or(&None);

    //             unsafe { self.deaccess(&resource_id, &access, *resource_key) };
    //         }
    //     }

    //     ProgramResolveResult::Found(resolve_result)
    // }
}

impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Program>")
    }
}