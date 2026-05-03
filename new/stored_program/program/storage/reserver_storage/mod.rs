use std::collections::HashMap;

use aion_state::prelude::Accesses;
use tracing::{Level, event};

use crate::prelude::{AccessStorage, UserId};

pub mod user_id;

pub struct ReservationStorage {
    inner: HashMap<UserId, Accesses<AccessStorage>>
}

impl Default for ReservationStorage {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl aion_state::prelude::ReservationStorage for ReservationStorage {
    type ReserverId = UserId;
    type AccessStorage = AccessStorage;

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