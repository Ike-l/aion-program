use std::sync::Arc;

use hecs::Entity;

use crate::prelude::{AccessBuilder, AccessSubmissionError, DerivedResult, FinalisedAccess, ProgramRegistry, ResolveResourceError};

pub mod access_builder;
pub mod finalised_access;

pub mod resolve_resource_error;
pub mod access_submission_error;

pub mod primitives;

/// MUST BE SIDE-EFFECT FREE
/// 
/// Can have side effects if it doesnt assume its actually being used
pub trait Injection {
    type Item<'new>;

    fn claim_manual_access_builders(accesses: Vec<&AccessBuilder>) -> Vec<usize>;

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError>;
    fn resolve_access<'new>(entity: Option<Entity>, program_registry: Arc<ProgramRegistry>, derived_results: Vec<DerivedResult<'new>>) -> Result<Self::Item<'new>, ResolveResourceError>;
}