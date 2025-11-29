use std::sync::mpsc::Receiver;

use tokio::sync::broadcast;

use crate::playback::commands::PlayerCommand;
use crate::playback::events::PlayerEvent;
use crate::playback::state::PlayerState;

pub struct Player {
    event_sender: broadcast::Sender<PlayerEvent>,
    state: PlayerState,
}

impl Player {
    pub fn new(event_sender: broadcast::Sender<PlayerEvent>) -> Self {
        Player {
            event_sender,
            state: PlayerState::Stopped,
        }
    }

    pub fn run(&mut self, command_receiver: Receiver<PlayerCommand>) {
        while let Ok(command) = command_receiver.recv() {
            match command {
                PlayerCommand::Play(path) => {
                    eprintln!("playing: {}", path.display());
                    self.state = PlayerState::Playing;
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
}
