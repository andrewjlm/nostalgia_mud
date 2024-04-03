use crate::message::GameMessage;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct Player {
    // TODO: Is this really how we want to handle player IDs?
    pub id: u32,
    pub username: String,
    // Sender for sending GameMessages to the player
    sender: mpsc::UnboundedSender<GameMessage>,
    // The player's current room
    pub current_room: u32,
}

impl Player {
    pub fn new(
        username: String,
        players: Players,
        sender: mpsc::UnboundedSender<GameMessage>,
        starting_room: u32,
    ) -> Player {
        let player_id = generate_player_id(players);
        Player {
            id: player_id,
            username: username,
            sender,
            current_room: starting_room,
        }
    }

    pub fn game_message(&self, message: GameMessage) {
        self.sender.send(message);
    }

    pub fn move_to_room(&mut self, room_id: u32) {
        // TODO: The player should get something sent when they enter a new room. This is probably
        // configurable - you either get a quick summary of where you are or a full "look"
        log::debug!("Player {} moved to {}", self.username, room_id);
        self.current_room = room_id;
    }
}

// Type aliases for ease of use
pub type Players = Arc<RwLock<HashMap<u32, Player>>>;

// TODO: Make this a method or associated function on Player?
fn generate_player_id(players: Players) -> u32 {
    // TODO: This seems horribly inefficient
    let mut id = 1;
    // NOTE: We don't technically write here but I think we want an exclusive lock
    while players.write().unwrap().contains_key(&id) {
        id += 1;
    }
    id
}
