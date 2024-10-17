use asimo::{
    comms,
    types::{self, inParams},
};
use gilrs::{Button, Event, Gilrs};
use std::{thread::sleep, time::Duration};
//TODO: Error handling

fn motor(left: f32, right: f32) -> inParams {
    let left = f32::clamp(left, -1.0, 1.0);
    let right = f32::clamp(right, -1.0, 1.0);

    types::inParams::HardwareControlWheelMotorsPower {
        leftWheelMotorPower: (left * 100.0).round() as i16,
        rightWheelMotorPower: (right * 100.0).round() as i16,
    }
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

fn motor_test() {
    let mut serial = comms::Serial::new("/dev/ttyUSB0", 115200, Duration::from_millis(1))
        .expect("Could not establish comms");

    let mut gilrs = Gilrs::new().unwrap();
    // Iterate over all connected gamepads
    for (id, gamepad) in gilrs.gamepads() {
        println!(
            "{} is {:?}, id: {:?}",
            gamepad.name(),
            gamepad.power_info(),
            id
        );
    }

    let mut active_gamepad = None;

    let resp = serial
        .send_message(types::inParams::DeviceInformationGetDeviceIdentification {})
        .unwrap();
    println!("{:02X?}", resp);

    serial
        .send_message(types::inParams::SystemSettingsSetLoopDetection { loopDetection: 0 })
        .unwrap();

    let resp = serial
        .send_message(types::inParams::SystemSettingsGetLoopDetection {})
        .unwrap();
    println!("{:02X?}", resp);

    loop {
        let state = serial
            .send_message(types::inParams::MowerAppGetState {})
            .expect("could not get state");

        println!("State : {:02X?}", state);

        if state.get(9).expect("State smaller than 11") == &6 {
            serial
                .send_message(types::inParams::MowerAppPause {})
                .expect("could not pause");
            println!("*******************************************");
            break;
        }
        sleep(Duration::from_millis(1000));
    }

    // Confirmation beep
    serial
        .send_message(types::inParams::SoundSetSoundType {
            soundType: types::tSoundType::SoundDoubleBeep,
        })
        .unwrap();

    let mut fwd = 0.0;
    let mut rot = 0.0;

    let mut left = 0.0;
    let mut right = 0.0;

    loop {
        // Examine new events
        while let Some(Event { id, .. }) = gilrs.next_event() {
            active_gamepad = Some(id)
        }

        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            match gamepad.axis_data(gilrs::Axis::LeftStickY) {
                Some(d) => fwd = d.value(),
                None => (),
            };
            match gamepad.axis_data(gilrs::Axis::RightStickX) {
                Some(d) => rot = d.value(),
                None => (),
            };

            if gamepad.is_pressed(Button::South) {
                //A
                println!("Wahoo")
            }
            if gamepad.is_pressed(Button::West) {
                //X
                break;
            }

            (left, right) = arcade_drive(rot, fwd, true);

            let lft = (left * 100.0).round() as i16;
            let rght = (right * 100.0).round() as i16;

            println!("Left: {}, Right: {}", lft, rght);
        }

        let resp = serial.send_message(motor(left, right)).unwrap();
        println!("{:02X?}", resp);

        let resp = serial
            .send_message(types::inParams::RealTimeDataGetWheelMotorData {})
            .unwrap();
        println!("{:02X?}", resp);

        sleep(Duration::from_millis(20));
    }

    let resp = serial.send_message(motor(0.0, 0.0)).unwrap();
    println!("{:02X?}", resp);
}

fn main() {}
