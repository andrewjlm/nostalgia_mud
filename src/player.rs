use crate::message::GameMessage;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub struct Player {
    // TODO: Is this really how we want to handle player IDs?
    pub id: u32,
    pub username: String,
    // Sender for sending GameMessages to the player
    sender: mpsc::UnboundedSender<GameMessage>,
}

impl Player {
    pub fn new(players: Players, sender: mpsc::UnboundedSender<GameMessage>) -> Player {
        let player_id = generate_player_id(players);
        Player {
            id: player_id,
            username: format!("Player{}", player_id),
            sender,
        }
    }

    pub fn game_message(&self, message: GameMessage) {
        self.sender.send(message);
    }
}

// Type aliases for ease of use
pub type Players = Arc<Mutex<HashMap<u32, Arc<Player>>>>;

// TODO: Make this a method or associated function on Player?
fn generate_player_id(players: Players) -> u32 {
    // TODO: This seems horribly inefficient
    let mut id = 1;
    while players.lock().unwrap().contains_key(&id) {
        id += 1;
    }
    id
}
