use crate::{actions::PlayerAction, message::GameMessage, player::Players, world::World};
use stylish::ansi::format as ansi_format;

#[derive(Debug)]
pub struct LookAction {
    pub sender: u32,
    // TODO: This should allow arguments to look a other objects or players
}

impl PlayerAction for LookAction {
    fn perform(&self, players: &Players, world: &World) {
        if let Some(sending_player) = players.read().get(&self.sender) {
            tracing::debug!("Received look from player: {}", sending_player.username);
            // TODO: Again, what if they're in a non-existent room or something
            if let Some(room) = world.get_player_room(&sending_player) {
                let exits = {
                    if room.exits.is_empty() {
                        String::from("You don't see any exits.")
                    } else {
                        let exits_list = room
                            .exits
                            .keys()
                            .map(String::as_str)
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("You see exits to the {}", exits_list)
                    }
                };

                let mut other_characters: Vec<String> = Vec::new();

                let players_in_room = room.get_players(&players);
                {
                    let guard = players.read();
                    let players_list = players_in_room
                        .iter()
                        // Don't tell us that we're in the room, we know that.
                        .filter(|key| *key != &self.sender)
                        .filter_map(|key| guard.get(key))
                        .map(|p| p.username.clone());
                    other_characters.extend(players_list);
                }

                // TODO: Consolidate this and other_players
                let mobiles_in_room = room.get_mobiles(world);
                let mobiles_list = mobiles_in_room
                    .iter()
                    .filter_map(|key| world.mobiles.get(key))
                    .map(|m| m.template.room_description.clone());

                other_characters.extend(mobiles_list);

                let other_characters_string = {
                    if other_characters.len() == 0 {
                        String::from("You're the only one here.")
                    } else {
                        format!("You see {} here.", other_characters.join(", "))
                    }
                };

                sending_player.send_message(ansi_format!(
                    "{:(fg=green,bold)}\n{}\n{}\n{}",
                    room.name,
                    room.description,
                    exits,
                    other_characters_string
                ));
            }
        }
    }
}
