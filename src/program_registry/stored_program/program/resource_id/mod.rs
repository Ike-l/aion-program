use std::any::TypeId;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ResourceId {
    Label(String),
    TypeId(TypeId)
}