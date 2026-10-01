use std::sync::Arc;

use aion_state::prelude::ReleasingResult;
use tracing::{event, span};

use crate::prelude::{Access, AccessResult, CastError, CastedResource, FUNCTION_LEVEL, Program, ProgramId, ProgramRegistry, ProgramRegistryReleaseResource, Programs, Resource, ResourceId, StoredProgram, UserId, UserPassword};

pub mod casted_resource;
pub mod cast_error;

pub struct ResolvedResource<'a> {
    access_result: Option<ReleasingResult<Resource, AccessResult<'a, Resource>, Program>>,
    
    program_registry: Option<Arc<ProgramRegistry>>,
    program: Option<ReleasingResult<StoredProgram, AccessResult<'a, StoredProgram>, Programs>>,
    program_id: Option<ProgramId>,

    resource_access: Option<Access>,
    resource_id: Option<ResourceId>,
    user_details: Option<(UserId, UserPassword)>,

    used: bool,
}

impl<'a> ResolvedResource<'a> {
    pub fn new(
        access_result: ReleasingResult<Resource, AccessResult<'a, Resource>, Program>,
        program_registry: Arc<ProgramRegistry>,
        program: ReleasingResult<StoredProgram, AccessResult<'a, StoredProgram>, Programs>,
        program_id: ProgramId,
        resource_access: Access,
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
            user_details,
            used: false,
        }
    }

    pub fn user_details(&self) -> &Option<(UserId, UserPassword)> {
        &self.user_details
    }

    pub fn resource_id(&self) -> Option<&ResourceId> {
        self.resource_id.as_ref()
    }

    pub fn cast<Y: 'static>(mut self) -> Result<CastedResource<'a, Y>, CastError> {
        self.used = true;

        let cast_result = self.access_result.take().unwrap().update(|access_result| {
            access_result.cast::<Y>()
        });

        match cast_result.as_ref().unwrap() {
            Ok(_) => {},
            Err(err) => return Err(err.clone()),
        }

        let cast_result = cast_result.update(|cast_result| cast_result.unwrap());

        Ok(CastedResource::new(
            cast_result, 
            self.program_registry.take().unwrap(),
            self.program.take().unwrap(), 
            self.program_id.take().unwrap(), 
            self.resource_access.take().unwrap(), 
            self.resource_id.take().unwrap(), 
            self.user_details.take()
        ))
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

        if self.used {
            event!(FUNCTION_LEVEL, "Used");

            return;
        }

        drop(self.access_result.take());
        drop(self.program.take());

        // Safety
        // We do not use the resources any further (in the drop)
        unsafe {
            self.program_registry.as_ref().unwrap().notify_of_release(&ProgramRegistryReleaseResource {
                program_id: self.program_id.as_ref().unwrap(),
                resource_id: self.resource_id.as_ref().unwrap(), 
            })
        };
    }
}