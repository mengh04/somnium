use std::{
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
};

#[derive(Clone)]
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
                        // For simplicity, we'll just call play with a placeholder path.
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

struct PlayerCore;

impl PlayerCore {
    fn new() -> Self {
        Self
    }

    fn play(&self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        eprintln!("Player: Play {}", path.display());
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

    fn resume(&self) {
        eprintln!("Player: Resume");
    }
}
