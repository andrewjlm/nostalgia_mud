use log;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::{
    net::TcpListener,
    sync::mpsc,
    time::{sleep, Duration},
};

mod connection;
mod message;
mod player;
mod telnet;

use connection::handle_connection;
use message::{GameMessage, PlayerMessage};

// TODO Some sort of structured logging ideally that would associate things like the game loop and
// various connection attributes

#[tokio::main]
async fn main() {
    // Start logging
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:4073").await.unwrap();
    log::info!("Telnet server started on localhost:4073");

    let players: player::Players = Arc::new(Mutex::new(HashMap::new()));

    // Channel shared among clients and the game loop
    let (game_sender, game_receiver) = mpsc::channel(32);

    tokio::spawn(game_loop(players.clone(), game_receiver));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let players_clone = players.clone();
        let sender_clone = game_sender.clone();
        tokio::spawn(async move {
            handle_connection(players_clone, stream, sender_clone).await;
        });
    }
}

async fn game_loop(players: player::Players, mut receiver: mpsc::Receiver<PlayerMessage>) {
    log::info!("Game loop spawned");
    loop {
        // Game logic goes here
        // Update game state, send messages to players, etc.
        // Access players map using players.lock().unwrap()
        // TODO: Figure out proper tick length
        while let Some(message) = receiver.recv().await {
            match message {
                PlayerMessage::Gossip(content) => {
                    log::debug!("Received gossip from player: {}", content.trim());

                    for player in players.lock().unwrap().values() {
                        log::debug!("Sending gossip to player {}", player.username);
                        player.game_message(GameMessage::Gossip(content.clone()));
                    }
                }
            }
        }
        sleep(Duration::from_millis(1000)).await;
        log::debug!("Tick!");
    }
}
