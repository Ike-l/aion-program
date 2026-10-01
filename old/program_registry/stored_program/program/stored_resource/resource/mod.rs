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

pub trait ValueTrait {
    fn as_ref<T: 'static>(&self) -> Result<&T, DowncastError>;
    fn as_mut<T: 'static>(&mut self) -> Result<&mut T, DowncastError>;
    fn into<T: 'static>(self) -> Result<T, DowncastError>;
}

impl ValueTrait for Resource {
    fn as_ref<T: 'static>(&self) -> Result<&T, DowncastError> {
        self.inner
            .downcast_ref()
            .ok_or_else(|| self.downcast_error::<T>())
    }

    fn as_mut<T: 'static>(&mut self) -> Result<&mut T, DowncastError> {
        let error = self.downcast_error::<T>();
        self.inner.downcast_mut().ok_or(error)
    }

    fn into<T: 'static>(self) -> Result<T, DowncastError> {
        let error = self.downcast_error::<T>();
        self.inner
            .downcast::<T>()
            .map(|boxed| *boxed)
            .map_err(|_| error)
    }
}

impl Resource {
    pub fn downcast_error<T: 'static>(&self) -> DowncastError {
        DowncastError::Downcasting {
            expected: type_name::<T>(),
            found: self.name,
        }
    }
}

impl Debug for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Resource> with name: {}", self.name)
    }
}