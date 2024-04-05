use crate::{player::Players, world::World};

mod communication;
mod look;
mod movement;

pub use communication::*;
pub use look::*;
pub use movement::*;

// TODO: Should Players just be a part of World...
pub trait PlayerAction: std::fmt::Debug {
    // TODO: Is this supposed to work or is there something I'm missing? For now doing this at the
    // level of each action...
    //#[tracing::instrument(skip(players, world))]
    fn perform(&self, players: &Players, world: &World);
}
