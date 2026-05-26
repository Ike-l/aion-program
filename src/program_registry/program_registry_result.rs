use std::fmt::Display;

use crate::prelude::{AccessSubmissionError, ResolveResourceError};

pub enum ProgramRegistryReplaceResourceError {
    NotFound,
    AccessConflict,
    ReservationConflict,
    VerificationFailure,
    OwnershipDenied,
    WhitelistDenied,
    BlacklistDenied,
}

pub enum ProgramRegistryResolveWithInsertError {
    ExpectedResourceId,
    ExpectedResource,
    IncompatibleReplacementAccess,
    VerificationFailure,
    ExpectedOwnership,
    ExpectedBlacklist,
    ExpectedWhitelist,
    ResolvingNotEnoughResults,
    ResolvingTooManyResults,
    AccessConflict,
    ReplacedResolveResourceError(ResolveResourceError),
    AccessSubmissionError(AccessSubmissionError)
}

pub enum ProgramRegistryResolveError {
    AccessSubmissionError(AccessSubmissionError),
    ResolveResourceError(ResolveResourceError),
}

impl Display for ProgramRegistryResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ProgramRegistryResolveError::AccessSubmissionError(access_submission_error) => {
                match access_submission_error {
                    AccessSubmissionError::NotEnoughPrompts(msg) => format!("NotEnoughPrompts: {}", msg),
                    AccessSubmissionError::TooManyPrompts(msg) => format!("TooManyPrompts: {}", msg),
                }
            },
            ProgramRegistryResolveError::ResolveResourceError(resolve_resource_error) => {
                match resolve_resource_error {
                    ResolveResourceError::Casting(msg) => format!("Casting: {}", msg),
                    ResolveResourceError::Resolving(msg) => format!("Resolving: {}", msg),
                    ResolveResourceError::NotEnoughResults(msg) => format!("NotEnoughResults: {}", msg),
                    ResolveResourceError::TooManyResults(msg) => format!("TooManyResults: {}", msg),
                }
            },
        };

        write!(f, "{}", s)
    }
}

pub enum ProgramRegistryResolveAsyncError {
    AccessSubmissionError(AccessSubmissionError),
    ResolvingNotEnoughResults,
    ResolvingTooManyResults
}

pub enum ProgramRegistryResolveAsyncWithInsertError {
    ExpectedResourceId,
    ExpectedResource,
    IncompatibleReplacementAccess,
    VerificationFailure,
    ExpectedOwnership,
    ExpectedBlacklist,
    ExpectedWhitelist,
    ProgramRegistryResolveAsyncError(ProgramRegistryResolveAsyncError),
}

pub enum ProgramRegistryResolveEitherError {
    SyncError(ProgramRegistryResolveError),
    AsyncError(ProgramRegistryResolveAsyncError)
}

pub enum ProgramRegistryResolveWithInsertEitherError {
    SyncError(ProgramRegistryResolveWithInsertError),
    AsyncError(ProgramRegistryResolveAsyncWithInsertError)
}