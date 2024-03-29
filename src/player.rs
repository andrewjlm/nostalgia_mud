use crate::message::Message;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub struct Player {
    // TODO: Is this really how we want to handle player IDs?
    pub id: u32,
    pub username: String,
    // TODO: Make this private?
    pub sender: mpsc::Sender<Message>,
}

impl Player {
    pub fn new(players: Players, sender: mpsc::Sender<Message>) -> Player {
        let player_id = generate_player_id(players);
        Player {
            id: player_id,
            username: format!("Player{}", player_id),
            sender,
        }
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
