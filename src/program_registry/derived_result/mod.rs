use std::sync::Arc;

use aion_state::prelude::RegistryAcquireAccessResult;

use crate::prelude::{AccessResult, Program, ResolvedResource, Resource};

pub enum DerivedResult<'a> {
    Complete(ResolvedResource<'a>),
    ResourceAccessNotFound(RegistryAcquireAccessResult<AccessResult<'a, Resource>>),
    ProgramAccessNotFound(RegistryAcquireAccessResult<AccessResult<'a, Arc<Program>>>)
}