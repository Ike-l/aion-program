use std::sync::Arc;

use aion_state::prelude::RegistryReleaseAccessResult;
use tracing::span;

use crate::prelude::{AccessResult, CastedResource, FUNCTION_LEVEL, Program, ProgramId, ProgramRegistry, ProgramRegistryReleaseResource, Resource, ResourceAccess, ResourceId, UserId, UserPassword};

pub mod casted_resource;

pub struct ResolvedResource<'a> {
    access_result: Option<AccessResult<'a, Resource>>,
    
    program_registry: Option<Arc<ProgramRegistry>>,
    program: Option<Arc<Program>>,
    program_id: Option<ProgramId>,

    resource_access: Option<ResourceAccess>,
    resource_id: Option<ResourceId>,
    user_details: Option<(UserId, UserPassword)>
}

impl<'a> ResolvedResource<'a> {
    pub fn new(
        access_result: AccessResult<'a, Resource>,
        program_registry: Arc<ProgramRegistry>,
        program: Arc<Program>,
        program_id: ProgramId,
        resource_access: ResourceAccess,
        resource_id: ResourceId,
        user_details: Option<(UserId, UserPassword)>
    ) -> Self {
        Self {
            access_result: Some(access_result),
            program_registry: Some(program_registry),
            program: Some(program),
            program_id: Some(program_id),
            resource_access: Some(resource_access),
            resource_id: Some(resource_id),
            user_details
        }
    }

    pub fn user_details(&self) -> &Option<(UserId, UserPassword)> {
        &self.user_details
    }

    pub fn resource_id(&self) -> &Option<ResourceId> {
        &self.resource_id
    }

    // NEVER MAKE PUBLIC
    fn take_all(&mut self) -> Option<(
        AccessResult<'a, Resource>,
        Arc<ProgramRegistry>,
        Arc<Program>,
        ProgramId,
        ResourceAccess,
        ResourceId,
        Option<(UserId, UserPassword)>
    )> {
        Some((
            self.access_result.take()?,
            self.program_registry.take()?,
            self.program.take()?,
            self.program_id.take()?,
            self.resource_access.take()?,
            self.resource_id.take()?,
            self.user_details.take()
        ))
    }

    pub fn cast<Y: 'static>(mut self) -> Result<CastedResource<'a, Y>, Self> {
        let (
            access_result,
            program_registry,
            program,
            program_id,
            resource_access,
            resource_id,
            user_details
        ) = self.take_all().unwrap();

        match access_result.cast::<Y>() {
            Ok(access_result) => Ok(CastedResource::new(access_result, program_registry, program, program_id, resource_access, resource_id, user_details)),
            Err(access_result) => Err(Self::new(access_result, program_registry, program, program_id, resource_access, resource_id, user_details))
        }
    }
}

impl Drop for ResolvedResource<'_> {
    fn drop(&mut self) {
        let span = span!(
            FUNCTION_LEVEL, 
            "ResolvedResource Drop",
            program_id =? self.program_id,
            resource_id =? self.resource_id,
            resource_access =? self.resource_access,
        );
        let _enter = span.enter();

        assert!(
            matches!(
                // Safety
                // We do not use the resources any further (in the drop)
                unsafe {
                    self.program_registry.as_ref().unwrap().release_resource(&ProgramRegistryReleaseResource {
                        program: self.program.as_ref().unwrap(), 
                        program_id: self.program_id.as_ref().unwrap(),
                        resource_id: self.resource_id.as_ref().unwrap(), 
                        resource_access: self.resource_access.as_ref().unwrap()
                    })
                },
                (RegistryReleaseAccessResult::Ok, RegistryReleaseAccessResult::Ok)
            )
        );
    }
}