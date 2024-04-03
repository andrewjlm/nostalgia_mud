use std::collections::HashMap;

pub struct Room {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub exits: HashMap<String, u32>,
}

impl Room {
    pub fn new(id: u32, name: &str, description: &str) -> Room {
        Room {
            id,
            name: name.to_string(),
            description: description.to_string(),
            exits: HashMap::new(),
        }
    }

    pub fn add_exit(&mut self, direction: &str, destination_id: u32) {
        self.exits.insert(direction.to_string(), destination_id);
    }

    pub fn remove_exit(&mut self, direction: &str) {
        // TODO: Not sure if we really need this
        self.exits.remove(direction);
    }

    pub fn get_exit(&self, direction: &str) -> Option<&u32> {
        self.exits.get(direction)
    }
}

pub fn get_sample_rooms() -> Vec<Room> {
    let mut rooms = Vec::new();

    let mut room1 = Room::new(1, "Cozy Cottage", "A small, quaint cottage nestled in the countryside. The warm fireplace and cozy furnishings make it a welcoming place to rest.");
    room1.add_exit("north", 2);
    room1.add_exit("east", 5);
    rooms.push(room1);

    let mut room2 = Room::new(2, "Enchanted Forest", "Towering trees stretch towards the sky, their branches creating a lush, green canopy. The sound of a nearby stream and the chirping of birds create a serene atmosphere.");
    room2.add_exit("south", 1);
    room2.add_exit("west", 4);
    rooms.push(room2);

    let mut room3 = Room::new(3, "Dusty Attic", "The attic is filled with old trunks, cobwebs, and the musty smell of forgotten memories. Rays of sunlight filter through the dirty windows, casting a warm glow over the space.");
    room3.add_exit("down", 4);
    rooms.push(room3);

    let mut room4 = Room::new(4, "Abandoned Mine", "The entrance to the mine is dark and foreboding, with the sound of dripping water echoing through the tunnel. The air is thick with the scent of damp earth and forgotten treasures.");
    room4.add_exit("east", 5);
    room4.add_exit("west", 2);
    room4.add_exit("down", 7);
    rooms.push(room4);

    let mut room5 = Room::new(5, "Bustling Marketplace", "The marketplace is a lively, crowded place, with merchants shouting their wares and the aroma of exotic spices filling the air. Colorful fabrics and stalls selling all manner of goods line the cobblestone streets.");
    room5.add_exit("north", 8);
    room5.add_exit("south", 1);
    room5.add_exit("east", 6);
    room5.add_exit("west", 4);
    rooms.push(room5);

    let mut room6 = Room::new(6, "Serene Lake", "The still, crystal-clear waters of the lake reflect the surrounding forest and mountains, creating a peaceful and tranquil atmosphere. The gentle lapping of the waves and the calls of waterfowl add to the calming ambiance.");
    room6.add_exit("north", 10);
    room6.add_exit("south", 5);
    rooms.push(room6);

    let mut room7 = Room::new(7, "Mysterious Crypt", "The air in the crypt is thick and oppressive, with the faint sound of wind whispering through the ancient, crumbling walls. The dim lighting casts eerie shadows, and the atmosphere is heavy with a sense of foreboding.");
    room7.add_exit("up", 4);
    room7.add_exit("down", 9);
    rooms.push(room7);

    let mut room8 = Room::new(8, "Grand Castle Hall", "The high ceilings and ornate decor of the castle hall are impressive and awe-inspiring. Tapestries and chandeliers adorn the space, and the sound of your footsteps echoes through the cavernous room.");
    room8.add_exit("north", 10);
    room8.add_exit("south", 5);
    room8.add_exit("east", 9);
    room8.add_exit("west", 7);
    rooms.push(room8);

    let mut room9 = Room::new(9, "Underground Lab", "The underground lab is a maze of corridors and dimly lit chambers, filled with the hum of machinery and the faint glow of computer screens. The air is cool and damp, and the atmosphere is one of scientific curiosity and experimentation.");
    room9.add_exit("up", 8);
    room9.add_exit("north", 7);
    room9.add_exit("south", 10);
    rooms.push(room9);

    let mut room10 = Room::new(10, "Celestial Observatory", "The observatory is a spacious, domed room with large windows that offer a breathtaking view of the night sky. Telescopes and astronomical equipment line the walls, and the air is filled with the sense of wonder and discovery.");
    room10.add_exit("north", 6);
    room10.add_exit("south", 9);
    room10.add_exit("down", 8);
    rooms.push(room10);

    rooms
}
