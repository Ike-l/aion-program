use crate::prelude::{ProgramId, ResourceAccess, ResourceId, UserId, UserPassword, ValuePassword};

pub struct FinalisedAccess<'a> {
    pub program_id: &'a ProgramId,
    pub program_password: Option<&'a ValuePassword>,

    pub user_details: Option<(UserId, UserPassword)>,

    pub resource_id: ResourceId,
    pub resource_password: Option<ValuePassword>,
    pub resource_access: ResourceAccess,
}