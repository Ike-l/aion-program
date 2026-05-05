use crate::prelude::{ProgramId, ValuePassword, UserId, UserPassword, ResourceId, ResourceAccess, AccessBuilder};

#[derive(Clone)]
pub struct OwnedAccessBuilder {
    pub program_id: Option<ProgramId>,

    pub program_password: Option<ValuePassword>,

    pub user_details: Option<(UserId, UserPassword)>,
    pub resource_id: Option<ResourceId>, 
    pub resource_access: Option<ResourceAccess>, 
    pub resource_password: Option<ValuePassword>,   
}

impl From<AccessBuilder<'_>> for OwnedAccessBuilder {
    fn from(value: AccessBuilder) -> Self {
        let user_details = value.user_details.map(|(user_id, user_password)| { (user_id.clone(), user_password.clone()) });

        OwnedAccessBuilder {
            program_id: value.program_id.cloned(), 
            program_password: value.program_password.cloned(), 
            user_details, 
            resource_id: value.resource_id,
            resource_access: value.resource_access, 
            resource_password: value.resource_password.cloned()
        }
    }
}