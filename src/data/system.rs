use chrono::{DateTime, Utc};

pub const MESSAGE_TYPE: u8 = 4;

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
    pub ua_classification: UaClassification,
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

#[derive(Debug, Clone, PartialEq)]
pub struct UaClassification {
    pub category: UaCategory,
    pub class: UaClass,
}

impl UaClassification {
    pub const fn undefined() -> Self {
        Self {
            category: UaCategory::Undefined,
            class: UaClass::Undefined,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UaCategory {
    Undefined,
    Open,
    Specific,
    Certified,
}

impl From<u8> for UaCategory {
    fn from(value: u8) -> Self {
        match value {
            0 => UaCategory::Undefined,
            1 => UaCategory::Open,
            2 => UaCategory::Specific,
            3 => UaCategory::Certified,

            _ => UaCategory::Undefined,
        }
    }
}

impl Into<u8> for UaCategory {
    fn into(self) -> u8 {
        match self {
            UaCategory::Undefined => 0,
            UaCategory::Open => 1,
            UaCategory::Specific => 2,
            UaCategory::Certified => 3,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UaClass {
    Undefined,
    Class0,
    Class1,
    Class2,
    Class3,
    Class4,
    Class5,
    Class6,
}

impl From<u8> for UaClass {
    fn from(value: u8) -> Self {
        match value {
            0 => UaClass::Undefined,
            1 => UaClass::Class0,
            2 => UaClass::Class1,
            3 => UaClass::Class2,
            4 => UaClass::Class3,
            5 => UaClass::Class4,
            6 => UaClass::Class5,
            7 => UaClass::Class6,

            _ => UaClass::Undefined,
        }
    }
}

impl Into<u8> for UaClass {
    fn into(self) -> u8 {
        match self {
            UaClass::Undefined => 0,
            UaClass::Class0 => 1,
            UaClass::Class1 => 2,
            UaClass::Class2 => 3,
            UaClass::Class3 => 4,
            UaClass::Class4 => 5,
            UaClass::Class5 => 6,
            UaClass::Class6 => 7,
        }
    }
}
