use std::sync::Arc;

use crate::prelude::{AccessResult, CastedResource, Program, ProgramId, ProgramRegistry, Resource, ResourceAccess, ResourceId};

pub mod casted_resource;

pub struct ResolvedResource<'a> {
    access_result: AccessResult<'a, Resource>,
    
    program_registry: Arc<ProgramRegistry>,
    program: Arc<Program>,
    program_id: &'a ProgramId,

    resource_access: ResourceAccess,
    resource_id: ResourceId,
}

impl<'a> ResolvedResource<'a> {
    pub fn new(
        access_result: AccessResult<'a, Resource>,
        program_registry: Arc<ProgramRegistry>,
        program: Arc<Program>,
        program_id: &'a ProgramId,
        resource_access: ResourceAccess,
        resource_id: ResourceId,
    ) -> Self {
        Self {
            access_result,
            program_registry,
            program,
            program_id,
            resource_access,
            resource_id
        }
    }

    pub fn cast<Y: 'static>(self) -> Result<CastedResource<'a, Y>, Self> {
        let Self {
            access_result,
            program_registry,
            program,
            program_id,
            resource_access,
            resource_id
        } = self;

        match access_result.cast::<Y>() {
            Ok(access_result) => Ok(CastedResource::new(access_result, program_registry, program, program_id, resource_access, resource_id)),
            Err(access_result) => Err(Self::new(access_result, program_registry, program, program_id, resource_access, resource_id))
        }
    }
}