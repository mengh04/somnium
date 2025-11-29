use std::sync::{
    Arc, Mutex, OnceLock, mpsc,
    mpsc::{Receiver, Sender},
};
use std::thread;

use crate::playback::commands::PlayerCommand;
use crate::playback::events::PlayerEvent;
use crate::playback::player::Player;

static PLAYER_SERVICE: OnceLock<PlayerService> = OnceLock::new();

#[derive(Debug)]
pub struct PlayerService {
    pub command_sender: Sender<PlayerCommand>,
    pub event_receiver: Arc<Mutex<Receiver<PlayerEvent>>>,
}

impl PlayerService {
    pub fn init() {
        let (command_tx, command_rx) = mpsc::channel::<PlayerCommand>();
        let (event_tx, event_rx) = mpsc::channel::<PlayerEvent>();

        let _ = thread::spawn(move || {
            let mut player = Player::new(event_tx);
            player.run(command_rx);
        });

        let player_service = PlayerService {
            command_sender: command_tx,
            event_receiver: Arc::new(Mutex::new(event_rx)),
        };

        PLAYER_SERVICE.set(player_service).unwrap();
    }

    pub fn get() -> &'static PlayerService {
        PLAYER_SERVICE
            .get()
            .expect("PlayerService is not initialized")
    }
}
