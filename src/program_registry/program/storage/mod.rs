use aion_state::prelude::StorageTrait;

use crate::prelude::{Access, AccessStorage, BlacklistStorage, CredentialStorage, RegistryStorage, ReservationStorage, ResourceId, StoredResource, UserId, WhitelistStorage};

pub struct Storage;

pub mod registry_storage;
pub mod access_storage;
pub mod reserver_storage;
pub mod credential_storage;
pub mod whitelist_storage;
pub mod blacklist_storage;
pub mod control_storage;

impl StorageTrait for Storage {
    type Value = StoredResource;
    type ValueId = ResourceId;
    type S = RegistryStorage;

    type Access = Access;
    type AS = AccessStorage;

    type ReserverId = UserId;
    type RS = ReservationStorage;

    type OS = CredentialStorage;

    type WS = WhitelistStorage;
    type BS = BlacklistStorage;

    type CS;
}