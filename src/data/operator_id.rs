#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OperatorId {
    pub id_type: OperatorIdType,
    pub operator_id: [u8; 20],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperatorIdType {
    OperatorId,

    Unknown(u8),
}

impl From<u8> for OperatorIdType {
    fn from(value: u8) -> Self {
        // 0: Operator ID
        // 1-200: Reserved
        // 201-255: Available for private use
        match value {
            0 => OperatorIdType::OperatorId,
            1.. => OperatorIdType::Unknown(value),
        }
    }
}

impl Into<u8> for OperatorIdType {
    fn into(self) -> u8 {
        match self {
            OperatorIdType::OperatorId => 0,
            OperatorIdType::Unknown(value) => value,
        }
    }
}
