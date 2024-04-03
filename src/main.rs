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
mod room;
mod telnet;
mod world;

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
    let world = Arc::new(world::get_sample_world());

    // Channel shared among clients and the game loop
    let (game_sender, game_receiver) = mpsc::channel(32);

    tokio::spawn(game_loop(players.clone(), world.clone(), game_receiver));

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

async fn read_commands(
    players: &player::Players,
    world: &Arc<world::World>,
    receiver: &mut mpsc::Receiver<RawCommand>,
) {
    // Game logic goes here
    // Update game state, send messages to players, etc.
    // Access players map using players.read().unwrap()
    while let Some(raw_command) = receiver.recv().await {
        // Interpret the command
        let command = raw_command.interpret();

        if let Some(mut sending_player) = players.write().unwrap().get_mut(&raw_command.sender()) {
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
                    PlayerMessage::Look => {
                        log::debug!("Received look from player: {}", sending_player.username);
                        // TODO: Again, what if they're in a non-existent room or something
                        if let Some(room) = world.get_player_room(&sending_player) {
                            sending_player.game_message(GameMessage::Look(format!(
                                "{}\n{}",
                                room.name, room.description
                            )));
                        }
                    }
                    PlayerMessage::Move(direction) => {
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
                        }
                        // TODO: Probably tell the user if there is no exit
                    }
                    PlayerMessage::Contextual(command, arguments) => {
                        log::debug!(
                            "Failed to parse potential contextual player message: {} - {}",
                            command,
                            arguments
                        );
                        sending_player.game_message(GameMessage::NotParsed);
                    }
                }
            } else {
                log::debug!("Failed to parse player message: {:?}", raw_command);
                sending_player.game_message(GameMessage::NotParsed);
            }
        } else {
            // NOTE: I'm not sure if this can happen
            log::warn!(
                "Unable to get mutable sending player for command: {:?}",
                raw_command
            );
        }
    }
}

async fn game_loop(
    players: player::Players,
    world: Arc<world::World>,
    mut receiver: mpsc::Receiver<RawCommand>,
) {
    log::info!("Game loop spawned");
    loop {
        tokio::select! {
            _game_clock = tick() => {},
            _commands = read_commands(&players, &world, &mut receiver) => {},
        }
    }
}
