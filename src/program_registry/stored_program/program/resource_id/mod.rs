use std::any::TypeId;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ResourceId {
    Label(String),
    TypeId(TypeId)
}