use std::{collections::HashMap, sync::Arc};

use aion_state::prelude::{RegistrySaferReplacementResult, RegistrySaferReplacement, Registry, RegistryAcquireAccess, RegistryAcquireAccessResult, RegistryReleaseAccess, RegistryReleaseAccessResult};

use crate::prelude::{ProgramRegistryResolveWithInsert, ResourceAccess, StoredResource, ProgramRegistryReplaceResourceError, ProgramRegistryReplaceResource, AccessBuilder, AccessResult, AccessStorage, AccessSubmissionError, BlacklistStorage, ControlStorage, CredentialStorage, DerivedResult, FinalisedAccess, Injection, ProgramAccess, ProgramId, ProgramRegistryReleaseAccess, RegistryStorage, ReservationStorage, ResolveResourceError, ResolvedResource, StoredProgram, WhitelistStorage};

pub mod program_id;
pub mod stored_program;
pub mod program_access;
pub mod resolved_resource;
pub mod derived_result;
pub mod program_registry_input;
pub mod program_registry_result;

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

impl ProgramRegistry {
    pub fn resolve<'a, T: Injection>(
        self: &'a Arc<Self>, 
        access_builders: Vec<AccessBuilder<'a>>
    ) -> Result<Result<<T as Injection>::Item<'a>, ResolveResourceError>, AccessSubmissionError> {
        let submitted_accesses = T::submit_access(access_builders)?;

        let mut resolved_resources = HashMap::new();

        let derived_results = submitted_accesses.into_iter().map(|FinalisedAccess { 
            program_id, 
            program_password,
            user_details, 
            resource_id, 
            resource_password, 
            resource_access 
        }| {
            let program_id = match program_id {
                Some(program_id) => program_id,
                None => &self.global_program_id,
            };

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
                    password: resource_password
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
        Ok(match resolve_result {
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
                                self.release_access(&ProgramRegistryReleaseAccess { program_id: &program_id })
                            },
                            RegistryReleaseAccessResult::Ok
                        )
                    );
                }
                Err(err)
            },
        })
    }

    /// # Safety
    /// 
    /// Ensure what is being released is actually released
    pub(crate) unsafe fn release_access(
        &self,
        ProgramRegistryReleaseAccess {
            program_id,
        }: &ProgramRegistryReleaseAccess
    ) -> RegistryReleaseAccessResult {
        unsafe { self.programs.release_access(&RegistryReleaseAccess {
            resource_id: *program_id,
            access: &ProgramAccess::Shared(1)
        }) }
    }

    pub fn replace_resource<'a>(
        &self,
        ProgramRegistryReplaceResource {
            user_details,
            program_id,
            program_password,

            resource,
            access,
            resource_id,
            resource_password,
        }: ProgramRegistryReplaceResource<'a>
    ) -> Result<RegistrySaferReplacementResult<StoredResource>, ProgramRegistryReplaceResourceError> {
        let program_id = match program_id {
            Some(program_id) => program_id,
            None => self.global_program_id.clone(),
        };

        match self.programs.acquire_access(RegistryAcquireAccess {
            user_details,
            resource_id: program_id,
            access: ProgramAccess::Shared(1),
            password: program_password
        }) {
            RegistryAcquireAccessResult::Found(access_result) => {
                let program = access_result.as_ref().unwrap();
                Ok(program.safer_replace(
                    RegistrySaferReplacement {
                        user_details,
                        access,
                        resource_id,
                        resource,
                        password: resource_password,
                    }
                ))
            },
            RegistryAcquireAccessResult::NotFound => Err(ProgramRegistryReplaceResourceError::NotFound),
            RegistryAcquireAccessResult::AccessConflict => Err(ProgramRegistryReplaceResourceError::AccessConflict),
            RegistryAcquireAccessResult::ReservationConflict => Err(ProgramRegistryReplaceResourceError::ReservationConflict),
            RegistryAcquireAccessResult::VerificationFailure => Err(ProgramRegistryReplaceResourceError::VerificationFailure),
            RegistryAcquireAccessResult::OwnershipDenied => Err(ProgramRegistryReplaceResourceError::OwnershipDenied),
            RegistryAcquireAccessResult::WhitelistDenied => Err(ProgramRegistryReplaceResourceError::WhitelistDenied),
            RegistryAcquireAccessResult::BlacklistDenied => Err(ProgramRegistryReplaceResourceError::BlacklistDenied),
        }
    }

    /// # Returns
    /// `None` if needs to replace with None `ResourceId`
    /// 
    /// `Some/Err` if `access_builders` is malformed
    /// 
    /// `Some/Ok/Err` if the `Injection` fails. Cannot be ResolveResourceError::Resolving
    /// 
    /// `Some/Ok/Ok/Err` for replacement error
    /// 
    /// `Some/Ok/Ok/Ok` for successfully resolved 
    pub fn resolve_with_insert<'a, T: Injection>(
        self: &'a Arc<Self>,
        access_builders: Vec<AccessBuilder<'a>>,
        ProgramRegistryResolveWithInsert {
            user_details,
            program_id,
            program_password,
            resource,
            resource_id,
            resource_password,
        }: ProgramRegistryResolveWithInsert
    ) -> Option<Result<Result<Result<T::Item<'a>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError>> {
        match self.resolve::<T>(access_builders.iter().cloned().collect()) {
            Ok(Ok(result)) => {
                Some(Ok(Ok(Ok(result))))
            },
            Ok(Err(resolve_resource_error)) => {
                if resolve_resource_error == ResolveResourceError::Resolving {
                    let resource_id = resource_id?;

                    let replace_result = self.replace_resource(ProgramRegistryReplaceResource { 
                        user_details, 
                        program_id, 
                        program_password, 
                        resource, 
                        access: &ResourceAccess::Replace, 
                        resource_id, 
                        resource_password
                    });

                    match replace_result {
                        Ok(_) => {
                            match self.resolve::<T>(access_builders) {
                                Ok(Ok(result)) => Some(Ok(Ok(Ok(result)))),
                                Ok(Err(_)) => unreachable!("Replacing makes this unreachable"),
                                Err(access_submission_error) => Some(Err(access_submission_error)),
                            }
                        },
                        Err(replace_error) => Some(Ok(Ok(Err(replace_error)))),
                    }
                } else {
                    Some(Ok(Err(resolve_resource_error)))
                }
            },
            Err(access_submission_error) => Some(Err(access_submission_error)),
        }
    }
}