use aion_state::prelude::Registry;

use crate::prelude::Storage;

pub mod storage;

pub struct Program {
    state: Registry<Storage>
}

impl Program {
    
}