use crate::prelude::{FinalisedAccess, ProgramId, ResourceAccess, ResourceId, UserId, UserPassword, ValuePassword, OwnedAccessBuilder};

pub mod owned_access_builder;

#[derive(Clone)]
pub struct AccessBuilder<'a> {
    pub program_id: Option<&'a ProgramId>,

    pub program_password: Option<&'a ValuePassword>,

    pub user_details: Option<(&'a UserId, &'a UserPassword)>,
    pub resource_id: Option<ResourceId>, 
    pub resource_access: Option<ResourceAccess>, 
    pub resource_password: Option<&'a ValuePassword>,   
}

impl<'a> AccessBuilder<'a> {
    /// None IFF: 
    /// 
    /// * No `ResourceId`
    /// 
    /// * No `ResourceAccess`
    /// 
    /// If `ProgramId` is None then will use the global program id
    pub fn build(self) -> Option<FinalisedAccess<'a>> {
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

impl<'a> From<&'a OwnedAccessBuilder> for AccessBuilder<'a> {
    fn from(value: &'a OwnedAccessBuilder) -> Self {
        let user_details = value.user_details.as_ref().map(|(user_id, user_password)| (user_id, user_password));
        Self {
            program_id: value.program_id.as_ref(),
            program_password: value.program_password.as_ref(),
            user_details,
            resource_id: value.resource_id.clone(),
            resource_access: value.resource_access.clone(),
            resource_password: value.resource_password.as_ref(),
        }
    }
}