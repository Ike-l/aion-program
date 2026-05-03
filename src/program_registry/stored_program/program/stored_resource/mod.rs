use crate::prelude::Resource;

pub mod resource;

pub struct StoredResource {
    resource: Resource
}

impl StoredResource {
    pub fn new(resource: Resource) -> Self {
        Self { resource }
    }

    pub fn get(&self) -> &Resource {
        &self.resource
    }

    pub fn get_mut(&mut self) -> &mut Resource {
        &mut self.resource
    }
}

