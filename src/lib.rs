mod gen_types;
pub use crate::gen_types::types;

pub mod comms {
    use std::time::Duration;

    use serialport::SerialPort;

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

        pub fn write(&mut self, output: &[u8]) -> Result<usize, std::io::Error> {
            self.port.write(&output[..])
        }

        pub fn read(&mut self) -> Result<(usize, Vec<u8>), std::io::Error> {
            let mut serial_buf: Vec<u8> = vec![0; 256];
            let bytes_written = self.port.read(serial_buf.as_mut_slice())?;
            Ok((bytes_written, serial_buf))
        }

        pub fn write_read(&mut self, output: &[u8]) -> Result<Vec<u8>, std::io::Error> {
            self.write(output)?;
            let (_, response) = self.read()?;

            Ok(response)
        }
    }
}

pub mod proto {
    use crate::gen_types::types::*;
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

    fn calc_crc(array: &Vec<u8>, start: usize) -> u8 {
        let mut crc = 0;
        for i in &array[start..] {
            crc = CRC_TABLE[(crc ^ i) as usize]
        }
        crc
    }

    struct AmProto {
        msgtype: u8,
        subcmd: u8,
        payload: inParams,
    }

    impl AmProto {
        fn to_byte_array(&self) -> Vec<u8> {
            let mut array = Vec::new();

            array.push(0x02); // STX
            array.push(self.msgtype); // msgtype
                                      // length, (added later)
            array.push(self.subcmd); // subcmd
            array.append(self.payload.to_byte_array().as_mut());
            array.insert(2, (array.len() - 2) as u8); //length, -2 to skip STX and msgtype
            array.push(calc_crc(&array, 1)); //crc, 1 to skip STX
            array.push(0x03); // ETX

            array
        }
        fn new(command: inParams) -> Self {
            let (msgtype, subcmd) = get_msgtype(command);
            Self {
                msgtype: msgtype.try_into().expect("amg3 command sent to amproto"),
                subcmd,
                payload: command,
            }
        }
    }

    struct Amg3 {
        id: u8,
        //length: u16,
        trans_id: u8,
        message_type: u16, // Can be u8 if the first byte is larger than 0x7F
        subcmd: u8,
        //payload_length: u8,
        payload: inParams,
        //crc: u8,
    }

    impl Amg3 {
        fn to_byte_array(&self) -> Vec<u8> {
            let mut array = Vec::new();

            array.push(0x02); // Stx
            array.push(self.id); // Id
                                 // length
            array.push(self.trans_id); // Transision id
            array.extend_from_slice(self.message_type.to_be_bytes().as_slice()); // is all of amproto BE hopefully not because then i have more work to do

            let mut payload = self.payload.to_byte_array();
            array.push((payload.len() as u8) + 1); // Payload length, add 1 for subcmd
            array.push(self.subcmd); // subcmd
            array.append(payload.as_mut()); // Payload

            array.splice(2..2, (array.len() as u16).to_le_bytes()); // Length

            array.push(calc_crc(&array, 1)); // Crc
            array.push(0x03); // ETX

            array
        }
        fn new(command: inParams) -> Self {
            let (message_type, subcmd) = get_msgtype(command);
            Self {
                id: 0x81,    // this seems to always be 0x81
                trans_id: 0, // this always seems to be 0 for some reason
                message_type: message_type | 0x8000,
                subcmd,
                payload: command,
            }
        }
    }

    impl inParams {
        fn to_byte_array(&self) -> Vec<u8> {
            match *self {
                inParams::SystemSettingsSetHeadlightEnabled(p) => vec![p],
                inParams::WheelsGetSpeed(p) => vec![p],
                inParams::WheelsGetRotationCounter(p) => vec![p],
                inParams::LoopSamplerGetLoopSignalMaster(p) => vec![p as u8],
                inParams::HardwareControlWheelMotorsPower(p, q) => {
                    [p.to_le_bytes(), q.to_le_bytes()].concat()
                }
                inParams::MowerAppSetMode(p) => vec![p as u8],
                inParams::SystemSettingsSetLoopDetection(p) => vec![p],
                inParams::HeightMotorSetHeight(p) => vec![p],
                inParams::CollisionSetSimulation(p) => vec![p as u8],
                inParams::CollisionSetSimulatedStatus(p) => Vec::from(p.to_le_bytes()),
                inParams::SoundSetSoundType(p) => vec![p as u8],
                _ => Vec::new(),
            }
        }
    }

    pub fn encode(command: inParams) -> (Vec<u8>, u8) {
        let (msgtype, subcmd) = get_msgtype(command);
        match msgtype {
            4000.. => (Amg3::new(command).to_byte_array(), subcmd),
            _ => (AmProto::new(command).to_byte_array(), subcmd),
        }
    }
}
