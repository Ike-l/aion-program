use crate::prelude::{CastError, DerivedError};

#[derive(Debug, thiserror::Error)]
pub enum ResolveResourceError {
    #[error("Expected Results: {expected}, Found: {found}")]
    ExpectedResults {
        expected: usize, 
        found: usize
    },
    #[error("Failed to downcast: {0}")]
    Casting(#[from] CastError),
    #[error("Failed to derive resource: {0}")]
    Deriving(#[from] DerivedError),
    #[error("Expected an Entity")]
    ExpectedEntity,
    #[error("Can Wait on Unknown Error: {0}")]
    CanWaitUnknownError(anyhow::Error),
    #[error("Unknown Error: {0}")]
    UnknownError(anyhow::Error),
}