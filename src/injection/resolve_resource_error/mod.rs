use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ResolveResourceError {
    Casting(String),
    Resolving(String),
    NotEnoughResults(String),
    TooManyResults(String),
}

impl Display for ResolveResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResolveResourceError::Casting(msg) => format!("Casting: {}", msg),
            ResolveResourceError::Resolving(msg) => format!("Resolving: {}", msg),
            ResolveResourceError::NotEnoughResults(msg) => format!("NotEnoughResults: {}", msg),
            ResolveResourceError::TooManyResults(msg) => format!("TooManyResults: {}", msg),
        };

        write!(f, "{}", s)
    }
}