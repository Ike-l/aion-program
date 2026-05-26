pub enum AccessSubmissionError {
    NotEnoughPrompts(String),
    TooManyPrompts(String)
}