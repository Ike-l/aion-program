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
    ResolvingNotEnoughResults(String),
    ResolvingTooManyResults(String),
    AccessConflict,
    ReplacedResolveResourceError(ResolveResourceError),
    AccessSubmissionError(AccessSubmissionError)
}

impl Display for ProgramRegistryResolveWithInsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ProgramRegistryResolveWithInsertError::ExpectedResourceId => "ExpectedResourceId",
            ProgramRegistryResolveWithInsertError::ExpectedResource => "ExpectedResource",
            ProgramRegistryResolveWithInsertError::IncompatibleReplacementAccess => "IncompatibleReplacementAccess",
            ProgramRegistryResolveWithInsertError::VerificationFailure => "VerificationFailure",
            ProgramRegistryResolveWithInsertError::ExpectedOwnership => "ExpectedOwnership",
            ProgramRegistryResolveWithInsertError::ExpectedBlacklist => "ExpectedBlacklist",
            ProgramRegistryResolveWithInsertError::ExpectedWhitelist => "ExpectedWhitelist",
            ProgramRegistryResolveWithInsertError::ResolvingNotEnoughResults(msg) => &format!("ResolvingNotEnoughResults: {msg}"),
            ProgramRegistryResolveWithInsertError::ResolvingTooManyResults(msg) => &format!("ResolvingTooManyResults: {msg}"),
            ProgramRegistryResolveWithInsertError::AccessConflict => "AccessConflict",
            ProgramRegistryResolveWithInsertError::ReplacedResolveResourceError(resolve_resource_error) => &format!("ReplacedResolveResourceError: {resolve_resource_error}"),
            ProgramRegistryResolveWithInsertError::AccessSubmissionError(access_submission_error) => &format!("AccessSubmissionError: {access_submission_error}"),
        };

        write!(f, "{}", s)
    }
}

pub enum ProgramRegistryResolveError {
    AccessSubmissionError(AccessSubmissionError),
    ResolveResourceError(ResolveResourceError),
}

impl Display for ProgramRegistryResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ProgramRegistryResolveError::AccessSubmissionError(access_submission_error) => {
                format!("ProgramRegistryResolveError: {}", access_submission_error)
            },
            ProgramRegistryResolveError::ResolveResourceError(resolve_resource_error) => {
                format!("ProgramRegistryResolveError: {}", resolve_resource_error)
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