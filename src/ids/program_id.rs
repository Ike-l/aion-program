use aion_state::prelude::{AccessKey, ResourceKey};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ProgramId(String);

impl ProgramId {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self(name.into())
    }
}

impl ResourceKey for ProgramId {}
impl AccessKey for ProgramId {}