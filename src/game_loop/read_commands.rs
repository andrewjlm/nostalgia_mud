use crate::{
    actions::{self, PlayerAction},
    message::{GameMessage, PlayerMessage, RawCommand},
    player::Players,
    world::World,
};
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn read_commands(
    players: &Players,
    world: &Arc<World>,
    receiver: &mut mpsc::Receiver<RawCommand>,
) {
    // Game logic goes here
    // Update game state, send messages to players, etc.
    // Access players map using players.read().unwrap()
    while let Some(raw_command) = receiver.recv().await {
        // Interpret the command
        let command = raw_command.interpret();

        if let Some(player_message) = command {
            // TODO: Could we further standardize the interface? Make every action take a sender
            // and a list of arguments, then somehow do a lookup to a function in a HashMap or
            // something?
            match player_message {
                PlayerMessage::Gossip(content) => {
                    let action = actions::GossipAction {
                        sender: raw_command.sender(),
                        content,
                    };
                    action.perform(&players, &world);
                }
                PlayerMessage::Say(content) => {
                    let action = actions::SayAction {
                        sender: raw_command.sender(),
                        content,
                    };
                    action.perform(&players, &world);
                }
                PlayerMessage::Look => {
                    let action = actions::LookAction {
                        sender: raw_command.sender(),
                    };
                    action.perform(&players, &world);
                }
                PlayerMessage::Move(direction) => {
                    let action = actions::MoveAction {
                        sender: raw_command.sender(),
                        direction,
                    };
                    action.perform(&players, &world);
                }
                PlayerMessage::Contextual(command, arguments) => {
                    log::debug!(
                        "Failed to parse potential contextual player message: {} - {}",
                        command,
                        arguments
                    );
                    if let Some(sending_player) = players.read().get(&raw_command.sender()) {
                        sending_player.game_message(GameMessage::NotParsed);
                    }
                }
            }
        } else {
            if let Some(sending_player) = players.read().get(&raw_command.sender()) {
                log::debug!("Failed to parse player message: {:?}", raw_command);
                sending_player.game_message(GameMessage::NotParsed);
            }
        }
    }
}
