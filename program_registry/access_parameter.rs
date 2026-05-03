use crate::prelude::{ProgramId, ProgramKeyId, ResourceId, ResourceKeyId};

// move to aion-state?
pub struct AccessParameter<'a> {
    pub program_id: Option<ProgramId>, 
    pub program_key: Option<&'a ProgramKeyId>,

    pub resource_id: Option<ResourceId>,
    pub resource_key: Option<&'a ResourceKeyId>,
}
