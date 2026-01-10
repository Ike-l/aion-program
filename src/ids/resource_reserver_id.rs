use aion_state::prelude::ReserverKey;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ResourceReserverId(String);

impl ReserverKey for ResourceReserverId {}