use std::{any::TypeId, ops::Deref, sync::Arc};

use hecs::Entity;
use tracing::event;

use crate::prelude::{AccessBuilder, AccessSubmissionError, CastedResource, DerivedError, FUNCTION_LEVEL, FinalisedAccess, Injection, ProgramRegistry, ResolveResourceError, ResolvedResource, ResourceAccess, ResourceId, trace_function};

pub struct Shared<'a, T> {
    resource: CastedResource<'a, T>
}

impl<'a, T> Shared<'a, T> {
    pub fn as_ref(&self) -> &T {
        self.resource.as_ref().expect("Expected Shared Resource")
    }
}

impl<'a, T> Deref for Shared<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()    
    }
}

impl<'a, T: 'static> Injection for Shared<'a, T> {
    type Item<'new> = Shared<'new, T>;

    fn claim_manual_access_builders(_access_builders: Vec<&AccessBuilder>) -> Vec<usize> { 
        trace_function!("Shared Claim Manual Access Builders");

        event!(FUNCTION_LEVEL, "Claiming no access builders");

        vec![] 
    }

    fn submit_access(mut prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
        trace_function!("Shared Submit Access");

        if prompted_accesses.len() == 0 {
            event!(FUNCTION_LEVEL, "Building default submission");

            return Ok(vec![
                AccessBuilder {
                    resource_id: Some(ResourceId::TypeId(TypeId::of::<T>())),
                    resource_access: Some(ResourceAccess::Shared(1)),
                    ..Default::default()
                }.build().unwrap()
            ])
        }

        event!(FUNCTION_LEVEL, "Using access builder at index 0");

        let mut access_builder = prompted_accesses.remove(0);
        access_builder.resource_access.replace(ResourceAccess::Shared(1));
        if access_builder.resource_id.is_none() {
            access_builder.resource_id.replace(ResourceId::TypeId(TypeId::of::<T>()));
        }

        Ok(vec![access_builder.build().unwrap()])
    }

    fn resolve_access<'new>(_entity: Option<Entity>, _program_registry: Arc<ProgramRegistry>, mut derived_results: Vec<Result<ResolvedResource<'new>, DerivedError>>) -> Result<Self::Item<'new>, ResolveResourceError> {
        trace_function!("Shared Resolve Access");

        if derived_results.len() < 1 {
            return Err(ResolveResourceError::ExpectedResults { expected: 1, found: 0 })
        }

        event!(FUNCTION_LEVEL, "Using derived result at index 0");
        
        let resolved_resource = derived_results.remove(0)?;

        let resource = resolved_resource.cast::<T>()?;

        Ok(Shared { resource })
    }
}
