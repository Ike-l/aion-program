use std::{collections::HashSet, sync::Arc};

use aion_state::prelude::{RegistrySaferReplacementResult, RegistrySaferReplacement, Registry, RegistryAcquireAccess, RegistryAcquireAccessResult, RegistryReleaseAccess, RegistryReleaseAccessResult};

use crate::prelude::{AccessBuilder, AccessResult, AccessStorage, AccessSubmissionError, BlacklistStorage, ControlStorage, CredentialStorage, DerivedResult, FinalisedAccess, Injection, ProgramAccess, ProgramId, ProgramRegistryAcquireProgram, ProgramRegistryReleaseProgram, ProgramRegistryReplaceResource, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, RegistryStorage, ReservationStorage, ResolveResourceError, ResolvedResource, ResourceAccess, StoredProgram, StoredResource, WhitelistStorage};

pub mod program_id;
pub mod stored_program;
pub mod program_access;
pub mod resolved_resource;
pub mod derived_result;
pub mod program_registry_input;
pub mod program_registry_result;

pub struct ProgramRegistry {
    program_ids: HashSet<ProgramId>,
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
        access_builders: Vec<AccessBuilder>
    ) -> Result<Result<<T as Injection>::Item<'a>, ResolveResourceError>, AccessSubmissionError> {
        let submitted_accesses = T::submit_access(access_builders)?;

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
                None => self.global_program_id.clone(),
            };

            let user_details = user_details.as_ref().map(|(u, p)| (u, p));

            let program_access_result = self.acquire_program(ProgramRegistryAcquireProgram { 
                user_details, 
                program_id: program_id.clone(), 
                program_password: program_password.as_ref()
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
                    DerivedResult::Complete(ResolvedResource::new(
                        access_result,
                        Arc::clone(self),
                        Arc::clone(program),
                        program_id,
                        resource_access,
                        resource_id,
                        user_details.map(|(user_id, user_password)| (user_id.clone(), user_password.clone())),
                    ))
                } else {
                    assert!(
                        matches!(
                            // Safety
                            // We do not use program any further
                            // and we do not store it
                            unsafe {
                                self.release_program(&ProgramRegistryReleaseProgram {
                                    program_id: &program_id,
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
                            self.release_program(&ProgramRegistryReleaseProgram {
                                program_id: &program_id,
                            })
                        },
                        RegistryReleaseAccessResult::Ok
                    )
                );

                DerivedResult::ProgramAccessNotFound(program_access_result)
            }
        }).collect::<Vec<_>>();

        let resolve_result = T::resolve_access(derived_results);
        Ok(resolve_result)
    }

    // dont need
    // pub fn check_resolve<'a, T: Injection>(
    //     self: &'a Arc<Self>,
    //     access_builders: Vec<AccessBuilder>
    // ) -> bool {
    //     // mayber a better way of doing this later?
    //     // it literally just gets the access and then drops it immediately
    //     // could mess up future "counters" or logging
    //     // fuck logging bruh
    //     // oh wait no its fine cause i can make a span saying its just checking ;)
    //     self.resolve::<T>(access_builders).is_ok_and(|result| result.is_ok())
    // }

    /// # Safety
    /// 
    /// Ensure what is being released is actually released
    pub(crate) unsafe fn release_program(
        &self,
        ProgramRegistryReleaseProgram {
            program_id,
        }: &ProgramRegistryReleaseProgram
    ) -> RegistryReleaseAccessResult {
        unsafe { self.programs.release_access(&RegistryReleaseAccess {
            resource_id: *program_id,
            access: &ProgramAccess::Shared(1)
        }) }
    }

    fn acquire_program(
        &self,
        ProgramRegistryAcquireProgram {
            user_details,
            program_id,
            program_password
        }: ProgramRegistryAcquireProgram
    ) -> RegistryAcquireAccessResult<AccessResult<'_, StoredProgram>>{
        self.programs.acquire_access(RegistryAcquireAccess {
            user_details: user_details,
            resource_id: program_id,
            access: ProgramAccess::Shared(1),
            password: program_password,
        })
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
    /// `None` if needs to replace with `ResourceId` OR `Resource` as None
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
        access_builders: Vec<AccessBuilder>,
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
                    let resource = resource?();

                    let replace_result = self.replace_resource(ProgramRegistryReplaceResource { 
                        user_details, 
                        program_id, 
                        program_password, 
                        resource: Some(resource), 
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

    pub fn program_ids(&self) -> impl Iterator<Item = &ProgramId> {
        // ensure program_ids is full of all program_ids AND the global program id
        self.program_ids.iter()
    }
}