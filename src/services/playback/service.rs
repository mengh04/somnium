use std::sync::OnceLock;

use super::player::Player;

static PLAYER_SERVICE: OnceLock<Player> = OnceLock::new();

pub struct PlayerService;

impl PlayerService {
    pub fn init() {
        PLAYER_SERVICE.get_or_init(|| Player::new());
    }

    pub fn get() -> Player {
        PLAYER_SERVICE.get_or_init(|| Player::new()).clone()
    }
}
