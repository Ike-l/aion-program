use crate::prelude::{Access, FinalisedAccess, ProgramId, ResourceId, ResourcePassword, UserId, UserPassword};

pub struct AccessBuilder {
    program_id: ProgramId,
    global_program_id: ProgramId,

    user_details: Option<(UserId, UserPassword)>,

    resource_id: ResourceId,
    resource_password: Option<ResourcePassword>,
    access: Access,

    use_global_program_id: bool,
}

impl AccessBuilder {
    pub fn build(self) -> FinalisedAccess {
        let program_id = match self.use_global_program_id {
            true => self.global_program_id,
            false => self.program_id,
        };

        FinalisedAccess::new(
            program_id, 
            self.user_details, 
            self.resource_id, 
            self.resource_password, 
            self.access
        )
    }
}