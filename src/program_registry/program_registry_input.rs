use crate::prelude::{ProgramId, UserId, UserPassword, ResourceId, ValuePassword, Resource, ResourceAccess};

pub struct ProgramRegistryReleaseAccess<'a> {
    pub program_id: &'a ProgramId,
}

pub struct ProgramReplaceResource<'a> {
    pub user_details: Option<(&'a UserId, &'a UserPassword)>,
    pub program_id: ProgramId,
    pub program_password: Option<&'a ValuePassword>,

    pub resource: Option<Resource>,
    pub access: &'a ResourceAccess,
    pub resource_id: ResourceId,
    pub resource_password: Option<&'a ValuePassword>
}