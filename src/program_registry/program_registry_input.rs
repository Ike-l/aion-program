use crate::prelude::{ProgramId, ResourceAccess, ResourceId};

pub struct ProgramReleaseAccess<'a> {
    pub program_id: &'a ProgramId,
    pub resource_id: &'a ResourceId,
    pub resource_access: &'a ResourceAccess,
}