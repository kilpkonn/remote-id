pub const MESSAGE_TYPE: u8 = 0x3;

#[derive(Debug, Clone, PartialEq)]
pub struct SelfId {
    pub description: Description,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Description {
    Text([u8; 23]),
}
