use crate::{actions::PlayerAction, message::GameMessage, player::Players, world::World};
use stylish::ansi::format as ansi_format;

#[derive(Debug)]
pub struct MobileAction {
    pub sender: u32,
    // TODO: This should allow arguments to look a other objects or players
}

impl PlayerAction for MobileAction {
    fn perform(&self, players: &Players, world: &World) {
        if let Some(sending_player) = players.read().get(&self.sender) {
            tracing::debug!("Received mobile from player: {}", sending_player.username);
            // Get a list of every mobile in the world and what room they're in
            let mut mobile_locations: Vec<String> = Vec::new();
            for (id, m) in world.mobiles.iter() {
                let mob_name = m.template.room_description.clone();
                let current_room_id = m.current_room;
                let current_room_name = world
                    .get_room(current_room_id)
                    .map_or_else(|| "UNKNOWN".to_string(), |r| r.name.clone());
                // TODO: If this persists beyond the debug state, make the formatting better
                mobile_locations.push(format!(
                    "{}\t{}\t{}\t{}",
                    id, mob_name, current_room_id, current_room_name
                ));
            }
            sending_player.send_message(mobile_locations.join("\n"));
        }
    }
}
