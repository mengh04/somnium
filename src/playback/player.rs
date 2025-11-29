use std::path::Path;
use std::sync::mpsc::Receiver;

use kira::sound::FromFileError;
use kira::sound::streaming::{StreamingSoundData, StreamingSoundHandle};
use kira::{AudioManager, AudioManagerSettings, DefaultBackend, Tween};
use tokio::sync::broadcast;

use crate::playback::commands::PlayerCommand;
use crate::playback::events::PlayerEvent;
use crate::playback::state::PlayerState;

pub struct Player {
    event_sender: broadcast::Sender<PlayerEvent>,
    state: PlayerState,
    manager: AudioManager<DefaultBackend>,
    streaming_sound_handle: Option<StreamingSoundHandle<FromFileError>>,
}

impl Player {
    pub fn new(event_sender: broadcast::Sender<PlayerEvent>) -> Self {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

        Player {
            event_sender,
            state: PlayerState::Stopped,
            manager,
            streaming_sound_handle: None,
        }
    }

    pub fn run(&mut self, command_receiver: Receiver<PlayerCommand>) {
        while let Ok(command) = command_receiver.recv() {
            match command {
                PlayerCommand::Play(path) => {
                    eprintln!("playing: {}", path.display());
                    self.state = PlayerState::Playing;
                    self.play(path);
                    let _ = self
                        .event_sender
                        .send(PlayerEvent::StateChanged(self.state.clone()));
                }
                PlayerCommand::Pause => {
                    eprintln!("Paused");
                    self.state = PlayerState::Paused;
                    let _ = self
                        .event_sender
                        .send(PlayerEvent::StateChanged(self.state.clone()));
                }
                PlayerCommand::Stop => {
                    eprintln!("Stopped");
                    self.state = PlayerState::Stopped;
                    let _ = self
                        .event_sender
                        .send(PlayerEvent::StateChanged(self.state.clone()));
                } // Add more command handling as needed
                PlayerCommand::Resume => {
                    eprintln!("Resumed");
                    self.state = PlayerState::Playing;
                    let _ = self
                        .event_sender
                        .send(PlayerEvent::StateChanged(self.state.clone()));
                }
                PlayerCommand::SkipBack => {
                    // Handle skip back command
                }
                PlayerCommand::SkipForward => {
                    // Handle skip forward command
                }
            }
        }
    }

    fn play(&mut self, path: impl AsRef<Path>) {
        if let Some(handle) = &mut self.streaming_sound_handle {
            let _ = handle.stop(Tween::default());
        }

        let path = path.as_ref();
        let sound_data = StreamingSoundData::from_file(path).unwrap();
        self.streaming_sound_handle = Some(self.manager.play(sound_data).unwrap());
    }
}
