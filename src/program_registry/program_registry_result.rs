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