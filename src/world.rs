use std::collections::HashMap;

use crate::player::Player;
use crate::room::{get_sample_rooms, Room};

// TODO: We might want to do something similar to what we did to the Players struct in terms of
// making it a wrapper around an Arc/RwLock. That is, if we ever need something other than the game
// loop to update the world. One example could be if we make the `tick` function do stuff to the
// world.
pub struct World {
    rooms: HashMap<u32, Room>,
}

impl World {
    pub fn new() -> Self {
        World {
            rooms: HashMap::new(),
        }
    }

    // TODO: I like having some kind of way to make this by collecting a list of rooms into the
    // internal hashmap

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.id, room);
    }

    pub fn get_room(&self, room_id: u32) -> Option<&Room> {
        self.rooms.get(&room_id)
    }

    pub fn get_player_room(&self, player: &Player) -> Option<&Room> {
        // TODO: Seems kinda bad if the player is in a non-existent room but not clear if we handle
        // here or somewhere else
        self.get_room(player.current_room)
    }

    pub fn get_player_exits(&self, player: &Player) -> Option<&HashMap<String, u32>> {
        self.get_room(player.current_room).map(|room| &room.exits)
    }
}

pub fn get_sample_world() -> World {
    let rooms = get_sample_rooms();

    let mut world = World::new();
    for room in rooms {
        world.add_room(room);
    }

    world
}
