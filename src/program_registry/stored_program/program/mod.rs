use crate::prelude::{AutoRegistry, ResourceId, StoredResource, ResourceAccess};

pub mod resource_id;
pub mod stored_resource;
pub mod resource_access;

pub type Program = AutoRegistry<ResourceId, StoredResource, ResourceAccess>;
