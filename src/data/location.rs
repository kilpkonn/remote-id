use core::{time::Duration, u8};

pub const MESSAGE_TYPE: u8 = 1;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VerticalAccuracy {
    /// >=150 m or Unknown
    Unknown,
    LessThan_150_m,
    LessThan_45_m,
    LessThan_25_m,
    LessThan_10_m,
    LessThan_3_m,
    LessThan_1_m,
}

impl From<u8> for VerticalAccuracy {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::LessThan_150_m,
            2 => Self::LessThan_45_m,
            3 => Self::LessThan_25_m,
            4 => Self::LessThan_10_m,
            5 => Self::LessThan_3_m,
            6 => Self::LessThan_1_m,
            7..=u8::MAX => Self::Unknown,
        }
    }
}

impl Into<u8> for VerticalAccuracy {
    fn into(self) -> u8 {
        match self {
            Self::Unknown => 0,
            Self::LessThan_150_m => 1,
            Self::LessThan_45_m => 2,
            Self::LessThan_25_m => 3,
            Self::LessThan_10_m => 4,
            Self::LessThan_3_m => 5,
            Self::LessThan_1_m => 6,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HorizontalAccuracy {
    /// >= 18.52 km (10 NM) or Unknown
    Unknown,
    /// <18.52 km
    LessThan_10_NM,
    /// <7.408 km
    LessThan_4_NM,
    /// <3.704 km
    LessThan_2_NM,
    /// <1852 m
    LessThan_1_NM,
    /// <926 m
    LessThan_half_NM,
    /// <555.6 m
    LessThan_third_NM,
    /// <185.2 m
    LessThan_tenth_NM,
    /// <92.6 m
    LessThan_twentieth_NM,
    /// <30 m
    LessThan_30_m,
    ///  <10 m
    LessThan_10_m,
    ///  <3 m
    LessThan_3_m,
    ///  <1 m
    LessThan_1_m,
}

impl From<u8> for HorizontalAccuracy {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::LessThan_10_NM,
            2 => Self::LessThan_4_NM,
            3 => Self::LessThan_2_NM,
            4 => Self::LessThan_1_NM,
            5 => Self::LessThan_half_NM,
            6 => Self::LessThan_third_NM,
            7 => Self::LessThan_tenth_NM,
            8 => Self::LessThan_twentieth_NM,
            9 => Self::LessThan_30_m,
            10 => Self::LessThan_10_m,
            11 => Self::LessThan_3_m,
            12 => Self::LessThan_1_m,
            13..=u8::MAX => Self::Unknown,
        }
    }
}

impl Into<u8> for HorizontalAccuracy {
    fn into(self) -> u8 {
        match self {
            Self::Unknown => 0,
            Self::LessThan_10_NM => 1,
            Self::LessThan_4_NM => 2,
            Self::LessThan_2_NM => 3,
            Self::LessThan_1_NM => 4,
            Self::LessThan_half_NM => 5,
            Self::LessThan_third_NM => 6,
            Self::LessThan_tenth_NM => 7,
            Self::LessThan_twentieth_NM => 8,
            Self::LessThan_30_m => 9,
            Self::LessThan_10_m => 10,
            Self::LessThan_3_m => 11,
            Self::LessThan_1_m => 12,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SpeedAccuracy {
    /// >=10 m/s or Unknown
    Unknown,
    /// <10 m/s
    LessThan_10_mps,
    /// <3 m/s
    LessThan_3_mps,
    /// <1 m/s
    LessThan_1_mps,
    /// <0.3 m/s
    LessThan_third_mps,
}

impl From<u8> for SpeedAccuracy {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::LessThan_10_mps,
            2 => Self::LessThan_3_mps,
            3 => Self::LessThan_1_mps,
            4 => Self::LessThan_third_mps,
            5..=u8::MAX => Self::Unknown,
        }
    }
}

impl Into<u8> for SpeedAccuracy {
    fn into(self) -> u8 {
        match self {
            Self::Unknown => 0,
            Self::LessThan_10_mps => 1,
            Self::LessThan_3_mps => 2,
            Self::LessThan_1_mps => 3,
            Self::LessThan_third_mps => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub operational_status: OperationalStatus,
    pub height_type: HeightType,
    // pub ew_direction_segment: EastWestDirectionSegment,
    /// Speed Multiplier enables speeds up to 254.25 m/s. Only use 1 when speed exceeds 63.75 m/s and add 63.75.
    // pub speed_multiplier: f32,
    pub speed: f32,
    pub vertical_speed: f32,
    pub pressure_altitude: f32,
    pub geodetic_altitude: f32,
    /// Direction expressed as the route course measured clockwise from true north.
    pub track_direction: u16,
    pub horizontal_accuracy: HorizontalAccuracy,
    pub vertical_accuracy: VerticalAccuracy,
    pub latidute: f32,
    pub longitude: f32,
    pub height: f32,
    pub baro_altitude_accuracy: VerticalAccuracy,
    pub speed_accuracy: SpeedAccuracy,
    pub timestamp: f32,
    pub timestamp_accuracy: Option<Duration>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperationalStatus {
    Undeclared,
    Ground,
    Airborne,
    Emergency,
    RemoteIdSystemFailure,
}

impl From<u8> for OperationalStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => OperationalStatus::Ground,
            2 => OperationalStatus::Airborne,
            3 => OperationalStatus::Emergency,
            4 => OperationalStatus::RemoteIdSystemFailure,

            _ => OperationalStatus::Undeclared,
        }
    }
}

impl Into<u8> for OperationalStatus {
    fn into(self) -> u8 {
        match self {
            OperationalStatus::Ground => 1,
            OperationalStatus::Airborne => 2,
            OperationalStatus::Emergency => 3,
            OperationalStatus::RemoteIdSystemFailure => 4,

            OperationalStatus::Undeclared => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HeightType {
    AboveTakeoff,
    AboveGroundLevel,
}

impl From<u8> for HeightType {
    fn from(value: u8) -> Self {
        match value {
            0 => HeightType::AboveTakeoff,
            1 => HeightType::AboveGroundLevel,

            _ => HeightType::AboveTakeoff,
        }
    }
}

impl Into<u8> for HeightType {
    fn into(self) -> u8 {
        match self {
            HeightType::AboveTakeoff => 0,
            HeightType::AboveGroundLevel => 1,
        }
    }
}

// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum EastWestDirectionSegment {
//     LowerThan180 = 0,
//     GreaterOrEqual180 = 1,
// }

// impl From<u8> for EastWestDirectionSegment {
//     fn from(value: u8) -> Self {
//         match value {
//             0 => EastWestDirectionSegment::LowerThan180,
//             1 => EastWestDirectionSegment::GreaterOrEqual180,

//             _ => EastWestDirectionSegment::LowerThan180,
//         }
//     }
// }

// impl Into<u8> for EastWestDirectionSegment {
//     fn into(self) -> u8 {
//         match self {
//             EastWestDirectionSegment::LowerThan180 => 0,
//             EastWestDirectionSegment::GreaterOrEqual180 => 1,
//         }
//     }
// }
