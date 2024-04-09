use std::sync::Arc;
use tokio::{net::TcpListener, sync::mpsc};

mod actions;
mod connection;
mod game_loop;
mod message;
mod player;
mod room;
mod world;

use connection::handle_connection;
use game_loop::game_loop;

// TODO Some sort of structured logging ideally that would associate things like the game loop and
// various connection attributes

#[tokio::main]
async fn main() {
    // TODO: Implement shutdown via ctrl-c or a command from a wiz
    // https://tokio.rs/tokio/topics/shutdown
    // Start logging
    let subscriber = tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:4073").await.unwrap();
    tracing::info!("Telnet server started on localhost:4073");

    let players = player::Players::new();
    let world = Arc::new(world::get_sample_world());

    // Channel shared among clients and the game loop
    let (game_sender, game_receiver) = mpsc::channel(32);

    tokio::spawn(game_loop(players.clone(), world.clone(), game_receiver));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let sender_clone = game_sender.clone();
        let players_clone = players.clone();
        tokio::spawn(async move {
            handle_connection(players_clone, stream, sender_clone).await;
        });
    }
}
