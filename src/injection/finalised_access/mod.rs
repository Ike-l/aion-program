use crate::prelude::{ProgramId, ResourceAccess, ResourceId, UserId, UserPassword, ValuePassword};

pub struct FinalisedAccess {
    pub program_id: Option<ProgramId>,
    pub program_password: Option<ValuePassword>,

    pub user_details: Option<(UserId, UserPassword)>,

    pub resource_id: ResourceId,
    pub resource_access: ResourceAccess,
    pub resource_password: Option<ValuePassword>,
}