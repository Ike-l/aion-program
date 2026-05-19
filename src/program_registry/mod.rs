use std::{collections::{HashMap, HashSet}, iter::once, sync::Arc, task::Waker};

use aion_state::prelude::{RegistrySaferReplacementResult, RegistrySaferReplacement, Registry, RegistryAcquireAccess, RegistryAcquireAccessResult, RegistryReleaseAccess, RegistryReleaseAccessResult};
use hecs::Entity;
use parking_lot::lock_api::Mutex;

use crate::prelude::{AccessBuilder, AccessResult, AccessStorage, AccessSubmissionError, BlacklistStorage, ControlStorage, CredentialStorage, FinalisedAccess, FutureResolve, Injection, ProgramAccess, ProgramId, ProgramRegistryAcquireProgram, ProgramRegistryReleaseProgram, ProgramRegistryReplaceResource, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, RegistryStorage, ReservationStorage, ResolveResourceError, ResourceAccess, ResourceId, StoredProgram, StoredResource, WhitelistStorage};

pub mod program_id;
pub mod stored_program;
pub mod program_access;
pub mod resolved_resource;
pub mod derived_result;
pub mod program_registry_input;
pub mod program_registry_result;

pub mod future_resolve;

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
    >,

    future_resources: Mutex<parking_lot::RawMutex, HashMap<(Option<ProgramId>, ResourceId), Vec<Arc<Mutex<parking_lot::RawMutex, (Option<Waker>, bool)>>>>>
}

impl ProgramRegistry {
    pub fn resolve<'a, T: Injection>(
        self: &'a Arc<Self>, 
        entity: Option<Entity>,
        access_builders: Vec<AccessBuilder>
    ) -> Result<Result<<T as Injection>::Item<'a>, ResolveResourceError>, AccessSubmissionError> {
        let submitted_accesses = T::submit_access(access_builders)?;

        let derived_results = submitted_accesses.into_iter().map(|finalised_access| finalised_access.derive(self)).collect::<Vec<_>>();

        let resolve_result = T::resolve_access(entity, Arc::clone(self), derived_results);
        Ok(resolve_result)
    }

    pub fn resolve_async<'a, T: Injection>(
        self: &'a Arc<Self>, 
        entity: Option<Entity>,
        access_builders: Vec<AccessBuilder>
    ) -> Result<Result<<T as Injection>::Item<'a>, FutureResolve<'a, T>>, AccessSubmissionError> {
        let submitted_accesses = T::submit_access(access_builders)?;

        let derived_results = submitted_accesses.iter().map(|finalised_access| finalised_access.derive(self)).collect::<Vec<_>>();

        let resolved_result = T::resolve_access(entity, Arc::clone(self), derived_results);

        Ok(resolved_result.map_err(|_| {
            let waker_ready = Arc::new(Mutex::new((None, false)));

            submitted_accesses
                .iter()
                .map(|submitted_access| 
                    (submitted_access.program_id.clone(), submitted_access.resource_id.clone()
                )).for_each(|resource_handle| {
                    self.future_resources.lock().entry(resource_handle).or_default().push(Arc::clone(&waker_ready));
                });

            FutureResolve::new(self, entity, submitted_accesses, waker_ready)
        }))
    }

    pub(crate) fn try_resolve<'a, T: Injection>(
        self: &'a Arc<Self>,
        entity: Option<Entity>,
        finalised_access_builders: Vec<FinalisedAccess>,
    ) -> Result<<T as Injection>::Item<'a>, ResolveResourceError> {
        let derived_results = finalised_access_builders.into_iter().map(|finalised_access| finalised_access.derive(self)).collect::<Vec<_>>();
        T::resolve_access(entity, Arc::clone(self), derived_results)
    }

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

    pub(crate) fn acquire_program(
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
        entity: Option<Entity>,
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
        match self.resolve::<T>(entity, access_builders.iter().cloned().collect()) {
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
                            match self.resolve::<T>(entity, access_builders) {
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
        self.program_ids.iter().chain(once(&self.global_program_id))
    }

    pub fn global_program_id(&self) -> &ProgramId {
        &self.global_program_id
    }
}