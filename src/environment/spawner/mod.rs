pub mod character_spawns;
use character_spawns::{ room_characters, no_room_characters };

pub mod item_spawns;
use item_spawns::{ room_items, no_room_items };

use crate::environment::{ Map, MAP_WIDTH, MAP_HEIGHT };
use crate::environment::map::Rect;
use crate::objects::{ Object, Character };

use std::collections::HashMap;
use rand::*;

use tcod::colors::*;

pub fn rooms_spawner(
    rooms: &Vec<Rect>,
    items: &mut HashMap<i32, Object>,
    map: &Map,
    characters: &mut Vec<Character>,
    level: u32
) {
    // Ensures that there are no existing entities in the character, or item collections.
    characters.clear();
    items.clear();

    // Keeps track of total items spawned on a map.
    let mut item_counter = 1;

    for room in rooms {
        room_characters(*room, &map, characters, level);
        room_items(*room, items, &map, characters, &mut item_counter, level);
    }

    // Create stairs at the center of the last room.
    let (last_room_center_x, last_room_center_y) = rooms[rooms.len() - 1].center();
    create_stairs(items, last_room_center_x, last_room_center_y);
}

pub fn no_rooms_spawner(
    items: &mut HashMap<i32, Object>,
    map: &Map,
    characters: &mut Vec<Character>,
    level: u32
) {
    // Ensures that there are no existing entities in the character, or item collections.
    characters.clear();
    items.clear();

    // Keeps track of total items spawned on a map.
    let mut item_counter = 1;

    no_room_characters(&map, characters, level);
    no_room_items(items, &map, characters, &mut item_counter, level);

    let mut stairs_placed = true;
    while stairs_placed {
        let x = rand::thread_rng().gen_range(1, MAP_WIDTH - 1);
        let y = rand::thread_rng().gen_range(1, MAP_HEIGHT - 1);

        if map[x as usize][y as usize].empty == true {
            create_stairs(items, x, y);
            stairs_placed = false;
        }
    }
}

pub struct Transition {
    pub level: u32,
    pub value: u32,
}

pub fn from_dungeon_level(table: &[Transition], level: u32) -> u32 {
    table.iter().rev()
        .find(|transition| level >= transition.level)
        .map_or(0, |transition| transition.value)
}

pub fn create_stairs(items: &mut HashMap<i32, Object>, x: i32, y: i32) {
    let stairs = Object {
        x: x,
        y: y,
        char: '<',
        color: WHITE,
        name: "Stairs".into(),
        blocks: false,
        alive: false,
        corpse_type: "Stairs".into(),
        fighter: None,
        ai: None,
        item: None,
        level: 1,
        always_visible: true,
    };

    let mut stairs_id = 1; // Sets up id for stairs to use in items hashmap.
    for _ in 0..items.len() {
        if items.contains_key(&stairs_id) {
            stairs_id += 1;
        } else {
            break;
        }
    }
    items.insert(stairs_id, stairs); // Finally, inserts stairs into the items hashmap.
}
