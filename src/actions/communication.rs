use crate::{
    actions::PlayerAction,
    message::GameMessage,
    player::{Player, Players},
    world::World,
};

use stylish::ansi::format as ansi_format;

#[derive(Debug)]
pub struct GossipAction {
    pub sender: u32,
    pub content: String,
}

impl PlayerAction for GossipAction {
    #[tracing::instrument(skip(players, _world), fields(username=tracing::field::Empty))]
    fn perform(&self, players: &Players, _world: &World) {
        // Do all the reading from the players map at once
        let sending_player_username = {
            if let Some(sending_player) = players.read().get(&self.sender) {
                sending_player.username.clone()
            } else {
                return;
            }
        };
        tracing::Span::current().record("username", &sending_player_username);

        tracing::debug!("Performing Gossip: '{}'", self.content.trim());

        send_targeted_message(
            players,
            ansi_format!(
                "{} gossips '{:(fg=cyan)}'",
                sending_player_username,
                self.content
            ),
            // Send to everyone so don't bother with a real predicate
            |_| true,
        );
    }
}

// TODO Further opportunities for consolidation? Everything in here (except Tell) will be the same
// stuff with a different predicate and GameMessage type
#[derive(Debug)]
pub struct SayAction {
    pub sender: u32,
    pub content: String,
}

impl PlayerAction for SayAction {
    #[tracing::instrument(skip(players, _world), fields(username=tracing::field::Empty))]
    fn perform(&self, players: &Players, _world: &World) {
        // Do all the reading from the players map at once
        let (sending_player_username, room) = {
            if let Some(sending_player) = players.read().get(&self.sender) {
                (sending_player.username.clone(), sending_player.current_room)
            } else {
                return;
            }
        };
        tracing::Span::current().record("username", &sending_player_username);

        tracing::debug!("Performing Say: '{}'", self.content.trim());

        send_targeted_message(
            players,
            ansi_format!(
                "{} says '{:(fg=yellow)}'",
                sending_player_username,
                self.content
            ),
            // Send to everyone so don't bother with a real predicate
            |&(_, player)| player.current_room == room,
        );
    }
}

// Utility function to send a message to some subset of players
// TODO: Should this be even more generic - use outside of player communications?
// TODO: It might be useful to be able to trace the predicates being called
// https://boydjohnson.dev/blog/impl-debug-for-fn-type/
#[tracing::instrument(skip(players, predicate))]
fn send_targeted_message<F>(players: &Players, message: String, predicate: F)
where
    F: FnMut(&(&u32, &Player)) -> bool,
{
    for (_id, player) in players.read().iter().filter(predicate) {
        tracing::debug!("Sending to {}", player.username);
        let message = message.clone();
        player.game_message(message);
    }
}
