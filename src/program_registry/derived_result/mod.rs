use std::sync::Arc;

use aion_state::prelude::RegistryAcquireAccessResult;

use crate::prelude::{AccessResult, Program, ResolveResourceError, ResolvedResource, Resource, ResourceId, UserId, UserPassword};

pub enum DerivedResult<'a> {
    Complete(ResolvedResource<'a>),
    ResourceAccessNotFound(RegistryAcquireAccessResult<AccessResult<'a, Resource>>),
    ProgramAccessNotFound(RegistryAcquireAccessResult<AccessResult<'a, Arc<Program>>>)
}

impl<'a> TryFrom<DerivedResult<'a>> for ResolvedResource<'a> {
    type Error = ResolveResourceError;

    fn try_from(value: DerivedResult<'a>) -> Result<Self, Self::Error> {
        match value {
            DerivedResult::Complete(resolved_resource) => Ok(resolved_resource),
            _ => Err(ResolveResourceError::Resolving)
        }    
    }
}

impl DerivedResult<'_> {
    pub fn user_details(&self) -> Option<&Option<(UserId, UserPassword)>> {
        match self {
            DerivedResult::Complete(resolved_resource) => Some(resolved_resource.user_details()),
            _ => None
        }
    }

    pub fn resource_id(&self) -> Option<&Option<ResourceId>> {
        match self {
            DerivedResult::Complete(resolved_resource) => Some(resolved_resource.resource_id()),
            _ => None
        }
    }
}