use aion_state::prelude::Accessor;

use crate::prelude::{AccessResult, StoredProgram};

#[derive(Debug, PartialEq)]
pub enum ProgramAccess {
    Shared(usize),
    Replace
}

impl Accessor for ProgramAccess {
    type AccessResult<'a, T> = AccessResult<'a, T> where T: 'a;
    type StoredResource = StoredProgram;
    type Resource = StoredProgram;
    
    fn can_access(&self, other: &Self) -> bool {
        match (self, other) {
            (ProgramAccess::Replace, _) => true,
            (ProgramAccess::Shared(0), _) => true,
            (ProgramAccess::Shared(_), ProgramAccess::Shared(_)) => true,
            (ProgramAccess::Shared(_), ProgramAccess::Replace) => false,
        }
    }
    
    fn can_insert(&self) -> bool {
        *self == ProgramAccess::Replace
    }
    
    fn can_remove(&self) -> bool {
        *self == ProgramAccess::Replace
    }
    
    fn is_active(&self) -> bool {
        matches!(self, ProgramAccess::Shared(_))
    }
    
    fn merge_access(&mut self, other: Self) {
        match (self, other) {
            (ProgramAccess::Shared(n), ProgramAccess::Shared(m)) => *n += m,
            (lhs @ ProgramAccess::Replace, rhs @ _) => *lhs = rhs,
            _ => panic!("Illegal Merging Access")
        }
    }
    
    fn split_access(&mut self, other: &Self) {
        match (self, other) {
            (ProgramAccess::Shared(0), ProgramAccess::Shared(0)) => (),
            (ProgramAccess::Shared(0), ProgramAccess::Shared(_)) => panic!("Tried Splitting Access beyond current Accesses"),
            (ProgramAccess::Shared(n), ProgramAccess::Shared(m)) => *n = n.checked_sub(*m).expect("Tried Splitting Access beyond current Accesses"),
            _ => panic!("Illegal Splitting Access"),
        }
    }
    
    fn access<'a>(&self, resource: &'a Self::StoredResource) -> Self::AccessResult<'a, Self::StoredResource> {
        match self {
            ProgramAccess::Shared(0) => panic!("Illegal Access"),
            ProgramAccess::Shared(_) => AccessResult::Shared(resource),
            ProgramAccess::Replace => panic!("Illegal Access"),
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