use std::any::TypeId;

#[derive(PartialEq)]
pub enum UserId {
    Label(String),
    TypeId(TypeId)
}