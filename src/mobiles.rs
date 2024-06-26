#[derive(Clone)]
pub struct Mobile {
    // TODO: Type aliases for all these IDs...
    pub id: u32,
    pub keywords: Vec<String>,
    pub room_description: String,
}

pub struct MobileInstance {
    pub id: u32,
    pub template: Mobile,
    pub current_room: u32,
}
