// Implement for compatability with Diku/Merc style areas
use crate::{room::Room, world::World};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use merc_parser::parse_area_file;

pub fn load_area_file(filename: &str) -> World {
    // TODO: More descriptive error?
    let file = File::open(filename).expect("Unable to open file.");
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer);

    let mut world = World::new();

    let (_, parsed_area) = parse_area_file(&buffer).unwrap();

    // Iterate over the rooms in the file and turn into our internal representation
    for r in parsed_area.rooms {
        let mut room = Room::new(r.vnum, &r.room_name, &r.description);

        for d in r.doors {
            // Map door indices to directions
            let direction = match d.direction {
                0u8 => "north",
                1u8 => "east",
                2u8 => "south",
                3u8 => "west",
                4u8 => "up",
                5u8 => "down",
                _ => panic!(),
            };
            room.add_exit(direction, d.to_room);
        }

        world.add_room(room);
    }

    world
}
