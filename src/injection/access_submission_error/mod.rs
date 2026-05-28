// use crate::prelude::{ProgramId, ResourceAccess, ResourceId, UserId};

#[derive(Debug, thiserror::Error)]
pub enum AccessSubmissionError {
    #[error("Expected Prompts: {expected}, Found: {found}")]
    ExpectedPrompts {
        expected: usize, 
        found: usize
    },
    // #[error("Expected ProgramId: {0:?} | None: Expected `Global` ProgramId")]
    // ExpectedProgramId(Option<ProgramId>),
    // #[error("Expected UserDetails with UserId: {0:?}")]
    // ExpectedUserDetails(UserId),
    // #[error("Expected ResourceId: {0:?}")]
    // ExpectedResourceId(ResourceId),
    // #[error("Expected ResourceAccess: {0:?}")]
    // ExpectedResourceAccess(ResourceAccess),
    // #[error("Expected a ProgramPassword")]
    // ExpectedProgramPassword,
    // #[error("Unknown Error: {0}")]
    // Unknown(anyhow::Error)
}

#[cfg(test)]
mod tests {
    use super::AccessSubmissionError;

    #[test]
    fn display_expected_prompts_error() {
        let error = AccessSubmissionError::ExpectedPrompts { expected: 1, found: 0 };
        let display = error.to_string();
        assert_eq!(display, "Expected Prompts: 1, Found: 0")
    }
}
