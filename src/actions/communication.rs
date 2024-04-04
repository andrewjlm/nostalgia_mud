use crate::{actions::PlayerAction, message::GameMessage, player::Players, world::World};

pub struct GossipAction {
    pub sender: u32,
    pub content: String,
}

impl PlayerAction for GossipAction {
    fn perform(&self, players: &Players, _world: &World) {
        // Do all the reading from the players map at once
        let sending_player_username = {
            if let Some(sending_player) = players.read().get(&self.sender) {
                sending_player.username.clone()
            } else {
                return;
            }
        };

        log::debug!(
            "Received gossip from player: {} - {}",
            sending_player_username,
            self.content.trim()
        );

        for player in players.read().values() {
            let message =
                GameMessage::Gossip(self.content.clone(), sending_player_username.clone());
            player.game_message(message);
        }
    }
}
