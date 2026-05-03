use tracing::{Level, event};

use crate::prelude::{AccessResult, BorrowType};

pub mod access_result;
pub mod borrow_type;

#[derive(Debug, PartialEq)]
pub enum Access {
    Shared(usize),
    Unique,
    Replace,
}

impl Access {
    fn to_borrow_type(&self) -> BorrowType {
        match self {
            Access::Shared(0) |
            Access::Replace => BorrowType::Instant,
            Access::Shared(_) |
            Access::Unique => BorrowType::Held,
        }
    }
}

impl<StoredResource, Resource> aion_state::prelude::Accessor for Access {
    type StoredValue = StoredResource;
    type Value = Resource;

    type AccessResult<'a> = AccessResult<'a>;

    fn accepts_incoming(&self, incoming_access: &Self) -> bool {
        event!(Level::TRACE, "Access Accepts Incoming");

        match (self.to_borrow_type(), incoming_access.to_borrow_type()) {
            (BorrowType::Held, BorrowType::Held) => {
                match (self, incoming_access) {
                    (Access::Shared(_), Access::Shared(_)) => true,
                    _ => false
                }
            },
            (BorrowType::Held, BorrowType::Instant) => *incoming_access != Access::Replace,
            (BorrowType::Instant, _) => true,
        }
    }

    fn can_insert_resource(&self) -> bool {
        event!(Level::TRACE, "Access Can Insert Resource");

        *self == Access::Replace
    }

    fn can_remove_resource(&self) -> bool {
        event!(Level::TRACE, "Access Can Remove Resource");

        *self == Access::Replace
    }

    fn acquire<'a>(
        &self, 
        stored_value: &'a mut Self::StoredValue
    ) -> Self::AccessResult<'a> {
        event!(Level::TRACE, "Access Acquire");

        match self {
            Access::Shared(_) => AccessResult::Shared(stored_value.get()),
            Access::Unique => AccessResult::Unique(stored_value.get_mut()),
            Access::Replace => unreachable!(),
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
            assert_ne!(incoming_access, Access::Replace, "Tried replacing a held borrow");

            return;
        }

        assert_eq!(incoming_access.to_borrow_type(), BorrowType::Held);

        match (self.to_borrow_type(), incoming_access.to_borrow_type()) {
            (BorrowType::Held, BorrowType::Held) => {
                match (self, incoming_access) {
                    (Access::Shared(n), Access::Shared(m)) => *n += m,
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
            (Access::Shared(n), Access::Shared(m)) => *n -= m,     
            _ => ()
        }  
    }

    fn insert<'a>(
        &self,
        value: Self::Value
    ) -> Self::StoredValue {
        event!(Level::TRACE, "Access Insert");

        Self::StoredValue::new(value)
    }

    fn remove<'a>(
        &self,
        stored_value: Self::StoredValue
    ) -> Self::StoredValue {
        event!(Level::TRACE, "Access Remove");

        stored_value
    }
}