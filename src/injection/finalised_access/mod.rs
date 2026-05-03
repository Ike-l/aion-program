use crate::prelude::{Access, ProgramId, ResourceId, ResourcePassword, UserId, UserPassword};

pub struct FinalisedAccess {
    program_id: ProgramId,

    user_details: Option<(UserId, UserPassword)>,

    resource_id: ResourceId,
    resource_password: Option<ResourcePassword>,
    access: Access,
}

impl FinalisedAccess {
    pub fn new(
        program_id: ProgramId,
        user_details: Option<(UserId, UserPassword)>,
        resource_id: ResourceId,
        resource_password: Option<ResourcePassword>,
        access: Access
    ) -> Self {
        Self { program_id, user_details, resource_id, resource_password, access }
    }
}