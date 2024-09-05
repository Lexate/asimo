use std::time::Duration;

//TODO: Error handling

fn main() {
    let mut serial = asimo_comms::Serial::new("/dev/ttyUSB0", 115_200, Duration::from_millis(10));

    let output: Vec<u8> = vec![2, 129, 8, 0, 0, 144, 172, 2, 0, 0, 96, 3]; // Setsound type (0)

    serial.write(&output[..]);

    let response = serial.read();
    println!("{:?}", response);
}

mod asimo_comms {
    use std::time::Duration;

    use serialport::SerialPort;

    pub struct Serial {
        port: Box<dyn SerialPort>,
    }

    impl Serial {
        pub fn new(path: &str, baud_rate: u32, timeout: Duration) -> Self {
            let port = serialport::new(path, baud_rate)
                .timeout(timeout)
                .open()
                .expect("Failed to open port ");

            Self { port }
        }

        pub fn write(&mut self, output: &[u8]) {
            self.port.write(&output[..]).expect("Could not write");
        }

        pub fn read(&mut self) -> Vec<u8> {
            let mut serial_buf: Vec<u8> = vec![0; 256];
            self.port
                .read(serial_buf.as_mut_slice())
                .expect("Found no data");
            serial_buf
        }

        pub fn write_read(&mut self, output: &[u8]) -> Vec<u8> {
            self.write(output);
            self.read()
        }
    }
}
