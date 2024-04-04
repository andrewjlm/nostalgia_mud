use crate::{actions::PlayerAction, message::GameMessage, player::Players, world::World};

pub struct LookAction {
    pub sender: u32,
    // TODO: This should allow arguments to look a other objects or players
}

impl PlayerAction for LookAction {
    fn perform(&self, players: &Players, world: &World) {
        if let Some(sending_player) = players.write().get_mut(&self.sender) {
            log::debug!("Received look from player: {}", sending_player.username);
            // TODO: Again, what if they're in a non-existent room or something
            if let Some(room) = world.get_player_room(&sending_player) {
                sending_player.game_message(GameMessage::Look(format!(
                    "{}\n{}",
                    room.name, room.description
                )));
            }
        }
    }
}
