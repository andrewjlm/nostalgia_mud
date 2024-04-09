use crate::{
    actions::{self, PlayerAction},
    message::{ConnectionMessage, GameMessage, PlayerMessage, RawCommand},
    player::Players,
    world::World,
};
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn read_commands(
    players: &Players,
    world: &Arc<World>,
    receiver: &mut mpsc::Receiver<ConnectionMessage>,
) {
    // Game logic goes here
    // Update game state, send messages to players, etc.
    // Access players map using players.read().unwrap()
    while let Some(msg) = receiver.recv().await {
        match msg {
            ConnectionMessage::AddPlayer(player) => {
                let player_id = player.id;
                tracing::info!("Adding new player {}: '{}'", player_id, player.username);
                players.write().insert(player_id, player.clone());
            }
            ConnectionMessage::RemovePlayer(player_id) => {
                // Remove the player from the connected players map when the connection is closed
                // TODO: Probably some sort of check here that a disconnecting player is actually
                // in the map
                let player = players.write().remove(&player_id).unwrap();
                tracing::info!("Removed player {}: '{}'", player_id, player.username);
            }
            ConnectionMessage::PlayerCommand(sender_id, message) => {
                // A potential command from the player we need to interpret
                let command = RawCommand::new(sender_id, message);

                tracing::debug!("Received possible command '{:?}'", command);

                if let Some(player_message) = command.interpret() {
                    // TODO: Could we further standardize the interface? Make every action take a sender
                    // and a list of arguments, then somehow do a lookup to a function in a HashMap or
                    // something?
                    match player_message {
                        PlayerMessage::Gossip(content) => {
                            let action = actions::GossipAction {
                                sender: sender_id,
                                content,
                            };
                            action.perform(&players, &world);
                        }
                        PlayerMessage::Say(content) => {
                            let action = actions::SayAction {
                                sender: sender_id,
                                content,
                            };
                            action.perform(&players, &world);
                        }
                        PlayerMessage::Look => {
                            let action = actions::LookAction { sender: sender_id };
                            action.perform(&players, &world);
                        }
                        PlayerMessage::Move(direction) => {
                            let move_action = actions::MoveAction {
                                sender: sender_id,
                                direction,
                            };
                            // TODO: Make this optional
                            let look_action = actions::LookAction { sender: sender_id };
                            move_action.perform(&players, &world);
                            look_action.perform(&players, &world);
                        }
                        PlayerMessage::Contextual(command, arguments) => {
                            tracing::debug!(
                                "Failed to parse potential contextual player message: {} {}",
                                command,
                                arguments
                            );
                            if let Some(sending_player) = players.read().get(&sender_id) {
                                sending_player.game_message(GameMessage::NotParsed);
                            }
                        }
                    }
                } else {
                    if let Some(sending_player) = players.read().get(&sender_id) {
                        tracing::debug!("Failed to parse player message: {:?}", command);
                        sending_player.game_message(GameMessage::NotParsed);
                    }
                }
            }
        }
    }
}
//if let Some(sending_player) = players.read().get(&raw_command.sender()) {
//sending_player.game_message(GameMessage::NotParsed);
//}
//}
//}
//} else {
//if let Some(sending_player) = players.read().get(&raw_command.sender()) {
//tracing::debug!("Failed to parse player message: {:?}", raw_command);
//sending_player.game_message(GameMessage::NotParsed);
//}
//}
//}
