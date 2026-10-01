
#[derive(Debug, thiserror::Error, Clone)]
pub enum DowncastError {
    #[error("Failed to Downcast from type: {found}, to {expected}")]
    Downcasting {
        expected: &'static str,
        found: &'static str,
    },
}