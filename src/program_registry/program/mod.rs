use aion_state::prelude::Registry;

use crate::prelude::{Injection, Storage};

pub mod storage;

pub struct Program {
    state: Registry<Storage>
}

impl Program {
    pub fn resolve<T: Injection>(&self) {
        
    }
}