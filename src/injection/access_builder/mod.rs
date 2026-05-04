use crate::prelude::{FinalisedAccess, ProgramId, ResourceAccess, ResourceId, UserId, UserPassword, ValuePassword};

pub struct AccessBuilder<'a> {
    pub program_id: &'a ProgramId,
    pub program_password: Option<ValuePassword>,

    pub global_program_id: &'a ProgramId,

    pub user_details: Option<(UserId, UserPassword)>,

    pub resource_id: ResourceId,
    pub resource_password: Option<ValuePassword>,
    pub resource_access: ResourceAccess,

    pub use_global_program_id: bool,
}

impl<'a> AccessBuilder<'a> {
    pub fn build(self) -> FinalisedAccess<'a> {
        let program_id = match self.use_global_program_id {
            true => self.global_program_id,
            false => self.program_id,
        };

        FinalisedAccess {
            program_id, 
            program_password: self.program_password,
            user_details: self.user_details, 
            resource_id: self.resource_id, 
            resource_password: self.resource_password, 
            resource_access: self.resource_access
        }
    }
}