use std::any::TypeId;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ResourceId {
    Label(String),
    TypeId(TypeId)
}