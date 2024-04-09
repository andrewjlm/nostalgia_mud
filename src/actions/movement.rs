use crate::{actions::PlayerAction, message::Direction, player::Players, world::World};

#[derive(Debug)]
pub struct MoveAction {
    pub sender: u32,
    pub direction: Direction,
}

impl PlayerAction for MoveAction {
    fn perform(&self, players: &Players, world: &World) {
        if let Some(sending_player) = players.write().get_mut(&self.sender) {
            // TODO: Deal with locking the world here at some point
            if let Some(exit) = world
                .get_player_room(&sending_player)
                .and_then(|player_room| player_room.get_exit(&self.direction.to_string()))
            {
                tracing::debug!("Moving player {} to {}", &self.direction, exit);
                sending_player.move_to_room(*exit);
            } else {
                // TODO: This will read sort of awkward (eg "You don't see an
                // exit north from here" when we'd probably say "north of
                // here"). Should figure out a way to get consistent.
                let response = format!("You don't see an exit {} from here", self.direction);
                sending_player.game_message(response);
            }
        }
    }
}
