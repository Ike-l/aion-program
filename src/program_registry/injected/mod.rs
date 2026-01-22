use std::sync::Arc;

use crate::prelude::{Program, ProgramId, ResolvedResource, ResourceAccess, ResourceId};

// pub mod injected_primitives;
pub mod injection_error;

use injection_error::InjectionError;

pub trait Injected {
    type Item<'new>;

    // type ParameterId;
    // Can return HashMap<ParameterId, (ResourceId, ResourceAccess)>
    fn accesses(
        prompted_resource_ids: Vec<(
            Option<ProgramId>, 
            Option<ResourceId>
        )>
    ) -> (
        Vec<(Option<ProgramId>, ResourceId, ResourceAccess)>, 
        Vec<Option<ProgramId>>
    );

    // takes HashMap<ParameterId, ResolvedResource<'new>>
    fn resolve<'new>(
        resolved_resources: Vec<Option<ResolvedResource<'new>>>,
        programs: Vec<Arc<Program>>,
    ) -> Result<Self::Item<'new>, InjectionError>;
}