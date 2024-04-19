use crate::{mobiles::Mobile, reset::ResetCommand, room::Room};

pub struct Area {
    pub rooms: Vec<Room>,
    pub mobiles: Vec<Mobile>,
    pub resets: Vec<ResetCommand>,
}
