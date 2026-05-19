use crate::prelude::{Program, ProgramId, Resource, ResourceAccess, ResourceId, UserId, UserPassword, ValuePassword};

pub struct ProgramRegistryReleaseProgram<'a> {
    pub program_id: &'a ProgramId,
}

pub struct ProgramRegistryReleaseResource<'a> {
    pub program: &'a Program,
    pub program_id: &'a ProgramId,
    pub resource_id: &'a ResourceId,
    pub resource_access: &'a ResourceAccess
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

    pub resource: Option<Box<dyn FnOnce() -> Resource + Send>>,
    pub resource_id: Option<ResourceId>,
    pub resource_password: Option<&'a ValuePassword>
}

pub struct ProgramRegistryAcquireProgram<'a> {
    pub user_details: Option<(&'a UserId, &'a UserPassword)>,
    pub program_id: ProgramId,
    pub program_password: Option<&'a ValuePassword>
}
