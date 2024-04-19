// Implement for compatability with Diku/Merc style areas
use crate::{area::Area, mobiles::Mobile, reset::ResetCommand, room::Room, world::World};
use std::io::Read;

use merc_parser::{parse_area_file, RomResetCommand};

pub fn load_area_file<R: Read>(mut area_file: R) -> Area {
    let mut buffer = String::new();
    area_file.read_to_string(&mut buffer);

    let mut world = World::new();

    let (_, parsed_area) = parse_area_file(&buffer).unwrap();
    tracing::info!(
        area_name = parsed_area.metadata.display_name,
        area_author = parsed_area.metadata.author,
        "Loaded ROM area file"
    );

    // Iterate over the rooms in the file and turn into our internal representation
    let mut rooms: Vec<Room> = Vec::with_capacity(parsed_area.rooms.len());
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

        tracing::debug!(
            room_id = room.id,
            room_name = room.name,
            "Adding room to world"
        );
        rooms.push(room);
    }

    // Iterate over the mobs in the file and turn into our internal representation
    let mut mobiles: Vec<Mobile> = Vec::with_capacity(parsed_area.mobiles.len());
    for m in parsed_area.mobiles {
        let mobile = Mobile {
            id: u32::try_from(m.vnum).unwrap(),
            keywords: m.keywords,
            room_description: m.short_description,
        };

        tracing::debug!(
            mobile_id = mobile.id,
            mobile_name = mobile.room_description,
            "Adding mobile template to world"
        );
        mobiles.push(mobile);
    }

    // Same with resets
    let mut resets: Vec<ResetCommand> = Vec::with_capacity(parsed_area.resets.len());
    for r in parsed_area.resets {
        let reset = {
            match r {
                RomResetCommand::LoadMobile(lm) => Some(ResetCommand {
                    mobile_id: u32::try_from(lm.mobile_vnum).unwrap(),
                    room_id: u32::try_from(lm.room_vnum).unwrap(),
                }),
                _ => {
                    tracing::warn!("Unimplemented reset {:?}", r);
                    None
                }
            }
        };

        if let Some(r) = reset {
            tracing::debug!(
                // TODO: Do everything different depending on the type of reset command we have
                r.mobile_id,
                r.room_id,
                "Adding reset to world"
            );

            resets.push(r);
        }
    }

    Area {
        rooms,
        mobiles,
        resets,
    }
}
