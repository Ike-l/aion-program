use std::sync::Arc;

use aion_state::prelude::{Registry, RegistryAccessResult};

pub mod program;
pub mod injected;
pub mod program_registry_results;
pub mod access_parameter;

use crate::prelude::{AccessParameter, AccessResult, Injected, ProgramAccess, ProgramId, ProgramKeyId, ProgramRegistryResolveResult, ProgramReserverId, ResolvedResource, ResourceAccess, ResourceId, ResourceKeyId, ResourceReserverId, StoredProgram};

pub struct ProgramRegistry {
    registry: Registry<ProgramId, ProgramReserverId, ProgramAccess, ProgramId, ProgramKeyId, Box<StoredProgram>>,
    global_program_id: Option<ProgramId>
}

impl ProgramRegistry {
    fn get_program(
        &self, 
        program_id: Option<&ProgramId>,
        program_reserver_id: Option<&ProgramReserverId>,
        program_key_id: Option<&ProgramKeyId>,
    ) -> Option<RegistryAccessResult<AccessResult<'_, StoredProgram>>> {
        let program_id = match (program_id, self.global_program_id.as_ref()) {
            (None, None) => return None,
            (None, Some(program_id)) => program_id.clone(),
            (Some(program_id), _) => program_id.clone(),
        };

        // deaccesses happen when?
        // can manually deaccess by checking if there are no internal accesses
        Some(self.registry.access(program_id, ProgramAccess::Shared(1), program_reserver_id, program_key_id))
    }

    pub fn contains_resources(
        &self,
        program_id: Option<&ProgramId>,
        program_key_id: Option<&ProgramKeyId>,
        program_reserver_id: Option<&ProgramReserverId>,
        resource_ids: &Vec<ResourceId>,
    ) -> Option<bool> {
        let program = self.get_program(program_id, program_reserver_id, program_key_id)?;
        
        match program {
            RegistryAccessResult::Found(AccessResult::Shared(stored_program)) => Some(stored_program.contains_resources(resource_ids)),
            _ => None
        }
    }

    pub fn permits_accesses(
        &self,
        program_id: Option<&ProgramId>,
        program_key_id: Option<&ProgramKeyId>,
        program_reserver_id: Option<&ProgramReserverId>,
        accesses: &Vec<(&ResourceId, &ResourceAccess, Option<&ResourceReserverId>, Option<&ResourceKeyId>)>,
    ) -> Option<bool> {
        let program = self.get_program(program_id, program_reserver_id, program_key_id)?;
        
        match program {
            RegistryAccessResult::Found(AccessResult::Shared(stored_program)) => Some(stored_program.permits_accesses(accesses)),
            _ => None
        }
    }

    // can give access to programs and increment Shared
    // cant deaccess 
    // (can automatically detect when i can deaccess by 
    // checking the registry itself for accesses)

    pub fn resolve<T: Injected>(
        &self,
        program_reserver_id: Option<&'_ ProgramReserverId>,
        resource_reserver_id: Option<&'_ ResourceReserverId>,
        access_parameters: Vec<AccessParameter<'_>>,
    ) -> ProgramRegistryResolveResult<'_, T> {

        let mut resource_ids = Vec::new();
        let mut keys = Vec::new();
        for access in access_parameters {
            resource_ids.push((access.program_id, access.resource_id));
            keys.push((access.program_key, access.resource_key));
        }

        let (
            accesses, 
            programs
        ) = T::accesses(resource_ids);

        if self.global_program_id.is_none() {
            if accesses.iter().any(|(program_id, _, _)| program_id.is_none()) {
                return ProgramRegistryResolveResult::ExpectsGlobalProgram
            }

            if programs.iter().any(|program_id| program_id.is_none()) {
                return ProgramRegistryResolveResult::ExpectsGlobalProgram
            }
        }

        if accesses
            .iter()
            .enumerate()
            .any(|(i, (program_id, _, _))| {
                let (program_key_id, _) = keys
                .get(i)
                .unwrap();

                let program = self.get_program(
                    program_id.as_ref(),
                    program_reserver_id, 
                    *program_key_id
                ).unwrap();

                !matches!(program, RegistryAccessResult::Found(_))
            }) { return ProgramRegistryResolveResult::AccessFailure; }

        let resolved_resources = accesses
            .into_iter()
            .enumerate()
            .map(|(i, (program_id, resource_id, access))| {
                let (program_key_id, resource_key_id) = keys
                    .get(i)
                    .unwrap();
    
                let program = self.get_program(
                    program_id.as_ref(),
                    program_reserver_id, 
                    *program_key_id
                ).unwrap();
    
                let RegistryAccessResult::Found(AccessResult::Shared(program)) = program else { unreachable!() };
    
                let result = program.access(
                    resource_id.clone(),
                    access.clone(),
                    resource_reserver_id,
                    *resource_key_id
                );

                if let RegistryAccessResult::Found(access_result) = result {
                    Some(ResolvedResource::new(
                        access_result, 
                        Arc::clone(&program), 
                        access, 
                        resource_id, 
                        resource_key_id.cloned()
                    ))
                } else {
                    None
                }
            }).collect::<Vec<_>>();

        let programs = programs.into_iter().map(|program_id| {
            let program_key_id;

            let program = self.get_program(
                program_id.as_ref(), 
                program_reserver_id, 
                program_key_id
            ).unwrap();

            let RegistryAccessResult::Found(AccessResult::Shared(stored_program)) = program else { unreachable!() };
            Arc::clone(&stored_program)
        }).collect::<Vec<_>>();

        let result = T::resolve(
            resolved_resources, 
            programs, 
        );

        if result.is_err() {
            todo!("Manually Drop Each Access");
            unreachable!()
        }

        ProgramRegistryResolveResult::Found(result)
    }
}

impl Default for ProgramRegistry {
    fn default() -> Self {
        let registry = Registry::default();

        // let global_program_id = ProgramId::new("__Global__");
        // registry.accessed_replacement(global_program_id.clone(), ProgramAccess::Replace, None, None, Some(StoredProgram::default()));

        Self {
            registry,
            global_program_id: None
            // global_program_id,
        }
    }
}