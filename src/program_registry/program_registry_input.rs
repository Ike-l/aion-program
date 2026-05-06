use crate::prelude::{ProgramId, UserId, UserPassword, ResourceId, ValuePassword, Resource, ResourceAccess};

pub struct ProgramRegistryReleaseAccess<'a> {
    pub program_id: &'a ProgramId,
}

pub struct ProgramRegistryReplaceResource<'a> {
    pub user_details: Option<(&'a UserId, &'a UserPassword)>,
    pub program_id: Option<ProgramId>,
    pub program_password: Option<&'a ValuePassword>,

    pub resource: Option<Resource>,
    pub access: &'a ResourceAccess,
    pub resource_id: ResourceId,
    pub resource_password: Option<&'a ValuePassword>
}

#[derive(Default)]
pub struct ProgramRegistryResolveWithInsert<'a> {
    pub user_details: Option<(&'a UserId, &'a UserPassword)>,
    pub program_id: Option<ProgramId>,
    pub program_password: Option<&'a ValuePassword>,

    pub resource: Option<Resource>,
    pub resource_id: Option<ResourceId>,
    pub resource_password: Option<&'a ValuePassword>
}