use std::{sync::mpsc, thread};
pub struct Player {
    pub command_sender: mpsc::Sender<PlayerCommand>,
}

impl Player {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let player_core = PlayerCore::new();

            while let Ok(command) = rx.recv() {
                match command {
                    PlayerCommand::Play => {
                        player_core.play();
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
                }
            }
        });

        Self { command_sender: tx }
    }
}

pub enum PlayerCommand {
    Play,
    Pause,
    Stop,
    Seek(u64),
    SkipForward,
    SkipBack,
}

struct PlayerCore;

impl PlayerCore {
    fn new() -> Self {
        Self
    }

    fn play(&self) {
        eprintln!("Player: Play");
    }

    fn pause(&self) {
        eprintln!("Player: Pause");
    }

    fn stop(&self) {
        eprintln!("Player: Stop");
    }

    fn seek(&self, position: u64) {
        eprintln!("Player: Seek to {}", position);
    }

    fn skip_back(&self) {
        eprintln!("Player: SkipBack");
    }

    fn skip_forward(&self) {
        eprintln!("Player: SkipForward");
    }
}
