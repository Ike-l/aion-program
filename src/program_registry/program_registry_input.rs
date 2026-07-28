use crate::prelude::{ProgramId, Resource, Access, ResourceId, UserId, UserPassword, ValuePassword};

pub struct ProgramRegistryReleaseProgram<'a> {
    pub program_id: &'a ProgramId,
}

pub struct ProgramRegistryReleaseResource<'a> {
    pub program_id: &'a ProgramId,
    pub resource_id: &'a ResourceId,
}

pub struct ProgramRegistryReplaceResource<'a> {
    pub user_details: Option<(UserId, UserPassword)>,
    pub program_id: Option<ProgramId>,
    pub program_password: Option<ValuePassword>,

    pub resource: Option<Resource>,
    pub access: &'a Access,
    pub resource_id: ResourceId,
    pub resource_password: Option<&'a ValuePassword>
}

#[derive(Default)]
pub struct ProgramRegistryResolveWithInsert<'a> {
    pub user_details: Option<(UserId, UserPassword)>,
    pub program_id: Option<ProgramId>,
    pub program_password: Option<ValuePassword>,

    pub resource: Option<Box<dyn FnOnce() -> Resource + Send>>,
    pub resource_id: Option<ResourceId>,
    pub resource_password: Option<&'a ValuePassword>
}

pub struct ProgramRegistryAcquireProgram {
    pub user_details: Option<(UserId, UserPassword)>,
    pub program_id: ProgramId,
    pub program_password: Option<ValuePassword>
}
