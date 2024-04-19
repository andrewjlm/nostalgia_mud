use std::collections::HashMap;

use crate::area::Area;
use crate::mobiles::{Mobile, MobileInstance};
use crate::player::Player;
use crate::reset::ResetCommand;
use crate::room::{get_sample_rooms, Room};

// TODO: We might want to do something similar to what we did to the Players struct in terms of
// making it a wrapper around an Arc/RwLock. That is, if we ever need something other than the game
// loop to update the world. One example could be if we make the `tick` function do stuff to the
// world.
pub struct World {
    rooms: HashMap<u32, Room>,
    mobile_templates: HashMap<u32, Mobile>,
    // TODO: Better accessing...
    pub mobiles: HashMap<u32, MobileInstance>,
    resets: Vec<ResetCommand>,
}

impl World {
    pub fn new() -> Self {
        World {
            rooms: HashMap::new(),
            mobile_templates: HashMap::new(),
            mobiles: HashMap::new(),
            resets: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        // Perform any resets
        for reset in &self.resets {
            tracing::info!("Performing reset {:?}", reset);
            // TODO: Check if we actually need to run the reset
            let template = self.mobile_templates.get(&reset.mobile_id).unwrap().clone();
            let target_room = reset.room_id;

            // Generate a unique ID for the MobileInstance
            let mut id = 1;
            while self.mobiles.contains_key(&id) {
                id += 1;
            }

            let mi = MobileInstance {
                id: id,
                template: template,
                current_room: target_room,
            };

            // TODO: Add check here that we're not inserting into an already used ID
            self.mobiles.insert(mi.id, mi);
        }
    }

    pub fn add_area(&mut self, area: Area) {
        // TODO: Right now we just put all the areas into one "scope"
        for r in area.rooms {
            self.rooms.insert(r.id, r);
        }

        for m in area.mobiles {
            self.mobile_templates.insert(m.id, m);
        }

        for rc in area.resets {
            self.resets.push(rc);
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.id, room);
    }

    pub fn add_mobile_template(&mut self, mobile: Mobile) {
        self.mobile_templates.insert(mobile.id, mobile);
    }

    pub fn add_reset(&mut self, reset: ResetCommand) {
        self.resets.push(reset);
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
