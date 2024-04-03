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

async fn login(stream: &mut TcpStream, players: Players) -> Option<String> {
    loop {
        stream.write_all(b"Enter a username: ").await.unwrap();

        let mut username_buffer = [0; 1024];
        let bytes_read = stream.read(&mut username_buffer).await.unwrap();

        if bytes_read == 0 {
            // TODO: Other info eg IPAddress?
            log::info!("Client disconnected during login");
            return None;
        }

        let username = String::from_utf8_lossy(&username_buffer[..bytes_read])
            .trim()
            .to_string();

        if players
            .read()
            .unwrap()
            .values()
            .any(|p| p.username == username)
        {
            stream
                .write_all(b"Username already taken. Try again.\r\n")
                .await
                .unwrap();
        } else {
            let welcome = format!("Welcome, {}\r\n", username);
            stream.write_all(welcome.as_bytes()).await.unwrap();
            return Some(username);
        }
    }
}

pub async fn handle_connection(
    players: Players,
    mut stream: TcpStream,
    game_sender: mpsc::Sender<RawCommand>,
) {
    // Generate a communication channel
    let (player_sender, mut player_receiver) = mpsc::unbounded_channel();

    // Dispatch to the login flow
    if let Some(username) = login(&mut stream, players.clone()).await {
        // Create a new player instance
        let player = Arc::new(Player::new(username, players.clone(), player_sender));

        // Add the player to the connected players map
        players.write().unwrap().insert(player.id, player.clone());

        log::info!("New player connected: {:?}", player.username);

        let mut buffer = [0; 1024];
        let mut telnet_buffer = Vec::new();

        // TODO: Buffered reading?
        loop {
            tokio::select! {
                bytes_read = stream.read(&mut buffer) => {
                    match bytes_read {
                        Ok(0) => {
                            // Connection closed by remote peer
                            log::info!("Client {} disconnected", &player.id);
                            players.write().unwrap().remove(&player.id);
                            return
                        }
                        Ok(n) => {
                            telnet_buffer.extend_from_slice(&buffer[..n]);

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
                        Err(e) => {
                            log::error!("Error reading from stream: {}", e);
                            players.write().unwrap().remove(&player.id);
                            return
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
    } else {
        log::info!("Client disconnected during login");
    }
}
