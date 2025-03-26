mod gen_types;
pub use crate::gen_types::Types;
mod type_methods;

mod error;
pub use error::{Error, Result};
mod ser;
pub use ser::{to_bytes, Serializer};
mod de;
pub use de::Deserializer;

trait Hcp {
    // Tuple is (msgtype: u16, subcmd: u8)
    fn get_msgtype_subcmd() -> (u16, u8);
    fn get_outparams() -> Self;
}

pub mod comms {
    use crate::{proto, type_methods::Hcp};
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

        pub(crate) fn send(&mut self, message: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
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

        pub fn send_message(&mut self, message: &impl Hcp) -> Result<Vec<u8>, std::io::Error> {
            let (bytes, _subcmd) = proto::encode(message);
            self.send(bytes)
        }
    }
}

pub mod proto {
    use crate::{gen_types::Types::*, type_methods::Hcp};
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

    fn calc_crc(array: &[u8], start: usize) -> u8 {
        let mut crc = 0;
        for i in &array[start..] {
            crc = CRC_TABLE[(crc ^ i) as usize]
        }
        crc
    }
}
