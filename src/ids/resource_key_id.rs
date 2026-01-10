use aion_state::prelude::Key;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct ResourceKeyId(u64);

impl Key for ResourceKeyId {}