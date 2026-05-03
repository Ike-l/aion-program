use std::any::TypeId;

pub enum ProgramId {
    Label(String),
    TypeId(TypeId)
}