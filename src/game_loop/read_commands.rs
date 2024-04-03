use crate::{
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
            match player_message {
                PlayerMessage::Gossip(content) => {
                    let sending_player_username = {
                        if let Some(sending_player) = players.read().get(&raw_command.sender()) {
                            sending_player.username.clone()
                        } else {
                            continue;
                        }
                    };

                    log::debug!(
                        "Received gossip from player: {} - {}",
                        sending_player_username,
                        content.trim()
                    );

                    for player in players.read().values() {
                        log::debug!("Sending gossip to player {}", player.username);
                        player.game_message(GameMessage::Gossip(
                            content.clone(),
                            sending_player_username.clone(),
                        ));
                    }
                }
                PlayerMessage::Look => {
                    if let Some(sending_player) = players.write().get_mut(&raw_command.sender()) {
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
                PlayerMessage::Move(direction) => {
                    if let Some(sending_player) = players.write().get_mut(&raw_command.sender()) {
                        log::debug!(
                            "Received move from player: {} - {:?}",
                            sending_player.username,
                            direction
                        );

                        if let Some(exit) = world
                            .get_player_room(&sending_player)
                            .and_then(|player_room| player_room.get_exit(&direction.to_string()))
                        {
                            log::debug!("Moving player {} to {}", direction, exit);
                            sending_player.move_to_room(*exit);
                            // TODO: There is probably some better way to architect so that "Look"
                            // logic is only written once
                            if let Some(room) = world.get_player_room(&sending_player) {
                                sending_player.game_message(GameMessage::Look(format!(
                                    "{}\n{}",
                                    room.name, room.description
                                )));
                            }
                            // TODO: Make configurable if we send the full room or a "glance" (just
                            // the room name and exits)
                        } else {
                            sending_player.game_message(GameMessage::NoExit(direction));
                        }
                    }
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
