use std::{fmt::Debug, marker::PhantomData};

use aion_state::prelude::Accessor;

pub mod access_result;

use access_result::AccessResult;

#[derive(Debug)]
pub enum Access<StoredResource, Resource> {
    Shared(usize),
    Unique,
    Owned,
    Replace,


    #[allow(private_interfaces)]
    _Sr(PhantomData<StoredResource>),
    #[allow(private_interfaces)]
    _Rs(PhantomData<Resource>),
}

impl<StoredResource, Resource> Clone for Access<StoredResource, Resource> {
    fn clone(&self) -> Self {
        match self {
            Access::Shared(n) => Access::Shared(*n),
            Access::Unique => Access::Unique,
            Access::Owned => Access::Owned,
            Access::Replace => Access::Replace,
            Access::_Sr(_) |
            Access::_Rs(_) => unreachable!(),
        }
    }
}

impl<StoredResource: Debug, Resource: Debug> Accessor for Access<StoredResource, Resource> {
    type StoredResource = StoredResource;
    type Resource = Resource;

    type AccessResult<'a, T> = AccessResult<'a, T> where T: 'a;

    fn can_access(&self, other: &Self) -> bool {
        todo!()
    }

    fn can_insert(&self) -> bool {
        todo!()
    }

    fn can_remove(&self) -> bool {
        todo!()
    }

    fn is_active(&self) -> bool {
        todo!()
    }

    fn merge_access(&mut self, other: Self) {
        todo!()
    }

    fn split_access(&mut self, other: &Self) {
        todo!()
    }

    fn access<'a>(&self, resource: &'a Self::StoredResource) -> Self::AccessResult<'a, Self::Resource> {
        todo!()
    }

    fn remove<'a>(&self, resource: Self::StoredResource) -> Self::AccessResult<'a, Self::StoredResource> {
        todo!()
    }
}