use crate::playback::state::PlayerState;

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    StateChanged(PlayerState),
    PositionChanged(u64),
    Error(String),
}
