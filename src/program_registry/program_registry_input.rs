use crate::prelude::ProgramId;

pub struct ProgramReleaseAccess<'a> {
    pub program_id: &'a ProgramId,
}