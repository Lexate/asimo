use asimo::{comms, proto, types};
use std::{thread::sleep, time::Duration};
//TODO: Error handling

fn motor(left: f32, right: f32) -> Vec<u8> {
    let left = f32::clamp(left, -1.0, 1.0);
    let right = f32::clamp(right, -1.0, 1.0);

    proto::encode(types::inParams::HardwareControlWheelMotorsPower(
        (left * 100.0) as i16,
        (right * 100.0) as i16,
    ))
    .0
}

fn arcade_drive(rot: f32, fwd: f32, square: bool) -> (f32, f32) {
    let (rot, fwd) = if square {
        (rot.signum() * rot * rot, fwd.signum() * fwd * fwd)
    } else {
        (rot, fwd)
    };

    let max = f32::max(rot.abs(), fwd.abs());
    let (tot, diff) = (fwd + rot, fwd - rot);

    if fwd >= 0.0 {
        if rot >= 0.0 {
            return (max, diff);
        } else {
            return (tot, max);
        }
    } else {
        if rot >= 0.0 {
            return (tot, -max);
        } else {
            return (-max, diff);
        }
    }
}

fn main() {
    let mut serial = comms::Serial::new("/dev/ttyUSB0", 115200, Duration::from_millis(1))
        .expect("Could not establish comms");

    let resp = serial
        .send_message(
            proto::encode(types::inParams::SoundSetSoundType(
                types::tSoundType::SoundClick,
            ))
            .0,
        )
        .unwrap();

    println!("{:?}", resp);
    //sleep(Duration::from_millis(20));
    //serial
    //    .write(&proto::encode(types::inParams::MowerAppPause()).0[..])
    //    .unwrap();
    //sleep(Duration::from_millis(20));

    //serial.write(&motor(0.3, 0.3)[..]).unwrap();
}
