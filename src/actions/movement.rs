use crate::{
    actions::PlayerAction,
    message::{Direction, GameMessage},
    player::Players,
    world::World,
};

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
                log::debug!("Moving player {} to {}", &self.direction, exit);
                sending_player.move_to_room(*exit);
            } else {
                sending_player.game_message(GameMessage::NoExit(self.direction));
            }
        }
    }
}
