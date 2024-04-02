use crate::{
    message::{GameMessage, RawCommand},
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

pub async fn handle_connection(
    players: Players,
    mut stream: TcpStream,
    game_sender: mpsc::Sender<RawCommand>,
) {
    // Generate a communication channel
    let (player_sender, mut player_receiver) = mpsc::unbounded_channel();

    // Implement the login flow here - such as getting a user name

    // Create a new player instance
    let player = Arc::new(Player::new(players.clone(), player_sender));

    // Add the player to the connected players map
    players.write().unwrap().insert(player.id, player.clone());

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

                            // Send the raw message to the game annotated with the player ID
                            let raw_command = RawCommand::new(player.id, message.to_string());
                            game_sender.send(raw_command).await;
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
                    GameMessage::Gossip(content, sending_user) => {
                        log::debug!(
                           "Player {} received gossip from game: {} - {}",
                          player.id,
                           sending_user,
                         content
                    );
                        let response = format!("Gossip <{}>: {}\r\n", sending_user, content);
                        stream.write_all(response.as_bytes()).await.unwrap();
                }
                    GameMessage::NotParsed => {
                        let response = "Arglebargle, glop-glyf!?!?!\r\n";
                        stream.write_all(response.as_bytes()).await.unwrap();
                    }
            }
            }
            }
        }
    }

    // Remove the player from the connected players map when the connection is closed
    players.write().unwrap().remove(&player.id);
    log::info!("Player {} disconnected", player.id);
}
