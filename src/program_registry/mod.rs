use std::{collections::{HashMap, HashSet}, iter::once, sync::Arc, task::Waker};

use aion_state::prelude::{ReceptionGetAccess, Registry, RegistryAcquireAccess, RegistryAcquireAccessError, RegistryReleaseAccess, RegistryReleaseAccessResult, RegistrySaferReplacement, RegistrySaferReplacementResult};
use hecs::Entity;
use parking_lot::{RawMutex, lock_api::Mutex};
use tokio::runtime::Runtime;
use tracing::{Level, event, span};

use crate::prelude::{AccessBuilder, AccessResult, AccessStorage, BlacklistStorage, ControlStorage, CredentialStorage, FUNCTION_LEVEL, FutureResolve, Injection, ProgramAccess, ProgramId, ProgramRegistryAcquireProgram, ProgramRegistryReleaseProgram, ProgramRegistryReleaseResource, ProgramRegistryReplaceResource, ProgramRegistryResolveAsyncError, ProgramRegistryResolveAsyncWithInsertError, ProgramRegistryResolveEitherError, ProgramRegistryResolveError, ProgramRegistryResolveWithInsert, ProgramRegistryResolveWithInsertEitherError, ProgramRegistryResolveWithInsertError, RegistryStorage, ReservationStorage, ResolveResourceError, ResourceAccess, ResourceId, StoredProgram, StoredResource, WhitelistStorage, trace_function};

pub mod program_id;
pub mod stored_program;
pub mod program_access;
pub mod resolved_resource;
pub mod derived_error;
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

impl Default for ProgramRegistry {
    fn default() -> Self {
        let global_program_id = ProgramId::Label("Global Program Id".to_owned());
        
        let program_ids = HashSet::from_iter(once(global_program_id.clone()));

        let programs = Registry::default();

        assert!(matches!(programs.safer_replace(RegistrySaferReplacement {
            user_details: None,
            access: &ProgramAccess::Replace,
            resource_id: global_program_id.clone(),
            resource: Some(StoredProgram::default()),
            password: None,
        }), RegistrySaferReplacementResult::NotFound));

        Self {
            program_ids,
            global_program_id,
            programs,
            future_resources: Mutex::new(HashMap::default())
        }
    }
}

impl ProgramRegistry {
    pub fn resolve<'a, T: Injection>(
        self: &'a Arc<Self>, 
        entity: Option<Entity>,
        access_builders: Vec<AccessBuilder>
    ) -> Result<<T as Injection>::Item<'a>, ProgramRegistryResolveError> {
        trace_function!("ProgramRegistry Resolve");

        let submitted_accesses = T::submit_access(access_builders)?;

        let derived_results = submitted_accesses
            .into_iter()
            .map(|finalised_access| finalised_access.derive(self))
            .collect::<Vec<_>>();

        Ok(T::resolve_access(entity, Arc::clone(self), derived_results)?)
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


        let err = match resolved_result {
            Ok(item) => return Ok(Ok(item)),
            Err(err) => err,
        };

        let span = span!(FUNCTION_LEVEL, "Resolved Error: {}", %err);
        let _enter = span.enter();

        match err {
            ResolveResourceError::ExpectedResults { expected, found } => {
                event!(Level::WARN, "Input is malformed so cannot try again");

                Err(ProgramRegistryResolveAsyncError::ExpectedResults { expected, found })
            },
            ResolveResourceError::UnknownError(err) => {
                event!(Level::WARN, "Unknown Error");

                Err(ProgramRegistryResolveAsyncError::UnknownError(err))
            }
            ResolveResourceError::CanWaitUnknownError(_) |
            ResolveResourceError::Casting(_) |
            ResolveResourceError::Deriving(_) => {
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
        } else { event!(FUNCTION_LEVEL, "No Futures waiting on resources") }

        (resource_result, program_result)
    }

    pub(crate) fn acquire_program(
        &self,
        ProgramRegistryAcquireProgram {
            user_details,
            program_id,
            program_password
        }: ProgramRegistryAcquireProgram
    ) -> Result<AccessResult<'_, StoredProgram>, RegistryAcquireAccessError>{
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
    ) -> Result<RegistrySaferReplacementResult<StoredResource>, RegistryAcquireAccessError> {
        trace_function!("ProgramRegistry ReplaceResource");

        let program_id = match program_id {
            Some(program_id) => program_id,
            None => self.global_program_id.clone(),
        };

        match self.acquire_program(ProgramRegistryAcquireProgram { 
            user_details, 
            program_id: program_id.clone(), 
            program_password 
        }) {
            Ok(access_result) => {
                let program = access_result.as_ref().unwrap();
                let result = program.safer_replace(
                    RegistrySaferReplacement {
                        user_details,
                        access,
                        resource_id,
                        resource,
                        password: resource_password,
                    }
                );

                unsafe { self.release_program(&ProgramRegistryReleaseProgram { program_id: &program_id }) };

                Ok(result)
            },
            Err(err) => {
                event!(Level::WARN, %err, "Acquiring Program");

                Err(err)
            }
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

        let result = self.resolve::<T>(entity, access_builders.clone());

        let err = match result {
            Ok(item) => return Ok(item),
            Err(err) => err
        };

        event!(Level::WARN, %err, "Continuing to Insert");

        match err {
            ProgramRegistryResolveError::AccessSubmissionError(err) => {
                return Err(err.into())
            },
            ProgramRegistryResolveError::ResolveResourceError(err) => {
                match err {
                    ResolveResourceError::CanWaitUnknownError(err) |
                    ResolveResourceError::UnknownError(err) => {
                        return Err(ProgramRegistryResolveWithInsertError::UnknownError(err))
                    }
                    ResolveResourceError::ExpectedResults { expected, found } => {
                        return Err(ProgramRegistryResolveWithInsertError::ExpectedResults { expected, found })
                    },
                    ResolveResourceError::Casting(_) |
                    ResolveResourceError::Deriving(_) => {
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
                            Ok(RegistrySaferReplacementResult::NotFound) |
                            Err(RegistryAcquireAccessError::NotFound) => {
                                return Ok(self.resolve::<T>(entity, access_builders)?);
                            },
                            Ok(RegistrySaferReplacementResult::AccessConflict) => return Err(ProgramRegistryResolveWithInsertError::AccessConflict { on_program: false }),
                            Err(RegistryAcquireAccessError::AccessConflict) => return Err(ProgramRegistryResolveWithInsertError::AccessConflict { on_program: true }),

                            Ok(RegistrySaferReplacementResult::ReservationConflict) => return Err(ProgramRegistryResolveWithInsertError::ReservationConflict { on_program: false }),
                            Err(RegistryAcquireAccessError::ReservationConflict) => return Err(ProgramRegistryResolveWithInsertError::ReservationConflict { on_program: true }),

                            Ok(RegistrySaferReplacementResult::BlacklistDenied) => return Err(ProgramRegistryResolveWithInsertError::ExpectedBlacklist { on_program: false }),
                            Err(RegistryAcquireAccessError::BlacklistDenied) => return Err(ProgramRegistryResolveWithInsertError::ExpectedBlacklist { on_program: false }),

                            Ok(RegistrySaferReplacementResult::WhitelistDenied) => return Err(ProgramRegistryResolveWithInsertError::ExpectedWhitelist { on_program: false }),
                            Err(RegistryAcquireAccessError::WhitelistDenied) => return Err(ProgramRegistryResolveWithInsertError::ExpectedWhitelist { on_program: false }),

                            Ok(RegistrySaferReplacementResult::VerificationFailure) => return Err(ProgramRegistryResolveWithInsertError::ExpectedVerified { on_program: false }),
                            Err(RegistryAcquireAccessError::VerificationFailure) => return Err(ProgramRegistryResolveWithInsertError::ExpectedVerified { on_program: false }),

                            Ok(RegistrySaferReplacementResult::OwnershipDenied) => return Err(ProgramRegistryResolveWithInsertError::ExpectedOwnership { on_program: false }),
                            Err(RegistryAcquireAccessError::OwnershipDenied) => return Err(ProgramRegistryResolveWithInsertError::ExpectedOwnership { on_program: false }),

                            Ok(RegistrySaferReplacementResult::NoOp) |
                            Ok(RegistrySaferReplacementResult::DeniedAccess) => unreachable!("Hard Coded 'Replacement' access"),
                            
                        }

                    },
                }
            }
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
                    Err(RegistryAcquireAccessError::NotFound) |
                    Ok(RegistrySaferReplacementResult::NotFound) |
                    Err(RegistryAcquireAccessError::ReservationConflict) |
                    Ok(RegistrySaferReplacementResult::ReservationConflict) |
                    Err(RegistryAcquireAccessError::AccessConflict) |
                    Ok(RegistrySaferReplacementResult::AccessConflict) => Ok(Err(future_resolve)),

                    Ok(RegistrySaferReplacementResult::VerificationFailure) => Err(ProgramRegistryResolveAsyncWithInsertError::ExpectedVerified { on_program: false }),
                    Err(RegistryAcquireAccessError::VerificationFailure) => Err(ProgramRegistryResolveAsyncWithInsertError::ExpectedVerified { on_program: true }),
                    Ok(RegistrySaferReplacementResult::OwnershipDenied) => Err(ProgramRegistryResolveAsyncWithInsertError::ExpectedOwnership { on_program: false }),
                    Err(RegistryAcquireAccessError::OwnershipDenied) => Err(ProgramRegistryResolveAsyncWithInsertError::ExpectedOwnership { on_program: true }),
                    Ok(RegistrySaferReplacementResult::BlacklistDenied) => Err(ProgramRegistryResolveAsyncWithInsertError::ExpectedBlacklist { on_program: false }),
                    Err(RegistryAcquireAccessError::BlacklistDenied) => Err(ProgramRegistryResolveAsyncWithInsertError::ExpectedBlacklist { on_program: true }),
                    Ok(RegistrySaferReplacementResult::WhitelistDenied) => Err(ProgramRegistryResolveAsyncWithInsertError::ExpectedWhitelist { on_program: false }),
                    Err(RegistryAcquireAccessError::WhitelistDenied) => Err(ProgramRegistryResolveAsyncWithInsertError::ExpectedWhitelist { on_program: true }),

                    Ok(RegistrySaferReplacementResult::DeniedAccess) |
                    Ok(RegistrySaferReplacementResult::NoOp) => unreachable!("Hard Coded 'Replacement' access"),
                }
            },
            Err(err) => Err(err.into()),
        }
    }

    pub fn resolve_simple_either<'a, T: Injection + 'a>(
        self: &'a Arc<Self>,
        runtime: Option<&Runtime>
    ) -> Result<<T as Injection>::Item<'a>, ProgramRegistryResolveEitherError> {
        let span = span!(FUNCTION_LEVEL, "ProgramRegistry Resolve Simple Either", access_builders =? "Empty");
        let _enter = span.enter();

        self.resolve_either::<T>(runtime, None, vec![])
    }

    pub fn resolve_either<'a, T: Injection + 'a>(
        self: &'a Arc<Self>,
        runtime: Option<&Runtime>,
        entity: Option<Entity>,
        access_builders: Vec<AccessBuilder>,
    ) -> Result<<T as Injection>::Item<'a>, ProgramRegistryResolveEitherError> {
        trace_function!("Program Registry Resolve Either");

        match runtime {
            Some(runtime) => {
                match self.resolve_async::<T>(entity, access_builders) {
                    Ok(Ok(item)) => Ok(item),
                    Ok(Err(future_item)) => Ok(runtime.block_on(future_item)),
                    Err(err) => Err(ProgramRegistryResolveEitherError::AsyncError(err)),
                }
            },
            None => {
                match self.resolve::<T>(entity, access_builders) {
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
        self.program_ids.iter().chain(once(&self.global_program_id))
    }

    pub fn global_program_id(&self) -> &ProgramId {
        &self.global_program_id
    }

    pub fn get_program_access(&self, program_id: Option<&ProgramId>) -> Option<ProgramAccess> {
        let program_id = match program_id {
            Some(program_id) => program_id,
            None => &self.global_program_id,
        };

        self.programs.get_access(&ReceptionGetAccess { access_id: program_id })
    }
}