use asimo::{proto, types};
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
    proto::decode(
        [
            2, 0x15, 0xf, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x91, 0x3,
        ]
        .to_vec(),
    )
    .unwrap();
}
