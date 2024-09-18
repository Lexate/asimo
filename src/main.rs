use asimo::{proto, types};
use std::time::Duration;
//TODO: Error handling

fn main() {
    println!(
        "{:x?}",
        proto::encode(types::inParams::SoundSetSoundType(
            types::tSoundType::SoundKeyClick
        ))
    );
    println!(
        "{:x?}",
        proto::encode(types::inParams::SystemSettingsSetHeadlightEnabled(1))
    );
}
