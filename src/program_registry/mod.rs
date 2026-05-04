use std::{collections::HashMap, sync::Arc};

use aion_state::prelude::{Registry, RegistryAcquireAccess, RegistryAcquireAccessResult, RegistryReleaseAccess, RegistryReleaseAccessResult};

use crate::prelude::{AccessResult, AccessStorage, BlacklistStorage, ControlStorage, CredentialStorage, DerivedResult, FinalisedAccess, Injection, ProgramAccess, ProgramId, ProgramReleaseAccess, PromptedProgramAccess, RegistryStorage, ReservationStorage, ResolveResourceError, ResolvedResource, StoredProgram, WhitelistStorage};

pub mod prompted_program_access;

pub mod program_id;
pub mod stored_program;
pub mod program_access;
pub mod resolved_resource;
pub mod derived_result;
pub mod program_registry_input;

pub struct ProgramRegistry {
    global_program_id: ProgramId,
    programs: Registry<
        RegistryStorage<ProgramId, StoredProgram>,
        ReservationStorage<ProgramId, ProgramAccess>,
        AccessStorage<ProgramId, ProgramAccess>,
        CredentialStorage,
        WhitelistStorage<ProgramId, ProgramAccess>,
        BlacklistStorage<ProgramId, ProgramAccess>,
        ControlStorage<ProgramId>
    >
}

unsafe impl Send for ProgramRegistry {}
unsafe impl Sync for ProgramRegistry {}

impl ProgramRegistry {
    pub fn resolve<'a, T: Injection>(self: &'a Arc<Self>, prompted_program_accesses: Vec<PromptedProgramAccess<'a>>) -> Result<<T as Injection>::Item<'a>, ResolveResourceError> {
        let access_builders = prompted_program_accesses.into_iter().map(|prompted_accesses| prompted_accesses.with(&self.global_program_id, false)).collect();

        let submitted_accesses = T::submit_access(access_builders);

        let mut resolved_resources = HashMap::new();

        let derived_results = submitted_accesses.into_iter().map(|FinalisedAccess { 
            program_id, 
            program_password,
            user_details, 
            resource_id, 
            resource_password, 
            resource_access 
        }| {
            // let user_details = user_details.as_ref().map(|(user_id, user_password)| (user_id, user_password));

            let program_access = ProgramAccess::Shared(1);

            let program_access_result = self.programs.acquire_access(RegistryAcquireAccess {
                user_details,
                resource_id: program_id.clone(),
                access: program_access.clone(),
                password: program_password,
            });  

            if let RegistryAcquireAccessResult::Found(access_result) = program_access_result {
                let AccessResult::Shared(program) = access_result else { unreachable!() };
                
                let resource_access_result = program.acquire_access(RegistryAcquireAccess {
                    user_details,
                    resource_id: resource_id.clone(),
                    access: resource_access.clone(),
                    password: resource_password.as_ref()
                });
                
                if let RegistryAcquireAccessResult::Found(access_result) = resource_access_result {
                    resolved_resources.entry(program_id.clone())
                        .or_insert(Vec::default())
                        .push((program, resource_access.clone(), resource_id.clone()));

                    DerivedResult::Complete(ResolvedResource::new(
                        access_result,
                        Arc::clone(self),
                        Arc::clone(program),
                        program_id,
                        resource_access,
                        resource_id,
                    ))
                } else {
                    assert!(
                        matches!(
                            // Safety
                            // We do not use program any further
                            // and we do not store it
                            unsafe { 
                                self.programs.release_access(&RegistryReleaseAccess {
                                    resource_id: &program_id,
                                    access: &program_access
                                }) 
                            }, 
                            RegistryReleaseAccessResult::Ok
                        )
                    );
                    DerivedResult::ResourceAccessNotFound(resource_access_result)
                }
            } else {
                assert!(
                    matches!(
                        // Safety
                        // We do not use program any further
                        // and we do not store it
                        unsafe { 
                            self.programs.release_access(&RegistryReleaseAccess {
                                resource_id: &program_id,
                                access: &program_access
                            }) 
                        }, 
                        RegistryReleaseAccessResult::Ok
                    )
                );
                DerivedResult::ProgramAccessNotFound(program_access_result)
            }
        }).collect::<Vec<_>>();

        let resolve_result = T::resolve_access(derived_results);
        match resolve_result {
            Ok(item) => Ok(item),
            Err(err) => {
                for (program_id, accesses) in resolved_resources {
                    for (program, resource_access, resource_id) in accesses {
                        assert!(
                            matches!(
                                // Safety
                                // We do not use the resources any further (in the drop)
                                unsafe {
                                    program.release_access(&RegistryReleaseAccess {
                                        resource_id: &resource_id,
                                        access: &resource_access
                                    })
                                },
                                RegistryReleaseAccessResult::Ok
                            )
                        );
                    }
    
                    assert!(
                        matches!(
                            // Safety
                            // We do not use program any further
                            // and we do not store it
                            unsafe {
                                self.release_access(&ProgramReleaseAccess { program_id: &program_id })
                            },
                            RegistryReleaseAccessResult::Ok
                        )
                    );
                }
                Err(err)
            },
        }
    }

    /// # Safety
    /// 
    /// Ensure what is being released is actually released
    pub(crate) unsafe fn release_access(
        &self,
        ProgramReleaseAccess {
            program_id,
        }: &ProgramReleaseAccess
    ) -> RegistryReleaseAccessResult {
        unsafe { self.programs.release_access(&RegistryReleaseAccess {
            resource_id: *program_id,
            access: &ProgramAccess::Shared(1)
        }) }
    }
}