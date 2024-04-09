use crate::message::GameMessage;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct Player {
    // TODO: Is this really how we want to handle player IDs?
    pub id: u32,
    pub username: String,
    // Sender for sending GameMessages to the player
    sender: mpsc::UnboundedSender<String>,
    // The player's current room
    // TODO: Should this be a reference? Makes things a mess but some things might only be possible
    // with it. In particular, I'm wondering if there is some sort of "Area" chat (Yell?). To use
    // our predicate channel creation, we'd need to know all the other players in the area. Not
    // that we have areas yet...
    pub current_room: u32,
}

impl Player {
    pub fn new(
        username: String,
        players: &Players,
        sender: mpsc::UnboundedSender<String>,
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

    pub fn game_message(&self, message: String) {
        let _ = self.sender.send(message);
    }

    #[tracing::instrument(skip_all,
                          fields(username = self.username,
                                 previous = self.current_room, new = room_id))]
    pub fn move_to_room(&mut self, room_id: u32) {
        // TODO: The player should get something sent when they enter a new room. This is probably
        // configurable - you either get a quick summary of where you are or a full "look"
        tracing::debug!("Player moved rooms");
        self.current_room = room_id;
    }
}

// newtype for ease of use
// We can derive Clone for free because it's a wrapper around Arc
#[derive(Clone)]
pub struct Players(Arc<RwLock<HashMap<u32, Player>>>);

impl Players {
    pub fn new() -> Self {
        Players(Arc::new(RwLock::new(HashMap::new())))
    }

    pub fn read(&self) -> RwLockReadGuard<HashMap<u32, Player>> {
        self.0.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<HashMap<u32, Player>> {
        self.0.write().unwrap()
    }
}

// TODO: Make this a method or associated function on Player?
fn generate_player_id(players: &Players) -> u32 {
    // TODO: This seems horribly inefficient
    let mut id = 1;
    // NOTE: We don't technically write here but I think we want an exclusive lock
    while players.write().contains_key(&id) {
        id += 1;
    }
    id
}
