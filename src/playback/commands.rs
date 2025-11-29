use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum PlayerCommand {
    Play(PathBuf),
    Pause,
    Stop,
    Resume,
    SkipBack,
    SkipForward,
}
