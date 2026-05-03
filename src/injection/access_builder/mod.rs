use crate::prelude::{Access, FinalisedAccess, ProgramId, ResourceId, ResourcePassword, UserId, UserPassword};

pub struct AccessBuilder {
    pub program_id: ProgramId,
    pub global_program_id: ProgramId,

    pub user_details: Option<(UserId, UserPassword)>,

    pub resource_id: ResourceId,
    pub resource_password: Option<ResourcePassword>,
    pub access: Access,

    pub use_global_program_id: bool,
}

impl AccessBuilder {
    pub fn build(self) -> FinalisedAccess {
        let program_id = match self.use_global_program_id {
            true => self.global_program_id,
            false => self.program_id,
        };

        FinalisedAccess {
            program_id, 
            user_details: self.user_details, 
            resource_id: self.resource_id, 
            resource_password: self.resource_password, 
            access: self.access
        }
    }
}