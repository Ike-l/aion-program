use crate::prelude::DowncastError;

#[derive(Debug, thiserror::Error)]
pub enum CastError {
    #[error("Failed to downcast with a Shared AccessResult: {0}")]
    Shared(DowncastError),
    #[error("Failed to downcast with a Unique AccessResult: {0}")]
    Unique(DowncastError),
}