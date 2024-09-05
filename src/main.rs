use std::time::Duration;

//TODO: Error handling

fn main() {
    let mut serial = asimo_comms::Serial::new("/dev/ttyUSB0", 115_200, Duration::from_millis(10))
        .expect("Could not open port");

    let output: Vec<u8> = vec![2, 129, 8, 0, 0, 144, 172, 2, 0, 0, 96, 3]; // Setsound type (0)

    serial.write(&output[..]).expect("Could not write");

    let response = serial.read().expect("Could not read");
    println!("{:?}", response);
}

mod asimo_comms {
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
