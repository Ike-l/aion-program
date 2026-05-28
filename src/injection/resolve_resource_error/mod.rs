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
    Deriving(#[from] DerivedError)
}