use crate::prelude::{FinalisedAccess, ProgramId, ResourceAccess, ResourceId, UserId, UserPassword, ValuePassword};

#[derive(Debug, Clone, Default)]
pub struct AccessBuilder {
    pub program_id: Option<ProgramId>,

    pub program_password: Option<ValuePassword>,

    pub user_details: Option<(UserId, UserPassword)>,
    pub resource_id: Option<ResourceId>, 
    pub resource_access: Option<ResourceAccess>, 
    pub resource_password: Option<ValuePassword>,   
}

impl AccessBuilder {
    /// None IFF: 
    /// 
    /// * No `ResourceId`
    /// 
    /// * No `ResourceAccess`
    /// 
    /// If `ProgramId` is None then will use the global program id
    pub fn build(self) -> Option<FinalisedAccess> {
        let Some(resource_id) = self.resource_id else { return None };
        let Some(resource_access) = self.resource_access else { return None };

        Some(FinalisedAccess {
            program_id: self.program_id, 
            program_password: self.program_password,
            user_details: self.user_details, 
            resource_id,
            resource_password: self.resource_password, 
            resource_access
        })
    }
}

