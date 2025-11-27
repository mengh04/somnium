use std::path::Path;

use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend,
    sound::{
        FromFileError,
        streaming::{StreamingSoundData, StreamingSoundHandle},
    },
};

pub struct PlayerCore {
    manager: AudioManager<DefaultBackend>,
    current_sound: Option<StreamingSoundHandle<FromFileError>>,
}

impl PlayerCore {
    pub fn new() -> Self {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .expect("Failed to initialize audio manager");

        Self {
            manager,
            current_sound: None,
        }
    }

    pub fn play(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();

        if let Some(mut sound) = self.current_sound.take() {
            let _ = sound.stop(Default::default());
        }

        match StreamingSoundData::from_file(path) {
            Ok(sound_data) => match self.manager.play(sound_data) {
                Ok(handle) => {
                    eprintln!("Player: Playing {} (streaming)", path.display());
                    self.current_sound = Some(handle);
                }
                Err(e) => {
                    eprintln!("Player: Failed to play {}: {}", path.display(), e);
                }
            },
            Err(e) => {
                eprintln!("Player: Failed to load {}: {}", path.display(), e);
            }
        }
    }

    /// 暂停播放
    pub fn pause(&mut self) {
        if let Some(sound) = &mut self.current_sound {
            let _ = sound.pause(Default::default());
            eprintln!("Player: Paused");
        }
    }

    /// 恢复播放
    pub fn resume(&mut self) {
        if let Some(sound) = &mut self.current_sound {
            let _ = sound.resume(Default::default());
            eprintln!("Player: Resumed");
        }
    }

    /// 停止播放
    pub fn stop(&mut self) {
        if let Some(mut sound) = self.current_sound.take() {
            let _ = sound.stop(Default::default());
            eprintln!("Player: Stopped");
        }
    }

    /// 跳转到指定位置（秒）
    pub fn seek(&mut self, position: u64) {
        if let Some(sound) = &mut self.current_sound {
            let _ = sound.seek_to(position as f64);
            eprintln!("Player: Seeked to {} seconds", position);
        }
    }

    /// 后退 10 秒
    pub fn skip_back(&mut self) {
        if let Some(sound) = &mut self.current_sound {
            let current_position = sound.position();
            let new_position = (current_position - 10.0).max(0.0);
            let _ = sound.seek_to(new_position);
            eprintln!("Player: Skipped back to {} seconds", new_position);
        }
    }

    /// 前进 10 秒
    pub fn skip_forward(&mut self) {
        if let Some(sound) = &mut self.current_sound {
            let current_position = sound.position();
            let new_position = current_position + 10.0;
            let _ = sound.seek_to(new_position);
            eprintln!("Player: Skipped forward to {} seconds", new_position);
        }
    }

    /// 获取当前播放位置（秒）
    pub fn position(&self) -> Option<f64> {
        self.current_sound.as_ref().map(|sound| sound.position())
    }

    /// 检查是否正在播放
    pub fn is_playing(&self) -> bool {
        self.current_sound
            .as_ref()
            .map(|sound| {
                use kira::sound::PlaybackState;
                matches!(sound.state(), PlaybackState::Playing)
            })
            .unwrap_or(false)
    }
}
