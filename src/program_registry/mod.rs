use std::{collections::{HashMap, HashSet}, iter::once, sync::Arc, task::Waker};

use aion_state::prelude::{RegistrySaferReplacementResult, RegistrySaferReplacement, Registry, RegistryAcquireAccess, RegistryAcquireAccessResult, RegistryReleaseAccess, RegistryReleaseAccessResult};
use hecs::Entity;
use parking_lot::{RawMutex, lock_api::Mutex};
use tokio::runtime::Runtime;
use tracing::{Level, event, span};

use crate::prelude::{AccessBuilder, AccessResult, AccessStorage, BlacklistStorage, ControlStorage, CredentialStorage, FUNCTION_LEVEL, FutureResolve, Injection, ProgramAccess, ProgramId, ProgramRegistryAcquireProgram, ProgramRegistryReleaseProgram, ProgramRegistryReleaseResource, ProgramRegistryReplaceResource, ProgramRegistryReplaceResourceError, ProgramRegistryResolveAsyncError, ProgramRegistryResolveAsyncWithInsertError, ProgramRegistryResolveEitherError, ProgramRegistryResolveError, ProgramRegistryResolveWithInsert, ProgramRegistryResolveWithInsertEitherError, ProgramRegistryResolveWithInsertError, RegistryStorage, ReservationStorage, ResolveResourceError, ResourceAccess, ResourceId, StoredProgram, StoredResource, WhitelistStorage, trace_function};

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

    future_resources: Mutex<RawMutex, HashMap<(ProgramId, ResourceId), Vec<Arc<Mutex<RawMutex, (Option<Waker>, bool)>>>>>
}

impl ProgramRegistry {
    pub fn resolve<'a, T: Injection>(
        self: &'a Arc<Self>, 
        entity: Option<Entity>,
        access_builders: Vec<AccessBuilder>
    ) -> Result<<T as Injection>::Item<'a>, ProgramRegistryResolveError> {
        trace_function!("ProgramRegistry Resolve");

        let submitted_accesses = T::submit_access(access_builders)
            .map_err(|err| ProgramRegistryResolveError::AccessSubmissionError(err))?;

        let derived_results = submitted_accesses
            .into_iter()
            .map(|finalised_access| finalised_access.derive(self))
            .collect::<Vec<_>>();

        Ok(
            T::resolve_access(entity, Arc::clone(self), derived_results)
            .map_err(|err| ProgramRegistryResolveError::ResolveResourceError(err))?
        )
    }

    pub fn resolve_async<'a, T: Injection>(
        self: &'a Arc<Self>, 
        entity: Option<Entity>,
        access_builders: Vec<AccessBuilder>
    ) -> Result<Result<<T as Injection>::Item<'a>, FutureResolve<'a, T>>, ProgramRegistryResolveAsyncError> {
        trace_function!("ProgramRegistry Resolve Async");

        let submitted_accesses = T::submit_access(access_builders)
            .map_err(|err| ProgramRegistryResolveAsyncError::AccessSubmissionError(err))?;

        let derived_results = submitted_accesses.iter().map(|finalised_access| finalised_access.derive(self)).collect::<Vec<_>>();

        let resolved_result = T::resolve_access(entity, Arc::clone(self), derived_results);

        match resolved_result {
            Ok(item) => Ok(Ok(item)),
            Err(ref error @ ResolveResourceError::Casting(ref msg)) |
            Err(ref error @ ResolveResourceError::Resolving(ref msg)) => {
                let span = span!(FUNCTION_LEVEL, "Resolved Error: {} with message: {}", %error, %msg);
                let _enter = span.enter();

                let waker_ready = Arc::new(Mutex::new((None, false)));

                let cached_keys = submitted_accesses
                    .iter()
                    .map(|submitted_access| 
                        (submitted_access.program_id.clone().unwrap_or(self.global_program_id.clone()), submitted_access.resource_id.clone()
                    )).map(|resource_handle| {
                        self.future_resources.lock().entry(resource_handle.clone()).or_default().push(Arc::clone(&waker_ready));
                        resource_handle
                    }).collect::<Vec<_>>();

                Ok(Err(FutureResolve::new(self, entity, submitted_accesses, cached_keys, waker_ready)))
            },
            Err(ResolveResourceError::TooManyResults(msg)) => {
                event!(Level::WARN, "TooManyResults: {}", msg);

                Err(ProgramRegistryResolveAsyncError::ResolvingTooManyResults)
            },
            Err(ResolveResourceError::NotEnoughResults(msg)) => {
                event!(Level::WARN, "NotEnoughResults: {}", msg);

                Err(ProgramRegistryResolveAsyncError::ResolvingNotEnoughResults)
            }
        }
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
        trace_function!("ProgramRegistry Release Program");

        unsafe { self.programs.release_access(&RegistryReleaseAccess {
            resource_id: *program_id,
            access: &ProgramAccess::Shared(1)
        }) }
    }

    /// # Safety
    /// 
    /// Ensure what is being released is actually released
    pub(crate) unsafe fn release_resource(
        &self,
        ProgramRegistryReleaseResource {
            program,
            program_id,
            resource_id,
            resource_access,
        }: &ProgramRegistryReleaseResource<'_>
    ) -> (RegistryReleaseAccessResult, RegistryReleaseAccessResult) {
        trace_function!("ProgramRegistry Release Resource");

        let resource_result = unsafe { program.release_access(&RegistryReleaseAccess {
            resource_id,
            access: resource_access,
        }) };

        let program_result = unsafe { self.release_program(&ProgramRegistryReleaseProgram { 
            program_id 
        }) };

        let future_resources = self.future_resources.lock();
        if let Some(waiters) = future_resources.get(&((*program_id).clone(), (*resource_id).clone())) {
            event!(FUNCTION_LEVEL, waiter_len =? waiters.len(), "Waking waiters");
            
            for waiter in waiters {
                let mut waiter = waiter.lock();
                
                waiter.1 = true;
                
                if let Some(waker) = waiter.0.take() {
                    waker.wake();
                }
            }
        }

        (resource_result, program_result)
    }

    pub(crate) fn acquire_program(
        &self,
        ProgramRegistryAcquireProgram {
            user_details,
            program_id,
            program_password
        }: ProgramRegistryAcquireProgram
    ) -> RegistryAcquireAccessResult<AccessResult<'_, StoredProgram>>{
        let access = ProgramAccess::Shared(1);

        let span = span!(FUNCTION_LEVEL, "ProgramRegistry Acquire Program", access =? access);
        let _enter = span.enter();

        self.programs.acquire_access(RegistryAcquireAccess {
            user_details: user_details,
            resource_id: program_id,
            access,
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
        trace_function!("ProgramRegistry ReplaceResource");

        let program_id = match program_id {
            Some(program_id) => program_id,
            None => self.global_program_id.clone(),
        };

        match self.acquire_program(ProgramRegistryAcquireProgram { 
            user_details, 
            program_id, 
            program_password 
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
            RegistryAcquireAccessResult::NotFound => {
                event!(Level::WARN, "NotFound");
                
                Err(ProgramRegistryReplaceResourceError::NotFound)
            },
            RegistryAcquireAccessResult::AccessConflict => {
                event!(Level::WARN, "AccessConflict");
                
                Err(ProgramRegistryReplaceResourceError::AccessConflict)
            },
            RegistryAcquireAccessResult::ReservationConflict => {
                event!(Level::WARN, "ReservationConflict");
                
                Err(ProgramRegistryReplaceResourceError::ReservationConflict)
            },
            RegistryAcquireAccessResult::VerificationFailure => {
                event!(Level::WARN, "VerificationFailure");
                
                Err(ProgramRegistryReplaceResourceError::VerificationFailure)
            },
            RegistryAcquireAccessResult::OwnershipDenied => {
                event!(Level::WARN, "OwnershipDenied");
                
                Err(ProgramRegistryReplaceResourceError::OwnershipDenied)
            },
            RegistryAcquireAccessResult::WhitelistDenied => {
                event!(Level::WARN, "WhitelistDenied");
                
                Err(ProgramRegistryReplaceResourceError::WhitelistDenied)
            },
            RegistryAcquireAccessResult::BlacklistDenied => {
                event!(Level::WARN, "BlacklistDenied");
                
                Err(ProgramRegistryReplaceResourceError::BlacklistDenied)
            },
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
    ) -> Result<T::Item<'a>, ProgramRegistryResolveWithInsertError> {
        trace_function!("ProgramRegistry Resolve With Insert");

        match self.resolve::<T>(entity, access_builders.clone()) {
            Ok(result) => {
                Ok(result)
            },
            Err(ref error @ ProgramRegistryResolveError::ResolveResourceError(ResolveResourceError::Resolving(ref msg))) |
            Err(ref error @ ProgramRegistryResolveError::ResolveResourceError(ResolveResourceError::Casting(ref msg))) => {
                let span = span!(FUNCTION_LEVEL, "Resolved Error: {} with message: {}", %error, %msg);
                let _enter = span.enter();

                let resource_id = resource_id.ok_or(ProgramRegistryResolveWithInsertError::ExpectedResourceId)?;
                let resource = resource.ok_or(ProgramRegistryResolveWithInsertError::ExpectedResource)?();

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
                    Ok(RegistrySaferReplacementResult::Found(_)) => unreachable!("If there was a resource which could be taken it would have passed the first `resolve`"),
                    Err(ProgramRegistryReplaceResourceError::NotFound) |
                    Ok(RegistrySaferReplacementResult::NotFound) => {
                        trace_function!("Replace Result is NotFound");

                        match self.resolve::<T>(entity, access_builders) {
                            Ok(result) => Ok(result),
                            Err(ProgramRegistryResolveError::AccessSubmissionError(access_submission_error)) => {
                                // only reachable if something changes between the first and last resolve
                                // so can almost assume unreachable!
                                Err(ProgramRegistryResolveWithInsertError::AccessSubmissionError(access_submission_error))
                            },
                            Err(ProgramRegistryResolveError::ResolveResourceError(resolve_resource_error)) => {
                                Err(ProgramRegistryResolveWithInsertError::ReplacedResolveResourceError(resolve_resource_error))
                            },
                        }
                    },
                    Ok(RegistrySaferReplacementResult::NoOp) | 
                    Ok(RegistrySaferReplacementResult::DeniedAccess) => Err(ProgramRegistryResolveWithInsertError::IncompatibleReplacementAccess),
                    Err(ProgramRegistryReplaceResourceError::AccessConflict) |
                    Err(ProgramRegistryReplaceResourceError::ReservationConflict) |
                    Ok(RegistrySaferReplacementResult::ReservationConflict) |
                    Ok(RegistrySaferReplacementResult::AccessConflict) => Err(ProgramRegistryResolveWithInsertError::AccessConflict),
                    Err(ProgramRegistryReplaceResourceError::VerificationFailure) |
                    Ok(RegistrySaferReplacementResult::VerificationFailure) => Err(ProgramRegistryResolveWithInsertError::VerificationFailure),
                    Err(ProgramRegistryReplaceResourceError::OwnershipDenied) |
                    Ok(RegistrySaferReplacementResult::OwnershipDenied) => Err(ProgramRegistryResolveWithInsertError::ExpectedOwnership),
                    Err(ProgramRegistryReplaceResourceError::WhitelistDenied) |
                    Ok(RegistrySaferReplacementResult::WhitelistDenied) => Err(ProgramRegistryResolveWithInsertError::ExpectedWhitelist),
                    Err(ProgramRegistryReplaceResourceError::BlacklistDenied) |
                    Ok(RegistrySaferReplacementResult::BlacklistDenied) => Err(ProgramRegistryResolveWithInsertError::ExpectedBlacklist),
                }
            },
            Err(ProgramRegistryResolveError::ResolveResourceError(ResolveResourceError::NotEnoughResults(msg))) => Err(ProgramRegistryResolveWithInsertError::ResolvingNotEnoughResults(msg)),
            Err(ProgramRegistryResolveError::ResolveResourceError(ResolveResourceError::TooManyResults(msg))) => Err(ProgramRegistryResolveWithInsertError::ResolvingTooManyResults(msg)),
            Err(ProgramRegistryResolveError::AccessSubmissionError(access_submission_error)) => Err(ProgramRegistryResolveWithInsertError::AccessSubmissionError(access_submission_error))
        }
    }

    pub async fn resolve_async_with_insert<'a, T: Injection + 'a>(
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
        }: ProgramRegistryResolveWithInsert<'a>
    ) -> Result<Result<T::Item<'a>, FutureResolve<'a, T>>, ProgramRegistryResolveAsyncWithInsertError> {
        trace_function!("Program Registry Resolve Async With Insert");

        match self.resolve_async::<T>(entity, access_builders.iter().cloned().collect()) {
            Ok(Ok(item)) => {
                Ok(Ok(item))
            },
            Ok(Err(future_resolve)) => {
                let resource_id = resource_id.ok_or(ProgramRegistryResolveAsyncWithInsertError::ExpectedResourceId)?;
                let resource = resource.ok_or(ProgramRegistryResolveAsyncWithInsertError::ExpectedResource)?();

                let access = ResourceAccess::Replace;

                let span = span!(FUNCTION_LEVEL, "Got Future Resolve", access =? access);
                let _enter = span.enter();

                let replace_result = self.replace_resource(ProgramRegistryReplaceResource { 
                    user_details, 
                    program_id, 
                    program_password, 
                    resource: Some(resource), 
                    access: &access,
                    resource_id, 
                    resource_password
                });

                match replace_result {
                    Ok(RegistrySaferReplacementResult::Found(_)) |
                    Err(ProgramRegistryReplaceResourceError::NotFound) |
                    Ok(RegistrySaferReplacementResult::NotFound) |
                    Err(ProgramRegistryReplaceResourceError::ReservationConflict) |
                    Ok(RegistrySaferReplacementResult::ReservationConflict) |
                    Err(ProgramRegistryReplaceResourceError::AccessConflict) |
                    Ok(RegistrySaferReplacementResult::AccessConflict) => Ok(Err(future_resolve)),
                    Ok(RegistrySaferReplacementResult::DeniedAccess) |
                    Ok(RegistrySaferReplacementResult::NoOp) => Err(ProgramRegistryResolveAsyncWithInsertError::IncompatibleReplacementAccess),
                    Err(ProgramRegistryReplaceResourceError::VerificationFailure) |
                    Ok(RegistrySaferReplacementResult::VerificationFailure) => Err(ProgramRegistryResolveAsyncWithInsertError::VerificationFailure),
                    Err(ProgramRegistryReplaceResourceError::OwnershipDenied) |
                    Ok(RegistrySaferReplacementResult::OwnershipDenied) => Err(ProgramRegistryResolveAsyncWithInsertError::ExpectedOwnership),
                    Err(ProgramRegistryReplaceResourceError::BlacklistDenied) |
                    Ok(RegistrySaferReplacementResult::BlacklistDenied) => Err(ProgramRegistryResolveAsyncWithInsertError::ExpectedBlacklist),
                    Err(ProgramRegistryReplaceResourceError::WhitelistDenied) |
                    Ok(RegistrySaferReplacementResult::WhitelistDenied) => Err(ProgramRegistryResolveAsyncWithInsertError::ExpectedWhitelist),
                }
            },
            Err(program_registry_resolve_async_error) => Err(ProgramRegistryResolveAsyncWithInsertError::ProgramRegistryResolveAsyncError(program_registry_resolve_async_error)),
        }
    }

    pub fn resolve_simple_either<'a, T: Injection + 'a>(
        self: &'a Arc<Self>,
        runtime: Option<&Runtime>
    ) -> Result<<T as Injection>::Item<'a>, ProgramRegistryResolveEitherError> {
        let span = span!(FUNCTION_LEVEL, "ProgramRegistry Resolve Simple Either", access_builders =? "Empty");
        let _enter = span.enter();

        self.resolve_either::<T>(runtime, vec![])
    }

    pub fn resolve_either<'a, T: Injection + 'a>(
        self: &'a Arc<Self>,
        runtime: Option<&Runtime>,
        access_builders: Vec<AccessBuilder>,
    ) -> Result<<T as Injection>::Item<'a>, ProgramRegistryResolveEitherError> {
        trace_function!("Program Registry Resolve Either");

        match runtime {
            Some(runtime) => {
                match self.resolve_async::<T>(None, access_builders) {
                    Ok(Ok(item)) => Ok(item),
                    Ok(Err(future_item)) => Ok(runtime.block_on(future_item)),
                    Err(err) => Err(ProgramRegistryResolveEitherError::AsyncError(err)),
                }
            },
            None => {
                match self.resolve::<T>(None, access_builders) {
                    Ok(item) => Ok(item),
                    Err(err) => Err(ProgramRegistryResolveEitherError::SyncError(err))
                }
            },
        }
    }

    pub fn resolve_with_insert_simple_either<'a, T: Injection + 'a>(
        self: &'a Arc<Self>,
        runtime: Option<&Runtime>,
        input: ProgramRegistryResolveWithInsert<'a>,
    ) -> Result<<T as Injection>::Item<'a>, ProgramRegistryResolveWithInsertEitherError> {
        let span = span!(FUNCTION_LEVEL, "ProgramRegistry Resolve With Insert Simple Either", access_builders =? "Empty");
        let _enter = span.enter();

        self.resolve_with_insert_either::<T>(runtime, vec![], input)
    }
    
    pub fn resolve_with_insert_either<'a, T: Injection + 'a>(
        self: &'a Arc<Self>,
        runtime: Option<&Runtime>,
        access_builders: Vec<AccessBuilder>,
        input: ProgramRegistryResolveWithInsert<'a>,
    ) -> Result<<T as Injection>::Item<'a>, ProgramRegistryResolveWithInsertEitherError> {
        trace_function!("Program Registry Resolve With Insert Either");

        match runtime {
            Some(runtime) => {
                match runtime.block_on(self.resolve_async_with_insert::<T>(
                    None, 
                    access_builders,
                    input
                )) {
                    Ok(Ok(item)) => Ok(item),
                    Ok(Err(future_item)) => Ok(runtime.block_on(future_item)),
                    Err(err) => Err(ProgramRegistryResolveWithInsertEitherError::AsyncError(err)),
                }
            },
            None => {
                match self.resolve_with_insert::<T>(
                    None, 
                    access_builders, 
                    input
                ) {
                    Ok(item) => Ok(item),
                    Err(err) => Err(ProgramRegistryResolveWithInsertEitherError::SyncError(err))
                }
            },
        }
    }

    pub fn program_ids(&self) -> impl Iterator<Item = &ProgramId> {
        trace_function!("ProgramRegistry Program Ids");

        self.program_ids.iter().chain(once(&self.global_program_id))
    }

    pub fn global_program_id(&self) -> &ProgramId {
        trace_function!("ProgramRegistry Global Program Id");

        &self.global_program_id
    }
}