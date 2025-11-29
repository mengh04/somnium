use crate::playback::state::PlayerState;

pub enum PlayerEvent {
    StateChanged(PlayerState),
    PositionChanged(u64),
    Error(String),
}
