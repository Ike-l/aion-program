use std::sync::Arc;

use aion_state::prelude::RegistryAcquireAccessResult;

use crate::prelude::{AccessResult, Program, ResolvedResource, Resource, ResolveResourceError};

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