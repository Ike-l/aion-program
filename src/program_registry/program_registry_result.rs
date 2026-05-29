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

#[derive(Debug, thiserror::Error)]
pub enum ProgramRegistryResolveWithInsertError {
    #[error("Expected ResourceId When Inserting")]
    ExpectedResourceId,
    #[error("Expected Resource When Inserting")]
    ExpectedResource,
    #[error("Expected an Entity")]
    ExpectedEntity,
    #[error("Expected Whitelist or Blacklist, on program access: {on_program}")]
    ListsDenied { on_program: bool },
    #[error("Expected to be Verified, on program access: {on_program}")]
    ExpectedVerified { on_program: bool },
    #[error("Expected Ownership over Resource, on program access: {on_program}")]
    ExpectedOwnership { on_program: bool },
    #[error("Access Conflict. On program access: {on_program}")]
    AccessConflict { on_program: bool },
    #[error("Reservation Conflict, on program access: {on_program}")]
    ReservationConflict { on_program: bool },
    #[error("When Resolving: {0}")]
    AccessSubmissionError(#[from] AccessSubmissionError),
    #[error("Resolving Expected Results: {expected}, Found: {found}")]
    ExpectedResults { expected: usize, found: usize },
    #[error("After Insert got Error: {0}")]
    ResolvingAfterInsert(#[from] ProgramRegistryResolveError),
    #[error("Unknown Error: {0}")]
    UnknownError(anyhow::Error)
}

#[derive(Debug, thiserror::Error)]
pub enum ProgramRegistryResolveError {
    #[error("When Resolving: {0}")]
    AccessSubmissionError(#[from] AccessSubmissionError),
    #[error("When Resolving: {0}")]
    ResolveResourceError(#[from] ResolveResourceError),
}

#[derive(Debug, thiserror::Error)]
pub enum ProgramRegistryResolveAsyncError {
    #[error("Access Submission Error: {0}")]
    AccessSubmissionError(#[from] AccessSubmissionError),
    #[error("Resolving Expected Results: {expected}, Found: {found}")]
    ExpectedResults {
        expected: usize,
        found: usize
    },
    #[error("Expected an Entity")]
    ExpectedEntity,
    #[error("Unknown Error: {0}")]
    UnknownError(anyhow::Error)
}

#[derive(Debug, thiserror::Error)]
pub enum ProgramRegistryResolveAsyncWithInsertError {
    #[error("Expected ResourceId When Inserting")]
    ExpectedResourceId,
    #[error("Expected Resource When Inserting")]
    ExpectedResource,
    #[error("Expected Whitelist or Blacklist, on program access: {on_program}")]
    ListsDenied { on_program: bool },
    #[error("Expected to be Verified, on program access: {on_program}")]
    ExpectedVerified { on_program: bool },
    #[error("Expected Ownership over Resource, on program access: {on_program}")]
    ExpectedOwnership { on_program: bool },
    #[error("When Resolving got Error: {0}")]
    ProgramRegistryResolveAsyncError(#[from] ProgramRegistryResolveAsyncError),
}

pub enum ProgramRegistryResolveEitherError {
    SyncError(ProgramRegistryResolveError),
    AsyncError(ProgramRegistryResolveAsyncError)
}

pub enum ProgramRegistryResolveWithInsertEitherError {
    SyncError(ProgramRegistryResolveWithInsertError),
    AsyncError(ProgramRegistryResolveAsyncWithInsertError)
}