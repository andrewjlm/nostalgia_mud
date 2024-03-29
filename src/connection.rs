use crate::{
    message::{GameMessage, PlayerMessage},
    player::{Player, Players},
    telnet::{parse_telnet_event, TelnetEvent},
};
use log;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
};

fn interpret_message(message: &str) -> Option<PlayerMessage> {
    // Get the first word (assumed to be the command)
    let mut parts = message.split_whitespace();

    match parts.next() {
        Some(cmd) => {
            // NOTE: We don't care about casing of our command
            let cmd = cmd.to_lowercase();
            let rest = parts.next().unwrap_or("").trim();

            match cmd.as_str() {
                "." | "gossip" => {
                    if rest.is_empty() {
                        // TODO: Should this alert somehow?
                        None
                    } else {
                        Some(PlayerMessage::Gossip(rest.to_string()))
                    }
                }
                // TODO: Under what circumstances can this happen?
                _ => None,
            }
        }
        None => None,
    }
}

pub async fn handle_connection(
    players: Players,
    mut stream: TcpStream,
    game_sender: mpsc::Sender<PlayerMessage>,
) {
    // Generate a communication channel
    let (player_sender, mut player_receiver) = mpsc::unbounded_channel();

    // Implement the login flow here - such as getting a user name

    // Create a new player instance
    let player = Arc::new(Player::new(players.clone(), player_sender));

    // Add the player to the connected players map
    players.lock().unwrap().insert(player.id, player.clone());

    log::info!("New player connected: {:?}", player.username);

    let mut buffer = [0; 1024];
    let mut telnet_buffer = Vec::new();

    // TODO: Buffered reading?
    loop {
        tokio::select! {
                bytes_read = stream.read(&mut buffer) => {

                let bytes_read = bytes_read.unwrap();

                if bytes_read == 0 {
                    break;
                }

                telnet_buffer.extend_from_slice(&buffer[..bytes_read]);

                while let Some(event) = parse_telnet_event(&mut telnet_buffer) {
                    match event {
                        TelnetEvent::Data(bytes) => {
                            let message = String::from_utf8_lossy(&bytes);
                            log::debug!(
                                "Received message from player {}: {}",
                                player.id,
                                message.trim()
                            );
                            // Interpret the message
                            let parsed_message = interpret_message(&message);

                            // If we were able to parse it, send it to the game
                            if let Some(player_message) = parsed_message {
                                game_sender.send(player_message).await;
                            } else {
                                // TODO: Tell the player we didn't understand them
                                log::debug!(
                                    "Unable to parse message from player {}: {}",
                                    player.id,
                                    &message.trim()
                                );
                            }
                        }
                        _ => {
                            // TODO: Figure out if we actually need to handle - wondering about 244
                            // (disconnect?)
                            log::warn!("Received unhandled TelnetEvent: {:?}", event);
                        }
                    }
                }
            }
            game_message = player_receiver.recv() => {
                if let Some(message) = game_message {
                 match message {
                    GameMessage::Gossip(content) => {
                        log::debug!(
                           "Player {} received gossip from game: {}",
                          player.id,
                         content
                    );
                        let response = format!("Gossip: {}\r\n", content);
                        stream.write_all(response.as_bytes()).await.unwrap();
                }
            }
            }
            }
        }
    }

    // Remove the player from the connected players map when the connection is closed
    players.lock().unwrap().remove(&player.id);
    log::info!("Player {} disconnected", player.id);
}
