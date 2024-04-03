use crate::{message::RawCommand, player::Players, world::World};
use std::sync::Arc;
use tokio::sync::mpsc;

mod read_commands;
mod tick;

use read_commands::read_commands;
use tick::tick;

pub async fn game_loop(
    players: Players,
    world: Arc<World>,
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
