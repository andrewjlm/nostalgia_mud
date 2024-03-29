use crate::{
    message::Message,
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
    game_sender: mpsc::Sender<Message>,
) {
    // Generate a communication channel
    let (player_sender, player_receiver) = mpsc::channel(32);

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
        let bytes_read = stream.read(&mut buffer).await.unwrap();

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
                    // TODO: For now made player sender public but might be cleaner to push down
                    // somehow?
                    game_sender
                        .send(Message::PlayerMessage(message.into_owned()))
                        .await;
                    stream.write_all(b"Message received\r\n").await.unwrap();
                }
                _ => {
                    // TODO: Figure out if we actually need to handle - wondering about 244
                    // (disconnect?)
                    println!("{:?}", event);
                }
            }
        }
    }

    // Remove the player from the connected players map when the connection is closed
    players.lock().unwrap().remove(&player.id);
    println!("Player {} disconnected", player.id);
}
