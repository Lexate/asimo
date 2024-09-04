use std::time::Duration;

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let output: Vec<u8> = vec![2, 129, 8, 0, 0, 144, 172, 2, 0, 0, 96, 3]; // Setsound type

    port.write(&output[..]).expect("Could not write");
}
