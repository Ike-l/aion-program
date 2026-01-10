use aion_state::prelude::ReserverKey;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ProgramReserverId(String);

impl ReserverKey for ProgramReserverId {}