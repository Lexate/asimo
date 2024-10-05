use asimo::{comms, proto, types};
use core::time;
use std::{io::Read, thread::sleep, time::Duration};
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
        .send_message(proto::encode(types::inParams::DeviceInformationGetDeviceIdentification()).0)
        .unwrap();
    println!("{:02X?}", resp);

    serial
        .send_message(proto::encode(types::inParams::SystemSettingsSetLoopDetection(0)).0)
        .unwrap();

    let resp = serial
        .send_message(proto::encode(types::inParams::SystemSettingsGetLoopDetection()).0)
        .unwrap();
    println!("{:02X?}", resp);

    loop {
        let state = serial
            .send_message(proto::encode(types::inParams::MowerAppGetState()).0)
            .expect("could not get state");

        println!("State : {:02X?}", state);

        if state.get(9).expect("State smaller than 11") == &6 {
            serial
                .send_message(proto::encode(types::inParams::MowerAppPause()).0)
                .expect("could not pause");
            println!("*******************************************");
            break;
        }
        sleep(Duration::from_millis(1000));
    }

    // Confirmation beep
    serial
        .send_message(
            proto::encode(types::inParams::SoundSetSoundType(
                types::tSoundType::SoundDoubleBeep,
            ))
            .0,
        )
        .unwrap();

    for _ in 1..100 {
        let resp = serial.send_message(motor(0.30, -0.30)).unwrap();
        println!("{:?}", resp);

        let resp = serial
            .send_message(proto::encode(types::inParams::RealTimeDataGetWheelMotorData()).0)
            .unwrap();
        println!("{:?}", resp);

        sleep(Duration::from_millis(20));
    }

    let resp = serial.send_message(motor(0.0, 0.0)).unwrap();
    println!("{:?}", resp);
}
