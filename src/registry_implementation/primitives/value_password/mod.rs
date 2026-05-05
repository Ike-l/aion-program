#[derive(Debug, PartialEq, Clone)]
pub struct ValuePassword {
    password: u64
}

impl From<u64> for ValuePassword {
    fn from(value: u64) -> Self {
        Self { password: value }
    }
}