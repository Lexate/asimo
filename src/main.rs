fn main() {
    let serial = asimo_comms::Serial::new("/dev/ttyUSB0", 115_200);

    let output: Vec<u8> = vec![2, 129, 8, 0, 0, 144, 172, 2, 0, 0, 96, 3]; // Setsound type (0)

    serial.write(&output[..]).expect("Could not write");
}

mod asimo_comms {
    use std::time::Duration;

    use serialport::SerialPort;

    pub struct Serial {
        port: Box<dyn SerialPort>,
    }

    impl Serial {
        pub fn new(path: &str, baud_rate: u32) -> Self {
            let port = serialport::new(path, baud_rate)
                .timeout(Duration::from_millis(10))
                .open()
                .expect("Failed to open port ");

            Self { port }
        }

        pub fn write(&mut self, output: &[u8]) {
            self.port.write(&output[..]);
        }
    }
}
