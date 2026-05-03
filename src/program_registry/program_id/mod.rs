use std::any::TypeId;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ProgramId {
    Label(String),
    TypeId(TypeId)
}