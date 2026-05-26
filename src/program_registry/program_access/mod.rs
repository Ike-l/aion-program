use std::{fmt::Display, sync::Arc};

use tracing::{Level, event};

use crate::prelude::{StoredProgramTrait, AccessResult, BorrowType, Program, StoredProgram};

use aion_state::prelude::Accessor;

#[derive(Debug, PartialEq, Clone)]
pub enum ProgramAccess {
    Shared(usize),
    Replace,
}

impl Display for ProgramAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramAccess::Shared(number) => write!(f, "Shared with {number}"),
            ProgramAccess::Replace => write!(f, "Replace"),
        }
    }
}

impl ProgramAccess {
    fn to_borrow_type(&self) -> BorrowType {
        match self {
            ProgramAccess::Shared(0) |
            ProgramAccess::Replace => BorrowType::Instant,
            ProgramAccess::Shared(_) => BorrowType::Held
        }
    }
}

impl Accessor for ProgramAccess {
    type StoredValue = StoredProgram;
    type Value = Arc<Program>;

    type AccessResult<'a> = AccessResult<'a, Self::Value>;

    fn accepts_incoming(&self, incoming_access: &Self) -> bool {
        event!(Level::TRACE, "Access Accepts Incoming");

        match (self.to_borrow_type(), incoming_access.to_borrow_type()) {
            (BorrowType::Held, BorrowType::Held) => {
                match (self, incoming_access) {
                    (ProgramAccess::Shared(_), ProgramAccess::Shared(_)) => true,
                    _ => false
                }
            },
            (BorrowType::Held, BorrowType::Instant) => *incoming_access != ProgramAccess::Replace,
            (BorrowType::Instant, _) => true,
        }
    }

    fn can_insert_resource(&self) -> bool {
        event!(Level::TRACE, "Access Can Insert Resource");

        *self == ProgramAccess::Replace
    }

    fn can_remove_resource(&self) -> bool {
        event!(Level::TRACE, "Access Can Remove Resource");

        *self == ProgramAccess::Replace
    }

    fn acquire<'a>(
        &self, 
        stored_value: &'a mut Self::StoredValue
    ) -> Self::AccessResult<'a> {
        event!(Level::TRACE, "Access Acquire");

        match self {
            ProgramAccess::Shared(_) => AccessResult::Shared(stored_value.get()),
            ProgramAccess::Replace => unreachable!(),
        }
    }

    fn merge(
        &mut self,
        incoming_access: Self
    ) {
        event!(Level::TRACE, "Access Merge");

        if self.to_borrow_type() == BorrowType::Instant {
            *self = incoming_access;
            return
        }

        assert_eq!(self.to_borrow_type(), BorrowType::Held);

        if incoming_access.to_borrow_type() == BorrowType::Instant {
            assert_ne!(incoming_access, ProgramAccess::Replace, "Tried replacing a held borrow");

            return;
        }

        assert_eq!(incoming_access.to_borrow_type(), BorrowType::Held);

        match (self.to_borrow_type(), incoming_access.to_borrow_type()) {
            (BorrowType::Held, BorrowType::Held) => {
                match (self, incoming_access) {
                    (ProgramAccess::Shared(n), ProgramAccess::Shared(m)) => *n += m,
                    _ => panic!("Tried merging unique held accesses")
                }
            },
            _ => unreachable!()
        }
    }

    fn release(
        &mut self,
        other: &Self
    ) {
        event!(Level::TRACE, "Access Release");

        match (self, other) {
            (ProgramAccess::Shared(n), ProgramAccess::Shared(m)) => *n -= m,     
            _ => ()
        }  
    }

    fn insert(
        &self,
        value: Self::Value
    ) -> Self::StoredValue {
        event!(Level::TRACE, "Access Insert");

        <Self::StoredValue as StoredProgramTrait>::new(value)
    }

    fn remove(
        &self,
        stored_value: Self::StoredValue
    ) -> Self::StoredValue {
        event!(Level::TRACE, "Access Remove");

        stored_value
    }
}