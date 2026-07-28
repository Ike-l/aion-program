use tracing::{Level, event};

use crate::prelude::BorrowType;

use aion_state::prelude::Accessor;

#[derive(Debug, PartialEq, Clone)]
pub enum Access {
    Shared(usize),
    Unique,
    Replace,
}

impl Access {
    fn borrow_type(&self) -> BorrowType {
        match self {
            Access::Replace |
            Access::Shared(0) => BorrowType::Instant,
            Access::Unique => BorrowType::Held,
            Access::Shared(_) => BorrowType::Held,
        }
    }
}

impl Accessor for Access {
    fn accepts_incoming(&self, incoming_access: &Self) -> bool {
        event!(Level::TRACE, "Access Accepts Incoming");

        match (self.borrow_type(), incoming_access.borrow_type()) {
            (BorrowType::Held, BorrowType::Held) => {
                match (self, incoming_access) {
                    (Access::Shared(_), Access::Shared(_)) => true,
                    _ => false
                }
            },
            (BorrowType::Held, BorrowType::Instant) => !(*incoming_access == Access::Replace || *self == Access::Unique),
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
    
    fn merge(
        &mut self,
        incoming_access: Self
    ) {
        event!(Level::TRACE, "Access Merge");

        if self.borrow_type() == BorrowType::Instant {
            *self = incoming_access;
            return
        }

        assert_eq!(self.borrow_type(), BorrowType::Held);

        if incoming_access.borrow_type() == BorrowType::Instant {
            assert_ne!(incoming_access, Access::Replace, "Tried replacing a held borrow");

            return;
        }

        assert_eq!(incoming_access.borrow_type(), BorrowType::Held);

        match (self.borrow_type(), incoming_access.borrow_type()) {
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
            (Access::Shared(n), Access::Shared(m)) => *n = n.saturating_sub(*m),     
            _ => ()
        }  
    }
    
    fn acquire<'a, V: aion_state::prelude::StoredValueTrait, R: aion_state::prelude::AccessorResult<'a, V::Value>>(
        &self, 
        stored_value: &'a mut V
    ) -> R {
        event!(Level::TRACE, "Access Acquire");
        
        match self {
            Access::Shared(0) => unreachable!(),
            Access::Shared(_) => R::new_shared(stored_value.as_shared()),
            Access::Unique => R::new_unique(stored_value.as_unique()),
            Access::Replace => unreachable!(),
        }
    }
}