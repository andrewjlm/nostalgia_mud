use log;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
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
use message::{GameMessage, PlayerMessage, RawCommand};

// TODO Some sort of structured logging ideally that would associate things like the game loop and
// various connection attributes

#[tokio::main]
async fn main() {
    // Start logging
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:4073").await.unwrap();
    log::info!("Telnet server started on localhost:4073");

    let players: player::Players = Arc::new(RwLock::new(HashMap::new()));

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

async fn tick() {
    // TODO: Figure out proper tick length
    log::debug!("Tick!");
    sleep(Duration::from_millis(1000)).await
}

async fn read_commands(players: &player::Players, receiver: &mut mpsc::Receiver<RawCommand>) {
    // Game logic goes here
    // Update game state, send messages to players, etc.
    // Access players map using players.read().unwrap()
    while let Some(raw_command) = receiver.recv().await {
        // Interpret the command
        let command = raw_command.interpret();

        // Get the sending player
        let sending_player = {
            let players = players.read().unwrap();
            players.get(&raw_command.sender()).cloned()
        };

        if let Some(sending_player) = sending_player {
            if let Some(player_message) = command {
                match player_message {
                    PlayerMessage::Gossip(content) => {
                        log::debug!(
                            "Received gossip from player: {} - {}",
                            sending_player.username,
                            content.trim()
                        );

                        for player in players.read().unwrap().values() {
                            log::debug!("Sending gossip to player {}", player.username);
                            player.game_message(GameMessage::Gossip(
                                content.clone(),
                                sending_player.username.clone(),
                            ));
                        }
                    }
                }
            } else {
                log::debug!("Failed to parse player message: {:?}", raw_command);
                sending_player.game_message(GameMessage::NotParsed);
            }
        } else {
            // NOTE: I'm not sure if this can happen
            log::warn!(
                "Unable to find sending player for command: {:?}",
                raw_command
            );
        }
    }
}

async fn game_loop(players: player::Players, mut receiver: mpsc::Receiver<RawCommand>) {
    log::info!("Game loop spawned");
    loop {
        tokio::select! {
            game_clock = tick() => {},
            commands = read_commands(&players, &mut receiver) => {},
        }
    }
}
