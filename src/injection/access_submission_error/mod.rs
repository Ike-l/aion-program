use std::fmt::Display;

pub enum AccessSubmissionError {
    NotEnoughPrompts(String),
    TooManyPrompts(String)
}

impl Display for AccessSubmissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::NotEnoughPrompts(msg) => format!("NotEnoughPrompts: {}", msg),
            Self::TooManyPrompts(msg) => format!("TooManyPrompts: {}", msg),
        };

        write!(f, "{}", s)
    }
}