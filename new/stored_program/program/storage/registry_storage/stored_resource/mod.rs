use crate::prelude::Resource;

pub mod resource;

pub struct StoredResource {
    inner: Resource
}

impl StoredResource {
    pub fn new(resource: Resource) -> Self {
        Self { inner: resource }
    }

    pub fn get(&self) -> &Resource {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut Resource {
        &mut self.inner
    }
}