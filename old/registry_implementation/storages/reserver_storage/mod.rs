use std::collections::HashMap;

use aion_state::prelude::Accesses;
use tracing::{Level, event};

use crate::prelude::{AccessStorage, UserId};

pub struct ReservationStorage<ValueId, Access> {
    inner: HashMap<UserId, Accesses<AccessStorage<ValueId, Access>>>
}

impl<ValueId, Access> Default for ReservationStorage<ValueId, Access> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<ValueId, Access> aion_state::prelude::ReservationStorage for ReservationStorage<ValueId, Access> {
    type ReserverId = UserId;
    type AccessStorage = AccessStorage<ValueId, Access>;

    fn get_mut(
        &mut self, 
        key: &Self::ReserverId
    ) -> Option<&mut Accesses<Self::AccessStorage>> {
        event!(Level::TRACE, "ReservationStorage get mut");

        self.inner.get_mut(key)
    }

    fn insert(
        &mut self,
        key: Self::ReserverId,
        accesses: Accesses<Self::AccessStorage>
    ) -> Option<Accesses<Self::AccessStorage>> {
        event!(Level::TRACE, "ReservationStorage insert");

        self.inner.insert(key, accesses)
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = (
        &'a Self::ReserverId, 
        &'a Accesses<Self::AccessStorage>
    )> 
        where Self: 'a {
        event!(Level::TRACE, "ReservationStorage iter");

        self.inner.iter()
    }
}