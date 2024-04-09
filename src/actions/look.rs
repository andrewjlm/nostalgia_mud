use crate::{actions::PlayerAction, message::GameMessage, player::Players, world::World};

#[derive(Debug)]
pub struct LookAction {
    pub sender: u32,
    // TODO: This should allow arguments to look a other objects or players
}

impl PlayerAction for LookAction {
    fn perform(&self, players: &Players, world: &World) {
        if let Some(sending_player) = players.write().get_mut(&self.sender) {
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

                sending_player
                    .game_message(format!("{}\n{}\n{}", room.name, room.description, exits));
            }
        }
    }
}
