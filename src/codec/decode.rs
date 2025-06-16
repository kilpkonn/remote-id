use core::time::Duration;

use basic_id::{BasicId, IdType, UAType};
use chrono::DateTime;
use location::{
    HeightType, HorizontalAccuracy, Location, OperationalStatus, SpeedAccuracy, VerticalAccuracy,
};
use operator_id::{OperatorId, OperatorIdType};
use system::{ClassificationType, OperatorLocationType, System};

extern crate std;
use crate::{data::*, get_bits};
use crate::{MAX_ID_BYTE_SIZE, OPEN_DRONE_ID_AD_CODE};

use super::{copy_to_id, MessageType};

pub trait IntoF32LE {
    fn into_f32_le(self) -> Result<f32, InvalidSliceLengthError>;
}
impl IntoF32LE for &[u8] {
    fn into_f32_le(self) -> Result<f32, InvalidSliceLengthError> {
        if let Ok(b) = TryInto::<[u8; 4]>::try_into(self) {
            Ok(u32::from_le_bytes(b) as f32)
        } else if let Ok(b) = TryInto::<[u8; 2]>::try_into(self) {
            Ok(u16::from_le_bytes(b) as f32)
        } else {
            Err(InvalidSliceLengthError((4, self.len())))
        }
    }
}

pub struct InvalidSliceLengthError((usize, usize));
impl core::fmt::Debug for InvalidSliceLengthError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "Invalid Slice Length: Expected {}, Got {}",
            self.0 .0, self.0 .1
        ))?;

        Ok(())
    }
}

pub trait IntoU32LE {
    fn into_u32_le(self) -> Result<u32, InvalidSliceLengthError>;
}
impl IntoU32LE for &[u8] {
    fn into_u32_le(self) -> Result<u32, InvalidSliceLengthError> {
        if let Ok(b) = TryInto::<[u8; 4]>::try_into(self) {
            Ok(u32::from_le_bytes(b))
        } else {
            Err(InvalidSliceLengthError((4, self.len())))
        }
    }
}

pub trait IntoU16LE {
    fn into_u16_le(self) -> Result<u16, InvalidSliceLengthError>;
}
impl IntoU16LE for &[u8] {
    fn into_u16_le(self) -> Result<u16, InvalidSliceLengthError> {
        if let Ok(b) = TryInto::<[u8; 2]>::try_into(self) {
            Ok(u16::from_le_bytes(b))
        } else {
            Err(InvalidSliceLengthError((2, self.len())))
        }
    }
}

pub fn from_service_data(data: &[u8]) -> Option<RemoteIDMessage> {
    let first_byte = data[0];
    if first_byte != OPEN_DRONE_ID_AD_CODE {
        // all RemoteID Messages start with this byte?
        return None;
    }

    let _message_counter = data[1];

    let msg_header = data[2];
    let (hi_nibble, lo_nibble) = ((msg_header & 0xF0) >> 4, msg_header & 0x0F);
    // first four bits of header are the message type
    let msg_type = match hi_nibble {
        0 => MessageType::BasicId,
        1 => MessageType::Location,
        2 => MessageType::Auth,
        3 => MessageType::Selfid,
        4 => MessageType::System,
        5 => MessageType::OperatorId,
        0xF => MessageType::MessagePack,

        _ => MessageType::Invalid,
    };

    // protocol version, declared as unused in F3411-22a
    let _version = lo_nibble;

    let data = &data[3..];
    match msg_type {
        MessageType::BasicId => parse_basic_id(data),
        MessageType::Location => parse_location(data),
        MessageType::OperatorId => parse_operator_id(data),
        MessageType::System => parse_system(data),

        // we have no examples for these yet
        MessageType::Selfid => todo!(),
        MessageType::Auth => todo!(),
        MessageType::MessagePack => todo!(),

        MessageType::Invalid => None,
    }
}

fn parse_basic_id(buffer: &[u8]) -> Option<RemoteIDMessage> {
    let b = buffer[0];
    let (first_nibble, last_nibble) = ((b & 0xF0) >> 4, b & 0x0F);
    let id_type = IdType::from(first_nibble);
    let ua_type = UAType::from(last_nibble);

    let uas_id = copy_to_id(&buffer[1..(MAX_ID_BYTE_SIZE + 1)]);

    Some(RemoteIDMessage::BasicID(BasicId {
        id_type,
        ua_type,
        uas_id,
    }))
}

fn parse_operator_id(buffer: &[u8]) -> Option<RemoteIDMessage> {
    // Operator ID Type
    //      0: Operator ID
    //      1–200: Reserved
    //      201–255: Available for private use
    let b = buffer[0];
    let id_type = OperatorIdType::from(b);

    // Operator ID
    //      ASCII Text. If numeric values exist, they shall be
    //      expressed as a string of ASCII characters (padded with nulls)
    let operator_id = copy_to_id(&buffer[1..MAX_ID_BYTE_SIZE + 1]);

    let _reserved = &buffer[(MAX_ID_BYTE_SIZE + 1)..];

    Some(RemoteIDMessage::OperatorId(OperatorId {
        id_type,
        operator_id,
    }))
}

fn parse_system(buffer: &[u8]) -> Option<RemoteIDMessage> {
    // Flags
    let b = buffer[0];
    // Classification Type: Bits [4..2]
    let classification_type = ClassificationType::from(get_bits!(b, 2..4));
    // Operator Location/Altitude source type: Bits [1–0]
    let operator_location_type = OperatorLocationType::from(get_bits!(b, 0..1));

    // Operator Latitude
    //    Latitude of Remote Pilot
    let operator_latidute = buffer[1..5].into_f32_le().unwrap() / f32::powf(10., 7.);

    // Operator Longitude
    //   Longitude of Remote Pilot
    let operator_longitude = buffer[5..9].into_f32_le().unwrap() / f32::powf(10., 7.);

    // Area Count
    //   Number of aircraft in Area, group or formation (default 1)
    let area_count = buffer[9..11].into_u16_le().unwrap();

    // Area Radius
    //   Radius of cylindrical area of group or formation * 10 m (default 0)
    //   centered on Location/Vector Message position
    let area_radius = buffer[11] as f32 * 10.;

    // Area Ceiling
    //   Group operations ceiling WGS-84 HAE (Altitude + 1000 m)/0.5
    let area_ceiling = buffer[12..14].into_f32_le().unwrap() / 2. - 1000.;

    // Area Floor
    //   Group operations floor WGS-84 HAE (Altitude + 1000 m)/0.5
    let area_floor = buffer[14..16].into_f32_le().unwrap() / 2. - 1000.;

    // TODO UA Classification
    let _b = buffer[16];

    // Operator Altitude
    let operator_altitude = buffer[17..19].into_f32_le().unwrap() / 2. - 1000.;

    // Timestamp
    let unix_secs = buffer[19..23].into_u32_le().unwrap();
    let timestamp = DateTime::from_timestamp(unix_secs as i64 + 1546300800, 0)?;

    // Reserved
    let _reserved = &buffer[23..];

    Some(RemoteIDMessage::System(System {
        classification_type,
        operator_location_type,
        operator_latidute,
        operator_longitude,
        operator_altitude,
        area_count,
        area_radius,
        area_floor,
        area_ceiling,
        timestamp,
    }))
}

fn parse_location(buffer: &[u8]) -> Option<RemoteIDMessage> {
    // Status, Flags
    let b = buffer[0];

    let operational_status = OperationalStatus::from(get_bits!(b, 3..7));
    let height_type = HeightType::from(get_bits!(b, 2..2));
    let ew_direction_segment = get_bits!(b, 1..1);

    let speed_multiplier = get_bits!(b, 0..0);

    // Track Direction
    let track_direction = buffer[1];
    let track_direction = if ew_direction_segment > 0 {
        track_direction as u16 + 180
    } else {
        ew_direction_segment as u16
    };

    // Speed
    let speed = buffer[2];
    let speed = if speed_multiplier == 0 {
        speed as f32 * 0.25
    } else {
        (speed as f32 + 255. * 0.25) * 0.75
    };

    // Vertical Speed
    let vertical_speed = buffer[3];
    let vertical_speed = (vertical_speed as f32 * 0.5) as f32;

    // Latitude
    let latidute = buffer[4..8].into_f32_le().unwrap() / f32::powf(10., 7.);

    // Longitude
    let longitude = buffer[8..12].into_f32_le().unwrap() / f32::powf(10., 7.);

    // Pressure Altitude
    let pressure_altitude = buffer[12..14].into_f32_le().unwrap() / 2.0 - 1000.;

    // Geodetic Altitude
    let geodetic_altitude = buffer[14..16].into_f32_le().unwrap() / 2.0 - 1000.;

    // Geodetic Altitude
    let height = buffer[16..18].into_f32_le().unwrap() / 2.0 - 1000.;

    // Vertical / Horizontal Accuracy
    let b = buffer[18];
    let vertical_accuracy = VerticalAccuracy::from(b & 0b1111_0000);
    let horizontal_accuracy = HorizontalAccuracy::from(b & 0b1111);

    let b = buffer[19];
    let baro_altitude_accuracy = VerticalAccuracy::from(b & 0b1111_0000);
    let speed_accuracy = SpeedAccuracy::from(b & 0b1111);

    let timestamp = buffer[20..22].into_u16_le().unwrap() as f32 / 10.;

    let timestamp_accuracy = buffer[23] & 0b1111;
    let timestamp_accuracy = if timestamp_accuracy == 0 {
        None
    } else {
        Some(Duration::from_secs_f32(timestamp_accuracy as f32 * 0.1))
    };

    Some(RemoteIDMessage::Location(Location {
        height_type,
        operational_status,
        latidute,
        longitude,
        height,
        pressure_altitude,
        geodetic_altitude,
        track_direction,
        speed,
        vertical_speed,
        horizontal_accuracy,
        vertical_accuracy,
        baro_altitude_accuracy,
        speed_accuracy,
        timestamp,
        timestamp_accuracy,
    }))
}

#[cfg(test)]
mod test {
    extern crate std;

    use operator_id::{OperatorId, OperatorIdType};

    use super::*;

    #[test]
    fn decode_basic_id_1() {
        // DroneTag Mini
        let expected = RemoteIDMessage::BasicID(BasicId {
            id_type: IdType::SerialNumber,
            ua_type: UAType::None,
            uas_id: copy_to_id("10000000000000000009".as_bytes()),
        });

        let service_data = [
            13, 1, 2, 16, 49, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
            48, 57, 0, 0, 0,
        ];

        assert_eq!(expected, from_service_data(&service_data).unwrap());
    }

    #[test]
    fn decode_basic_id_2() {
        // DroneTag BS
        let expected = RemoteIDMessage::BasicID(BasicId {
            id_type: IdType::SerialNumber,
            ua_type: UAType::None,
            uas_id: copy_to_id("10000000000000000009".as_bytes()),
        });

        let service_data = [
            13, 1, 2, 16, 49, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
            48, 57, 0, 0, 0,
        ];

        assert_eq!(expected, from_service_data(&service_data).unwrap());
    }

    #[test]
    fn decode_location_1() {
        let expected = RemoteIDMessage::Location(Location {
            height_type: HeightType::AboveTakeoff,
            operational_status: OperationalStatus::RemoteIdSystemFailure,
            speed: 0.,
            pressure_altitude: 190.5,
            geodetic_altitude: 210.0,
            vertical_speed: 0.,
            latidute: 49.874855,
            longitude: 8.912173,
            height: 0.,
            track_direction: 337,
            horizontal_accuracy: location::HorizontalAccuracy::LessThan_3_m,
            vertical_accuracy: location::VerticalAccuracy::Unknown,
            baro_altitude_accuracy: location::VerticalAccuracy::Unknown,
            speed_accuracy: location::SpeedAccuracy::LessThan_third_mps,
            timestamp: 361.0,
            timestamp_accuracy: None,
        });

        let service_data = [
            13, 72, 18, 34, 157, 0, 0, 143, 76, 186, 29, 192, 227, 79, 5, 77, 9, 116, 9, 208, 7,
            91, 4, 26, 14, 0, 0,
        ];
        assert_eq!(expected, from_service_data(&service_data).unwrap());
    }

    #[test]
    fn decode_location_2() {
        let expected = RemoteIDMessage::Location(Location {
            height_type: HeightType::AboveTakeoff,
            operational_status: OperationalStatus::RemoteIdSystemFailure,
            speed: 5.25,
            pressure_altitude: 201.5,
            geodetic_altitude: 218.0,
            vertical_speed: 0.,
            latidute: 49.875015,
            longitude: 8.912442,
            height: 11.0,
            track_direction: 0,
            horizontal_accuracy: location::HorizontalAccuracy::LessThan_3_m,
            vertical_accuracy: location::VerticalAccuracy::Unknown,
            baro_altitude_accuracy: location::VerticalAccuracy::Unknown,
            speed_accuracy: location::SpeedAccuracy::LessThan_third_mps,
            timestamp: 886.0,
            timestamp_accuracy: None,
        });

        let service_data = [
            13, 85, 18, 32, 52, 21, 0, 188, 82, 186, 29, 69, 238, 79, 5, 99, 9, 132, 9, 230, 7, 91,
            4, 156, 34, 0, 0,
        ];
        assert_eq!(expected, from_service_data(&service_data).unwrap());
    }

    #[test]
    fn decode_location_3() {
        let service_data = [
            13, 1, 18, 32, 77, 2, 20, 128, 76, 186, 29, 200, 227, 79, 5, 77, 9, 116, 9, 208, 7, 91,
            4, 26, 14, 0, 0,
        ];
        std::dbg!(from_service_data(&service_data).unwrap());
    }

    #[test]
    fn decode_operator_id() {
        let expected = RemoteIDMessage::OperatorId(OperatorId {
            id_type: OperatorIdType::OperatorId,
            operator_id: copy_to_id("NULL".as_bytes()),
        });

        let service_data = [
            13, 3, 82, 0, 78, 85, 76, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(expected, from_service_data(&service_data).unwrap());
    }

    #[test]
    fn decode_operator_id_2() {
        let expected = RemoteIDMessage::OperatorId(OperatorId {
            id_type: OperatorIdType::OperatorId,
            operator_id: copy_to_id("FIN87astrdge12k8".as_bytes()),
        });

        let service_data = [
            13, 3, 82, 0, 70, 73, 78, 56, 55, 97, 115, 116, 114, 100, 103, 101, 49, 50, 107, 56, 0,
            0, 0, 0,
        ];
        assert_eq!(expected, from_service_data(&service_data).unwrap());
    }

    #[test]
    fn decode_system_1() {
        let expected = RemoteIDMessage::System(System {
            classification_type: ClassificationType::EuropeanUnion,
            operator_location_type: OperatorLocationType::TakeOff,
            operator_latidute: 49.874855,
            operator_longitude: 8.912173,
            operator_altitude: 210.,
            area_ceiling: -1000.,
            area_count: 1,
            area_floor: -1000.,
            area_radius: 250.,
            timestamp: DateTime::parse_from_rfc3339(&"2024-07-04T14:05:54Z")
                .unwrap()
                .to_utc(),
        });

        let service_data = [
            13, 3, 66, 4, 131, 76, 186, 29, 188, 227, 79, 5, 1, 0, 25, 0, 0, 0, 0, 16, 116, 9, 194,
            254, 91, 10, 0,
        ];
        assert_eq!(expected, from_service_data(&service_data).unwrap());
    }
}
