use clap::Parser;
use clio::InputPath;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::mpsc};

extern crate merc_parser;

mod actions;
mod connection;
mod game_loop;
mod merc;
mod message;
mod mobiles;
mod player;
mod reset;
mod room;
mod world;

use connection::handle_connection;
use game_loop::game_loop;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Area filename to load
    #[arg(short, long)]
    #[clap(value_parser)]
    area_file: InputPath,
    #[arg(short, long)]
    port: u16,
}

#[tokio::main]
async fn main() {
    // TODO: Implement shutdown via ctrl-c or a command from a wiz
    // https://tokio.rs/tokio/topics/shutdown
    // Start logging
    let subscriber = tracing_subscriber::fmt::init();

    // Parse the CLI arguments
    let args = Args::parse();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await
        .unwrap();
    tracing::info!(port = args.port, "Starting Telnet server");

    let players = player::Players::new();

    // Open the provided area file
    tracing::info!(filename = %args.area_file, "Loading area file");
    let area_file = args.area_file.open().unwrap();
    let midgard = merc::load_area_file(area_file);
    let mut world = midgard;
    world.reset();

    // TODO: Should ensure players always span into a particular room that cannot fail

    // Channel shared among clients and the game loop
    let (game_sender, game_receiver) = mpsc::channel(32);

    tokio::spawn(game_loop(players.clone(), world, game_receiver));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let sender_clone = game_sender.clone();
        let players_clone = players.clone();
        tokio::spawn(async move {
            handle_connection(players_clone, stream, sender_clone).await;
        });
    }
}
