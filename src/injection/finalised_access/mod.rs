use crate::prelude::{Access, ProgramId, ResourceId, ResourcePassword, UserId, UserPassword};

pub struct FinalisedAccess {
    pub program_id: ProgramId,

    pub user_details: Option<(UserId, UserPassword)>,

    pub resource_id: ResourceId,
    pub resource_password: Option<ResourcePassword>,
    pub access: Access,
}