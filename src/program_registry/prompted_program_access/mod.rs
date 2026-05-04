use crate::prelude::{AccessBuilder, ProgramId, ResourceAccess, ResourceId, UserId, UserPassword, ValuePassword};

pub struct PromptedProgramAccess<'a> {
    pub program_id: &'a ProgramId,
    pub program_password: Option<ValuePassword>,

    pub user_details: Option<(UserId, UserPassword)>,

    

    pub resource_id: ResourceId,
    pub resource_password: Option<ValuePassword>,
    pub resource_access: ResourceAccess,
}

impl<'a> PromptedProgramAccess<'a> {
    pub fn with(self, global_program_id: &'a ProgramId, use_global_program_id: bool) -> AccessBuilder<'a> {
        AccessBuilder {
            program_id: self.program_id,
            program_password: self.program_password,

            global_program_id,

            user_details: self.user_details,
            resource_id: self.resource_id,
            resource_password: self.resource_password,
            resource_access: self.resource_access,

            use_global_program_id,
        }
    }
}