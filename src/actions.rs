use crate::{player::Players, world::World};

mod communication;
mod look;
mod movement;

pub use communication::*;
pub use look::*;
pub use movement::*;

// TODO: Should Players just be a part of World...
pub trait PlayerAction {
    fn perform(&self, players: &Players, world: &World);
}
