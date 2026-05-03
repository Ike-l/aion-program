use crate::prelude::{DerivedResource, FinalisedAccess, AccessBuilder, ResolveResourceError};

pub mod access_builder;
pub mod finalised_access;

pub mod derived_resource;
pub mod resolve_resource_error;

pub trait Injection {
    type Item<'new>;

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Vec<FinalisedAccess> {
        todo!()
    }

    fn resolve_access<'new>(derived_resources: Vec<DerivedResource>) -> Result<Self::Item<'new>, ResolveResourceError> {
        todo!()
    }
}