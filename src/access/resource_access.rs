use aion_state::prelude::Accessor;

use crate::prelude::{AccessResult, BorrowType, Resource, StoredResource};

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceAccess {
    Shared(usize),
    Unique,
    Replace,
}

impl ResourceAccess {
    pub fn borrow_type(&self) -> BorrowType {
        match self {
            Self::Shared(0) => BorrowType::Instant,
            
            Self::Replace => BorrowType::Instant,

            Self::Shared(_) => BorrowType::Held,
            Self::Unique => BorrowType::Held,
        }
    }
}

impl Accessor for ResourceAccess {
    type AccessResult<'a, T> = AccessResult<'a, T> where T: 'a;
    type Resource = Resource;
    type StoredResource = StoredResource;
    
    fn can_access(&self, other: &Self) -> bool {
        match (self, other) {
            (ResourceAccess::Shared(_), ResourceAccess::Shared(_)) => true,
            (ResourceAccess::Replace, _) => true,
            (ResourceAccess::Shared(0), _) => true,
            _ => false
        }
    }
    
    fn can_insert(&self) -> bool {
        *self == ResourceAccess::Replace
    }
    
    fn can_remove(&self) -> bool {
        *self == ResourceAccess::Replace
    }
    
    fn is_active(&self) -> bool {
        self.borrow_type() == BorrowType::Held
    }
    
    fn merge_access(&mut self, other: Self) {
        match (self, other) {
            (ResourceAccess::Shared(n), ResourceAccess::Shared(m)) => *n += m,
            (lhs @ &mut ResourceAccess::Replace, rhs @ _) => *lhs = rhs,
            
            _ => panic!("Illegal Merge Access")
        }
    }
    
    fn split_access(&mut self, other: &Self) {
        match (self, other) {
            (ResourceAccess::Shared(0), ResourceAccess::Shared(0)) => (),
            (ResourceAccess::Shared(n), ResourceAccess::Shared(m)) => *n = n.checked_sub(*m).expect("Tried Splitting Access beyond current Accesses"),
            (lhs @ &mut ResourceAccess::Unique, ResourceAccess::Unique) => *lhs = ResourceAccess::Shared(0),
            
            _ => panic!("Illegal Splitting Access"),
        }
    }
    
    fn access<'a>(&self, resource: &'a Self::StoredResource) -> Self::AccessResult<'a, Self::Resource> {
        match self {
            ResourceAccess::Shared(0) => panic!("Illegal Access"),
            ResourceAccess::Shared(_) => AccessResult::Shared(unsafe { resource.as_ref() }),
            ResourceAccess::Unique => AccessResult::Unique(unsafe { resource.as_mut() }),
            ResourceAccess::Replace => panic!("Illegal Access"),
        }
    }
    
    fn remove<'a>(&self, resource: Self::StoredResource) -> Self::AccessResult<'a, Self::StoredResource> {
        if self.can_remove() {
            AccessResult::Taken(resource)
        } else {
            panic!("Illegal Removal")
        }
    }

    fn insert<'a>(&self, _resource: &'a Self::StoredResource) {
        if !self.can_insert() {
            panic!("Illegal Insert")
        }
    }
}