use core::time::Duration;

use basic_id::{BasicId, IdType, UAType};
use chrono::DateTime;
use location::{
    HeightType, HorizontalAccuracy, Location, OperationalStatus, SpeedAccuracy, VerticalAccuracy,
};
use operator_id::{OperatorId, OperatorIdType};
use system::{ClassificationType, OperatorLocationType, System};

extern crate std;
use crate::data::system::{UaCategory, UaClass, UaClassification};
use crate::{data::*, get_bits, get_bytes};
use crate::{MAX_ID_BYTE_SIZE, OPEN_DRONE_ID_AD_CODE};

use super::{copy_to_id, MessageType};

pub fn from_service_data(data: &[u8]) -> Option<RemoteIDMessage> {
    let first_byte = data[0];
    if first_byte != OPEN_DRONE_ID_AD_CODE {
        // all RemoteID Messages start with this byte?
        return None;
    }

    let _message_counter = data[1];

    from_message_buffer(&data[2..])
}

pub fn from_message_buffer(data: &[u8]) -> Option<RemoteIDMessage> {
    // protocol version, reserved for private use
    let _version = get_bits!(data[0], 3..0);

    match MessageType::from(get_bits!(data[0], 7..4)) {
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
    let id_type = IdType::from(get_bits!(buffer[1], 7..4));
    let ua_type = UAType::from(get_bits!(buffer[1], 3..0));

    let uas_id = copy_to_id(&get_bytes!(buffer, 2, crate::MAX_ID_BYTE_SIZE));

    Some(RemoteIDMessage::BasicID(BasicId {
        id_type,
        ua_type,
        uas_id,
    }))
}

fn parse_operator_id(buffer: &[u8]) -> Option<RemoteIDMessage> {
    // Operator ID Type
    let id_type = OperatorIdType::from(buffer[1]);

    // Operator ID
    let operator_id = copy_to_id(get_bytes!(buffer, 2, MAX_ID_BYTE_SIZE));

    let _reserved = get_bytes!(buffer, MAX_ID_BYTE_SIZE + 2, 3);

    Some(RemoteIDMessage::OperatorId(OperatorId {
        id_type,
        operator_id,
    }))
}

fn parse_system(buffer: &[u8]) -> Option<RemoteIDMessage> {
    let flags = buffer[1];
    // Reserved: Bits [7..5]
    let _reserved = get_bits!(flags, 7..5);
    // Classification Type: Bits [4..2]
    let classification_type = ClassificationType::from(get_bits!(flags, 4..2));
    // Operator Location/Altitude source type: Bits [1–0]
    let operator_location_type = OperatorLocationType::from(get_bits!(flags, 1..0));

    // Operator Latitude
    //    Latitude of Remote Pilot
    let operator_latidute =
        u32::from_le_bytes(get_bytes!(buffer, 2, 4)) as f32 / f32::powf(10., 7.);

    // Operator Longitude
    //   Longitude of Remote Pilot
    let operator_longitude =
        u32::from_le_bytes(get_bytes!(buffer, 6, 4)) as f32 / f32::powf(10., 7.);

    // Area Count
    //   Number of aircraft in Area, group or formation (default 1)
    let area_count = u16::from_le_bytes(get_bytes!(buffer, 10, 2));

    // Area Radius
    //   Radius of cylindrical area of group or formation * 10 m (default 0)
    //   centered on Location/Vector Message position
    let area_radius = get_bytes!(buffer, 12, 1) as f32 * 10.;

    // Area Ceiling
    //   Group operations ceiling WGS-84 HAE (Altitude + 1000 m)/0.5
    let area_ceiling = u16::from_le_bytes(get_bytes!(buffer, 13, 2)) as f32 / 2. - 1000.;

    // Area Floor
    //   Group operations floor WGS-84 HAE (Altitude + 1000 m)/0.5
    let area_floor = u16::from_be_bytes(get_bytes!(buffer, 15, 2)) as f32 / 2. - 1000.;

    // TODO UA Classification
    let ua_classification = if classification_type == ClassificationType::EuropeanUnion {
        UaClassification {
            category: UaCategory::from(get_bits!(buffer[16], 4..7)),
            class: UaClass::from(get_bits!(buffer[16], 0..3)),
        }
    } else {
        UaClassification::undefined()
    };

    // Operator Altitude
    let operator_altitude = u16::from_le_bytes(get_bytes!(buffer, 18, 2)) as f32 / 2. - 1000.;

    // Timestamp
    let unix_secs = u32::from_le_bytes(get_bytes!(buffer, 20, 4));
    let timestamp = DateTime::from_timestamp(unix_secs as i64 + 1546300800, 0)?;

    // Reserved
    let _reserved = get_bytes!(buffer, 24, 1);

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
        ua_classification,
        timestamp,
    }))
}

fn parse_location(buffer: &[u8]) -> Option<RemoteIDMessage> {
    // Status, Flags
    let status_flags = get_bytes!(buffer, 1, 1);

    let operational_status = OperationalStatus::from(get_bits!(status_flags, 7..4));
    let _reserved = get_bits!(status_flags, 3..3);
    let height_type = HeightType::from(get_bits!(status_flags, 2..2));
    let ew_direction_segment = get_bits!(status_flags, 1..1);
    let speed_multiplier = get_bits!(status_flags, 0..0);

    // Track Direction
    let track_direction = get_bytes!(buffer, 2, 1);
    let track_direction = if ew_direction_segment > 0 {
        track_direction as u16 + 180
    } else {
        ew_direction_segment as u16
    };

    // Speed
    let speed = get_bytes!(buffer, 3, 1);
    let speed = if speed_multiplier == 0 {
        speed as f32 * 0.25
    } else {
        (speed as f32 + 255. * 0.25) * 0.75
    };

    // Vertical Speed
    let vertical_speed = get_bytes!(buffer, 4, 1);
    let vertical_speed = (vertical_speed as f32 * 0.5) as f32;

    // Latitude
    let latidute = u32::from_le_bytes(get_bytes!(buffer, 5, 4)) as f32 / f32::powf(10., 7.);

    // Longitude
    let longitude = u32::from_le_bytes(get_bytes!(buffer, 9, 4)) as f32 / f32::powf(10., 7.);

    // Pressure Altitude
    let pressure_altitude = u16::from_le_bytes(get_bytes!(buffer, 13, 2)) as f32 / 2.0 - 1000.;

    // Geodetic Altitude
    let geodetic_altitude = u16::from_le_bytes(get_bytes!(buffer, 15, 2)) as f32 / 2.0 - 1000.;

    // Geodetic Altitude
    let height = u16::from_le_bytes(get_bytes!(buffer, 17, 2)) as f32 / 2.0 - 1000.;

    // Vertical / Horizontal Accuracy
    let accuracy = get_bytes!(buffer, 19, 1);
    let vertical_accuracy = VerticalAccuracy::from(get_bits!(accuracy, 7..4));
    let horizontal_accuracy = HorizontalAccuracy::from(get_bits!(accuracy, 3..0));

    let accuracy = get_bytes!(buffer, 20, 1);
    let baro_altitude_accuracy = VerticalAccuracy::from(get_bits!(accuracy, 7..4));
    let speed_accuracy = SpeedAccuracy::from(get_bits!(accuracy, 3..0));

    let timestamp = u16::from_le_bytes(get_bytes!(buffer, 21, 2)) as f32 / 10.;

    let timestamp_accuracy = get_bits!(get_bytes!(buffer, 23, 1), 3..0);
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
            operational_status: OperationalStatus::Airborne,
            speed: 0.,
            pressure_altitude: 190.5,
            geodetic_altitude: 210.0,
            vertical_speed: 0.,
            latidute: 49.874855,
            longitude: 8.912173,
            height: 0.,
            track_direction: 337,
            horizontal_accuracy: location::HorizontalAccuracy::LessThan_3_m,
            vertical_accuracy: location::VerticalAccuracy::LessThan_3_m,
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
            operational_status: OperationalStatus::Airborne,
            speed: 5.25,
            pressure_altitude: 201.5,
            geodetic_altitude: 218.0,
            vertical_speed: 0.,
            latidute: 49.875015,
            longitude: 8.912442,
            height: 11.0,
            track_direction: 0,
            horizontal_accuracy: location::HorizontalAccuracy::LessThan_3_m,
            vertical_accuracy: location::VerticalAccuracy::LessThan_3_m,
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
            0, 0, 0, 0, 0, 0,
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
            ua_classification: UaClassification {
                category: UaCategory::Open,
                class: UaClass::Undefined,
            },
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
