#[expect(unused)]
mod gen_types;
pub use crate::gen_types::{Commands, Types};
mod type_methods;

mod error;
pub use error::{Error, Result};
mod ser;
pub use ser::{to_bytes, Serializer};
mod de;
pub use de::{from_bytes, Deserializer};

pub mod comms {
    use crate::{to_bytes, type_methods::Hcp};
    use serde::{Deserialize, Serialize};
    use serialport::SerialPort;
    use std::time::Duration;

    pub struct Serial {
        port: Box<dyn SerialPort>,
    }

    impl Serial {
        pub fn new(
            path: &str,
            baud_rate: u32,
            timeout: Duration,
        ) -> Result<Self, serialport::Error> {
            let port = serialport::new(path, baud_rate).timeout(timeout).open()?;

            Ok(Self { port })
        }

        fn send(&mut self, message: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
            self.port.write_all(&message[..])?;

            let mut buf = vec![0u8; 256];
            loop {
                match self.port.read(buf.as_mut_slice()) {
                    Ok(t) => return Ok(buf[..t].to_vec()),
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }

        // pub fn send_message<T>(&mut self, message: &T) -> Result<T, std::io::Error>
        // where
        //     T: Hcp + Serialize + Deserialize,
        // {
        //     let response = to_bytes(message)?
        // }
    }
}

pub mod proto {
    const CRC_TABLE: [u8; 256] = [
        0, 94, 188, 226, 97, 63, 221, 131, 194, 156, 126, 32, 163, 253, 31, 65, 157, 195, 33, 127,
        252, 162, 64, 30, 95, 1, 227, 189, 62, 96, 130, 220, 35, 125, 159, 193, 66, 28, 254, 160,
        225, 191, 93, 3, 128, 222, 60, 98, 190, 224, 2, 92, 223, 129, 99, 61, 124, 34, 192, 158,
        29, 67, 161, 255, 70, 24, 250, 164, 39, 121, 155, 197, 132, 218, 56, 102, 229, 187, 89, 7,
        219, 133, 103, 57, 186, 228, 6, 88, 25, 71, 165, 251, 120, 38, 196, 154, 101, 59, 217, 135,
        4, 90, 184, 230, 167, 249, 27, 69, 198, 152, 122, 36, 248, 166, 68, 26, 153, 199, 37, 123,
        58, 100, 134, 216, 91, 5, 231, 185, 140, 210, 48, 110, 237, 179, 81, 15, 78, 16, 242, 172,
        47, 113, 147, 205, 17, 79, 173, 243, 112, 46, 204, 146, 211, 141, 111, 49, 178, 236, 14,
        80, 175, 241, 19, 77, 206, 144, 114, 44, 109, 51, 209, 143, 12, 82, 176, 238, 50, 108, 142,
        208, 83, 13, 239, 177, 240, 174, 76, 18, 145, 207, 45, 115, 202, 148, 118, 40, 171, 245,
        23, 73, 8, 86, 180, 234, 105, 55, 213, 139, 87, 9, 235, 181, 54, 104, 138, 212, 149, 203,
        41, 119, 244, 170, 72, 22, 233, 183, 85, 11, 136, 214, 52, 106, 43, 117, 151, 201, 74, 20,
        246, 168, 116, 42, 200, 150, 21, 75, 169, 247, 182, 232, 10, 84, 215, 137, 107, 53,
    ];

    pub fn calc_crc(array: &[u8], start: usize) -> u8 {
        let mut crc = 0;
        for i in &array[start..] {
            crc = CRC_TABLE[(crc ^ i) as usize]
        }
        crc
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn serialize_setsoundtype() {
        let command = Commands::Sound::SetSoundType::inParams {
            soundType: Types::tSoundType::SOUND_CLICK,
        };
        assert_eq!(
            to_bytes(&command).unwrap(),
            vec![0x02, 0x81, 0x08, 0x00, 0x00, 0x90, 0xac, 0x02, 0x00, 0x01, 0x3e, 0x03]
        )
    }

    #[test]
    fn serialize_deviceid() {
        let command = Commands::DeviceInformation::GetDeviceIdentification::inParams {};
        assert_eq!(
            to_bytes(&command).unwrap(),
            vec![0x2, 0x16, 0x1, 0x0, 0x5f, 0x3]
        );
    }

    #[test]
    fn deserialize_deviceid() {
        let message = [
            0x2, 0x17, 0x8, 0x0, 0xa, 0x8, 0x51, 0xda, 0x6c, 0xa, 0x0, 0xd8, 0x3,
        ];
        let response: Commands::DeviceInformation::GetDeviceIdentification =
            from_bytes(&message).unwrap();
        let expected = Commands::DeviceInformation::GetDeviceIdentification::outParams {
            deviceTypeGroup: Types::tDeviceTypeGroup::DEVICE_TYPE_GROUP_MOWER,
            mowerDeviceType: Types::tMowerDeviceType::MOWER_DEVICE_TYPE_I,
            mowerSerialNo: 174905937,
            mowerVariantType: Types::tMowerVariantType::MOWER_VARIANT_TYPE_ORG,
        };

        assert_eq!(response, expected);
        println!("{:?}", response);
    }

    #[test]
    fn deserialize_setsoundtype() {
        let message = [
            0x2, 0x81, 0x8, 0x0, 0x0, 0x90, 0xad, 0x2, 0x0, 0x1, 0xb1, 0x3,
        ];
        let response: Commands::Sound::SetSoundType = from_bytes(&message).unwrap();

        let expected = Commands::Sound::SetSoundType::outParams {
            soundType: Types::tSoundType::SOUND_CLICK,
        };

        println!("{:?}", response);

        assert_eq!(response, expected);
    }

    #[test]
    fn deserialize_getsafetysupervisor() {
        let message = [
            0x2, 0x81, 0x1c, 0x0, 0x0, 0x91, 0x73, 0x16, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x52, 0x3,
        ];
        let response: Commands::SafetySupervisor::GetStatus = from_bytes(&message).unwrap();
        let expected = Commands::SafetySupervisor::GetStatus::outParams {
            stopButtonPressed: false,
            onOffSwitchInactive: false,
            lifted: false,
            upsideDown: false,
            tooMuchTilt: false,
            collision3s: false,
            tooFarOutsideBoundary: false,
            noLoopSignalWheels: false,
            pinCodeNeeded: false,
            twoSeperateActionsNeededBlade: false,
            twoSeperateActionsNeededWheels: false,
            warningSoundNeeded: true,
            chargingOngoing: false,
            noLoopSignalBlade: false,
            collisionIsActive: false,
            memNotValidated: false,
            blade10sLift: false,
            blade10sTilt: false,
            blade10sCollision: false,
            bladeUpSideDown: false,
            powerModeLedBroken: false,
        };
        assert_eq!(response, expected);
    }
}
