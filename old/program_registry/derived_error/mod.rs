use aion_state::prelude::RegistryAcquireAccessError;

#[derive(Debug, thiserror::Error)]
pub enum DerivedError {
    #[error("When Accessing Resource: {0}")]
    ResourceAccessNotFound(RegistryAcquireAccessError),
    #[error("When Accessing Program: {0}")]
    ProgramAccessNotFound(RegistryAcquireAccessError)
}