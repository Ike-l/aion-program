use std::any::TypeId;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ProgramId {
    Label(String),
    TypeId(TypeId)
}