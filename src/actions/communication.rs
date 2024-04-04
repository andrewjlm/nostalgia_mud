use crate::{
    actions::PlayerAction,
    message::GameMessage,
    player::{Player, Players},
    world::World,
};

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

        send_targeted_message(
            players,
            GameMessage::Gossip(self.content.clone(), sending_player_username.clone()),
            // Send to everyone so don't bother with a real predicate
            |_| true,
        );
    }
}

// Utility function to send a message to some subset of players
// TODO: Should this be even more generic - use outside of player communications?
fn send_targeted_message<F>(players: &Players, message: GameMessage, predicate: F)
where
    F: FnMut(&(&u32, &Player)) -> bool,
{
    for (_id, player) in players.read().iter().filter(predicate) {
        let message = message.clone();
        player.game_message(message);
    }
}
