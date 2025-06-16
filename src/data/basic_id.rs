use crate::MAX_ID_BYTE_SIZE;

pub const MESSAGE_TYPE: u8 = 0;

#[derive(Debug, Clone, PartialEq)]
pub struct BasicId {
    pub id_type: IdType,
    pub ua_type: UAType,
    pub uas_id: [u8; MAX_ID_BYTE_SIZE],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IdType {
    None = 0,
    /// ANSI/CTA-2063-A
    SerialNumber = 1,
    CaaRegistrationId = 2,
    UtmAssignedId = 3,
    SpecificSessionId = 4,
}

impl From<u8> for IdType {
    fn from(value: u8) -> Self {
        match value {
            0 => IdType::None,
            1 => IdType::SerialNumber,
            2 => IdType::CaaRegistrationId,
            3 => IdType::UtmAssignedId,
            4 => IdType::SpecificSessionId,

            _ => IdType::None,
        }
    }
}

impl Into<u8> for IdType {
    fn into(self) -> u8 {
        match self {
            IdType::None => 0,
            IdType::SerialNumber => 1,
            IdType::CaaRegistrationId => 2,
            IdType::UtmAssignedId => 3,
            IdType::SpecificSessionId => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UAType {
    None = 0,
    Aeroplane = 1,
    HelicopterOrMultirotor = 2,
    Gyroplane = 3,
    HybridLift = 4,
    Ornithopter = 5,
    Glider = 6,
    Kite = 7,
    FreeBalloon = 8,
    CaptiveBalloon = 9,
    Airship = 10,
    FreeFallParachute = 11,
    Rocket = 12,
    TetheredPoweredAircraft = 13,
    GroundObstacle = 14,
    Other = 15,
}

impl From<u8> for UAType {
    fn from(value: u8) -> Self {
        match value {
            0 => UAType::None,
            1 => UAType::Aeroplane,
            2 => UAType::HelicopterOrMultirotor,
            3 => UAType::Gyroplane,
            4 => UAType::HybridLift,
            5 => UAType::Ornithopter,
            6 => UAType::Glider,
            7 => UAType::Kite,
            8 => UAType::FreeBalloon,
            9 => UAType::CaptiveBalloon,
            10 => UAType::Airship,
            11 => UAType::FreeFallParachute,
            12 => UAType::Rocket,
            13 => UAType::TetheredPoweredAircraft,
            14 => UAType::GroundObstacle,
            _ => UAType::Other,
        }
    }
}

impl Into<u8> for UAType {
    fn into(self) -> u8 {
        match self {
            UAType::Aeroplane => 1,
            UAType::HelicopterOrMultirotor => 2,
            UAType::Gyroplane => 3,
            UAType::HybridLift => 4,
            UAType::Ornithopter => 5,
            UAType::Glider => 6,
            UAType::Kite => 7,
            UAType::FreeBalloon => 8,
            UAType::CaptiveBalloon => 9,
            UAType::Airship => 10,
            UAType::FreeFallParachute => 11,
            UAType::Rocket => 12,
            UAType::TetheredPoweredAircraft => 13,
            UAType::GroundObstacle => 14,
            UAType::Other => 15,

            UAType::None => 0,
        }
    }
}
