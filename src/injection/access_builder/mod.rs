pub struct AccessBuilder {
    program_id: ProgramId,
    global_program_id: ProgramId,

    user_details: Option<(UserId, UserPassword)>,

    resource_id: ResourceId,
    resource_password: Option<ResourcePassword>,
    access: Access,
}