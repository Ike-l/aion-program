use crate::prelude::Resource;

pub mod resource;

pub type StoredResource = Box<Resource>;

pub trait StoredResourceTrait {
    type Resource;

    fn new(resource: Self::Resource) -> Self;

    fn get(&self) -> &Self::Resource;

    fn get_mut(&mut self) -> &mut Self::Resource;
}

impl StoredResourceTrait for StoredResource {
    type Resource = Resource;

    fn new(resource: Self::Resource) -> Self {
        Box::new(resource)
    }

    fn get(&self) -> &Self::Resource {
        self
    }

    fn get_mut(&mut self) -> &mut Self::Resource {
        self
    }
}

