use asimo;
use std::time::Duration;
//TODO: Error handling

fn main() {
    let mut serial = asimo::comms::Serial::new("/dev/ttyUSB0", 115_200, Duration::from_millis(10))
        .expect("Could not open port");

    let output: Vec<u8> = vec![2, 129, 8, 0, 0, 144, 172, 2, 0, 0, 96, 3]; // Setsound type (0)

    serial.write(&output[..]).expect("Could not write");

    let response = serial.read().expect("Could not read");
    println!("{:?}", response);
}
