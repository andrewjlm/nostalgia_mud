use crate::{
    message::{GameMessage, RawCommand},
    player::{Player, Players},
    telnet::{read_from_buffer, TelnetWrapper},
};
use log;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::mpsc};

async fn login(telnet: &mut TelnetWrapper, players: Players) -> Option<String> {
    loop {
        telnet.write_all(b"Enter a username: ").await;

        let mut username_buffer = [0; 1024];

        if let Ok(bytes_read) = telnet.read(&mut username_buffer).await {
            if bytes_read == 0 {
                // TODO: Other info eg IPAddress?
                // TODO: I don't think this is actually working right now
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
                telnet
                    .write_all(b"Username already taken. Try again.\r\n")
                    .await;
            } else {
                let welcome = format!("Welcome, {}\r\n", username);
                telnet.write_all(welcome.as_bytes()).await;
                return Some(username);
            }
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
    let mut telnet = TelnetWrapper::new(stream);

    // Dispatch to the login flow
    if let Some(username) = login(&mut telnet, players.clone()).await {
        // Create a new player instance
        // TODO: Right now everyone ALWAYS starts in the same room. Also, if Room 1 doesn't
        // exist... not sure what happens but presumably bad
        let player = Player::new(username, players.clone(), player_sender, 1);

        // Reserve a copy of the ID for retrieval
        let player_id = player.id;

        // Add the player to the connected players map
        players.write().unwrap().insert(player_id, player.clone());

        log::info!("New player connected: {:?}", player.username);

        let mut buffer = [0; 1024];
        let mut telnet_buffer = Vec::new();

        // TODO: Buffered reading?
        loop {
            // NOTE: Once we have the player in the RwLocked HashMap, we should never use the
            // *original* player so we retrieve it here
            let player = {
                let players_guard = players.read().unwrap();
                players_guard.get(&player_id).cloned()
            };

            if let Some(player) = player {
                tokio::select! {
                    message = read_from_buffer(&mut buffer, &mut telnet_buffer, &mut telnet) => {
                        if let Some(bytes) = message {
                            let message = String::from_utf8_lossy(&bytes);
                            log::debug!(
                                "Received message from player {}: {}",
                                player.id,
                                message.trim()
                            );

                            // Send the raw message to the game annotated with the player ID
                            let raw_command = RawCommand::new(player.id, message.to_string());
                            game_sender.send(raw_command).await;
                        } else {
                            // Remove the player from the connected players map when the connection is closed
                            players.write().unwrap().remove(&player.id);
                            log::info!("Player {} disconnected", player.id);
                            return
                        }
                    }
                    game_message = player_receiver.recv() => {
                        if let Some(message) = game_message {
                            match message {
                                // TODO: Some low hanging fruit for consolidation here, just match and
                                // return the response then write at the end (if we get one)
                                GameMessage::Gossip(content, sending_user) => {
                                    log::debug!(
                                        "Player {} received gossip from game: {} - {}",
                                        player.id,
                                        sending_user,
                                        content
                                    );
                                    let response = format!("Gossip <{}>: {}\r\n", sending_user, content);
                                    telnet.write_all(response.as_bytes()).await;
                                }
                                GameMessage::Look(description) => {
                                    log::debug!(
                                        "Player {} looked, saw {}",
                                        player.id,
                                        description
                                        );
                                    let response = format!("{}\r\n", description);
                                    telnet.write_all(response.as_bytes()).await;
                                }
                                GameMessage::NotParsed => {
                                    let response = "Arglebargle, glop-glyf!?!?!\r\n";
                                    telnet.write_all(response.as_bytes()).await;
                                }
                            }
                        }
                    }
                }
            } else {
                // Player not found, exit the loop
                break;
            }
        }
    }
}
