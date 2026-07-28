use crate::prelude::{AutoRegistry, ResourceId, StoredResource, Access};

pub mod resource_id;
pub mod stored_resource;

pub type Program = AutoRegistry<ResourceId, StoredResource, Access>;
