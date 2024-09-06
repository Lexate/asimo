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
    /*struct AmProto {
        msgtype: u8,
        length: ,
        subcmd: ,
        payload: ,
        crc: ,
    }*/
}
