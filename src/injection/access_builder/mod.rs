use crate::prelude::{FinalisedAccess, ProgramId, ResourceAccess, ResourceId, UserId, UserPassword, ValuePassword};

pub struct AccessBuilder<'a> {
    pub program_id: &'a ProgramId,
    pub program_password: Option<&'a ValuePassword>,

    pub global_program_id: &'a ProgramId,
    pub use_global_program_id: bool,

    pub user_details: Option<(&'a UserId, &'a UserPassword)>,
    pub resource_id: Option<ResourceId>, 
    pub resource_access: Option<ResourceAccess>, 
    pub resource_password: Option<&'a ValuePassword>,   
}

impl<'a> AccessBuilder<'a> {
    /// None IFF: 
    /// 
    /// * No ResourceId
    /// 
    /// * No Resource Access
    pub fn build(self) -> Option<FinalisedAccess<'a>> {
        let program_id = match self.use_global_program_id {
            true => self.global_program_id,
            false => self.program_id,
        };

        let Some(resource_id) = self.resource_id else { return None };
        let Some(resource_access) = self.resource_access else { return None };

        Some(FinalisedAccess {
            program_id, 
            program_password: self.program_password,
            user_details: self.user_details, 
            resource_id,
            resource_password: self.resource_password, 
            resource_access
        })
    }
}