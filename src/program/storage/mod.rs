use aion_state::prelude::StorageTrait;

use crate::prelude::{Access, AccessStorage, RegistryStorage, ResourceId, StoredResource};

pub struct Storage;

pub mod registry_storage;
pub mod access_storage;
pub mod reserver_storage;

impl StorageTrait for Storage {
    type Value = StoredResource;
    type ValueId = ResourceId;
    type S = RegistryStorage;

    type Access = Access;
    type AS = AccessStorage;

    type ReserverId;
    type RS;

    type OS;

    type WS;
    type BS;

    type CS;
}