#![no_std]

pub mod codec;
pub mod data;

const MAX_ID_BYTE_SIZE: usize = 20;

// https://github.com/opendroneid/receiver-android/blob/a6359b6ee7c2b06c035137c8348cf979705624c3/Android/app/src/main/java/org/opendroneid/android/bluetooth/BluetoothScanner.java#L121
const OPEN_DRONE_ID_AD_CODE: u8 = 0x0D;

/// Remote ID Service Data Advertisement UUID
// The UUID is combined from the
//   - Service Data Object (SDO) UUID for Remote ID: "fffa"
//     (source: https://www.bluetooth.com/specifications/assigned-numbers/, Section 3.10 in the PDF)
//
//   - a base Bluetooth LE UUID:           0000____-0000-1000-8000-00805f9b34fb
pub const REMOTE_ID_SERVICE_UUID: u128 = 0x0000fffa_0000_1000_8000_00805f9b34fb;
