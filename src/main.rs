use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use patharg::InputArg;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::mpsc};

extern crate merc_parser;

mod actions;
mod area;
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
use world::World;

#[derive(Parser, Debug, Deserialize, Serialize)]
#[command(version, about, long_about = None)]
struct Args {
    // Path to config file
    #[arg(short, long)]
    #[clap(value_parser)]
    config_file: InputArg,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Config {
    areas: Vec<PathBuf>,
    port: u16,
}

#[tokio::main]
async fn main() {
    // TODO: Implement shutdown via ctrl-c or a command from a wiz
    // https://tokio.rs/tokio/topics/shutdown
    // Start logging
    let subscriber = tracing_subscriber::fmt::init();

    // Parse the CLI argument
    let args = Args::parse();
    // Read the config file
    let config: Config = Figment::new()
        .merge(Toml::string(
            // TODO: This is ugly
            args.config_file.read_to_string().unwrap().as_str(),
        ))
        .extract()
        .unwrap();

    // TODO: Debug tracing of config load

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .unwrap();
    tracing::info!(port = config.port, "Starting Telnet server");

    let players = player::Players::new();
    let mut world = World::new();

    for area_file in config.areas {
        tracing::info!(filename = ?area_file, "Loading area file");
        let area_file = File::open(area_file).unwrap();
        let area = merc::load_area_file(area_file);
        world.add_area(area);
    }

    // Call an initial reset of the world to place all the mobs and objects
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
