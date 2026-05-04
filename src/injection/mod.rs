use crate::prelude::{AccessBuilder, FinalisedAccess, ResolveResourceError, DerivedResult, AccessSubmissionError};

pub mod access_builder;
pub mod finalised_access;

pub mod derived_resource;

pub mod resolve_resource_error;
pub mod access_submission_error;

pub trait Injection {
    type Item<'new>;

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError>;
    fn resolve_access<'new>(derived_results: Vec<DerivedResult<'new>>) -> Result<Self::Item<'new>, ResolveResourceError>;
}