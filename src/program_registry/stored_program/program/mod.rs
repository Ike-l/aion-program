use aion_state::prelude::Registry;

use crate::prelude::{AccessStorage, BlacklistStorage, ControlStorage, CredentialStorage, RegistryStorage, ReservationStorage, ResourceAccess, ResourceId, StoredResource, WhitelistStorage};

pub mod resource_id;
pub mod stored_resource;
pub mod resource_access;

pub type Program = Registry<
        RegistryStorage<ResourceId, StoredResource>,
        ReservationStorage<ResourceId, ResourceAccess>,
        AccessStorage<ResourceId, ResourceAccess>,
        CredentialStorage,
        WhitelistStorage<ResourceId, ResourceAccess>,
        BlacklistStorage<ResourceId, ResourceAccess>,
        ControlStorage<ResourceId>
>;
