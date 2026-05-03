use std::any::TypeId;

#[derive(PartialEq, Eq, Hash)]
pub enum UserId {
    Label(String),
    TypeId(TypeId)
}