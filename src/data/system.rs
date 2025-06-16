use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct System {
    pub classification_type: ClassificationType,
    pub operator_location_type: OperatorLocationType,
    pub operator_latidute: f32,
    pub operator_longitude: f32,
    pub area_count: u16,
    pub area_radius: f32,
    pub area_ceiling: f32,
    pub area_floor: f32,
    pub operator_altitude: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClassificationType {
    Undeclared = 0,
    EuropeanUnion = 1,
}

impl From<u8> for ClassificationType {
    fn from(value: u8) -> Self {
        match value {
            0 => ClassificationType::Undeclared,
            1 => ClassificationType::EuropeanUnion,

            // reserved classifications
            2.. => ClassificationType::Undeclared,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperatorLocationType {
    TakeOff = 0,
    Dynamic = 1,
    Fixed = 2,
}

impl From<u8> for OperatorLocationType {
    fn from(value: u8) -> Self {
        match value {
            0 => OperatorLocationType::TakeOff,
            1 => OperatorLocationType::Dynamic,
            2 => OperatorLocationType::Fixed,

            3.. => OperatorLocationType::TakeOff,
        }
    }
}
