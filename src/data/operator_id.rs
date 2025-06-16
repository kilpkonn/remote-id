#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OperatorId {
    pub id_type: OperatorIdType,
    pub operator_id: [u8; 20],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperatorIdType {
    OperatorId,

    Unknown,
}

impl From<u8> for OperatorIdType {
    fn from(value: u8) -> Self {
        match value {
            0 => OperatorIdType::OperatorId,
            1.. => OperatorIdType::Unknown,
        }
    }
}

impl Into<u8> for OperatorIdType {
    fn into(self) -> u8 {
        match self {
            OperatorIdType::OperatorId => 0,
            OperatorIdType::Unknown => 1,
        }
    }
}
