use std::thread;

use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, sound::static_sound::StaticSoundData,
};

const SOUND_FILE_PATH: &str = "D:\\Music\\Old Memory - 市川淳.flac";

fn main() -> anyhow::Result<()> {
    // Create an audio manager. This plays sounds and manages resources.
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let sound_data = StaticSoundData::from_file(SOUND_FILE_PATH)?;
    manager.play(sound_data.clone())?;

    thread::sleep(std::time::Duration::from_secs(60));

    Ok(())
}
