use aion_state::prelude::Key;

#[derive(Hash, PartialEq, Eq)]
pub struct ProgramKeyId(u64);

impl Key for ProgramKeyId {}