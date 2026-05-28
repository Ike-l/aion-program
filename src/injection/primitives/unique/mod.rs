use std::{any::TypeId, ops::{Deref, DerefMut}, sync::Arc};

use hecs::Entity;
use tracing::event;

use crate::prelude::{AccessBuilder, AccessSubmissionError, CastedResource, DerivedError, FUNCTION_LEVEL, FinalisedAccess, Injection, ProgramRegistry, ResolveResourceError, ResolvedResource, ResourceAccess, ResourceId, trace_function};

pub struct Unique<'a, T> {
    resource: CastedResource<'a, T>
}

impl<'a, T> Unique<'a, T> {
    pub fn as_ref(&self) -> &T {
        self.resource.as_ref().expect("Expected Unique Resource")
    }

    pub fn as_mut(&mut self) -> &mut T {
        self.resource.as_mut().expect("Expected Unique Resource")
    }
}

impl<'a, T> Deref for Unique<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()    
    }
}

impl<'a, T> DerefMut for Unique<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()    
    }
}

impl<'a, T: 'static> Injection for Unique<'a, T> {
    type Item<'new> = Unique<'new, T>;

    fn claim_manual_access_builders(_access_builders: Vec<&AccessBuilder>) -> Vec<usize> { 
        trace_function!("Unique Claim Manual Access Builders");

        event!(FUNCTION_LEVEL, "Claiming no access builders");

        vec![] 
    }

    fn submit_access(mut prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
        trace_function!("Unique Submit Access");

        if prompted_accesses.len() == 0 {
            event!(FUNCTION_LEVEL, "Building default submission");

            return Ok(vec![
                AccessBuilder {
                    resource_id: Some(ResourceId::TypeId(TypeId::of::<T>())),
                    resource_access: Some(ResourceAccess::Unique),
                    ..Default::default()
                }.build().unwrap()
            ])
        }

        event!(FUNCTION_LEVEL, "Using access builder at index 0");

        let mut access_builder = prompted_accesses.remove(0);
        access_builder.resource_access.replace(ResourceAccess::Unique);
        if access_builder.resource_id.is_none() {
            access_builder.resource_id.replace(ResourceId::TypeId(TypeId::of::<T>()));
        }

        Ok(vec![access_builder.build().unwrap()])
    }

    fn resolve_access<'new>(_entity: Option<Entity>, _program_registry: Arc<ProgramRegistry>, mut derived_results: Vec<Result<ResolvedResource<'new>, DerivedError>>) -> Result<Self::Item<'new>, ResolveResourceError> {
        trace_function!("Unique Resolve Access");

        if derived_results.len() < 1 {
            return Err(ResolveResourceError::ExpectedResults { expected: 1, found: 0 })
        }

        event!(FUNCTION_LEVEL, "Using derived result at index 0");

        let resolved_resource = derived_results.remove(0)?;

        let resource = resolved_resource.cast::<T>()?;

        Ok(Unique { resource })
    }
}
