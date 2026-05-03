use aion_state::prelude::StorageTrait;

pub mod access_storage;
pub 

pub struct ProgramStorage;

impl StorageTrait for ProgramStorage {
    type Value;

    type ValueId;

    type S;

    type Access;

    type AS;

    type ReserverId;

    type RS;

    type OS;

    type WS;

    type BS;

    type CS;
}