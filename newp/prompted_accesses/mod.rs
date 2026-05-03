use crate::prelude::{Access, AccessBuilder, ProgramId, ResourceId, ValuePassword, UserId, UserPassword};

pub struct PromptedAccesses {
    pub program_id: ProgramId,

    pub user_details: Option<(UserId, UserPassword)>,

    pub resource_id: ResourceId,
    pub resource_password: Option<ValuePassword>,
    pub access: Access,
}

impl PromptedAccesses {
    pub fn with(self, global_program_id: ProgramId, use_global_program_id: bool) -> AccessBuilder {
        AccessBuilder {
            program_id: self.program_id,
            global_program_id,

            user_details: self.user_details,
            resource_id: self.resource_id,
            resource_password: self.resource_password,
            access: self.access,

            use_global_program_id,
        }
    }
}