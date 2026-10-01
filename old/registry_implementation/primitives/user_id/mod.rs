use std::any::TypeId;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum UserId {
    Label(String),
    TypeId(TypeId)
}