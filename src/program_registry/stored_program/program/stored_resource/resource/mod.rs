use std::{any::{Any, type_name}, fmt::Debug};

use crate::prelude::DowncastError;

pub mod downcast_error;

pub struct Resource {
    name: &'static str,
    inner: Box<dyn Any + Send + Sync>,
}

impl Resource {
    pub fn new<T: 'static + Sync + Send>(value: T) -> Self {
        Self {
            name: type_name::<T>(),
            inner: Box::new(value)
        }
    }
}

impl Resource {
    pub fn as_ref<T: 'static>(&self) -> Result<&T, DowncastError> {
        self.inner.downcast_ref().ok_or(DowncastError::Downcasting { expected: type_name::<T>(), found: self.name })
    }

    pub fn as_mut<T: 'static>(&mut self) -> Result<&mut T, DowncastError> {
        self.inner.downcast_mut().ok_or(DowncastError::Downcasting { expected: type_name::<T>(), found: self.name })
    }
}

impl Debug for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Resource> with name: {}", self.name)
    }
}