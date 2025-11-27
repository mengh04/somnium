use std::{path::PathBuf, sync::mpsc, thread};

use super::player_core::PlayerCore;

#[derive(Clone)]
pub struct Player {
    pub command_sender: mpsc::Sender<PlayerCommand>,
}

impl Player {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut player_core = PlayerCore::new();

            while let Ok(command) = rx.recv() {
                match command {
                    PlayerCommand::Play(path) => {
                        player_core.play(path);
                    }
                    PlayerCommand::Pause => {
                        player_core.pause();
                    }
                    PlayerCommand::Stop => {
                        player_core.stop();
                    }
                    PlayerCommand::Seek(position) => {
                        player_core.seek(position);
                    }
                    PlayerCommand::SkipBack => {
                        player_core.skip_back();
                    }
                    PlayerCommand::SkipForward => {
                        player_core.skip_forward();
                    }
                    PlayerCommand::Resume => {
                        player_core.resume();
                    }
                }
            }
        });

        Self { command_sender: tx }
    }
}

#[derive(Debug, Clone)]
pub enum PlayerCommand {
    Play(PathBuf),
    Pause,
    Stop,
    Seek(u64),
    SkipForward,
    SkipBack,
    Resume,
}
