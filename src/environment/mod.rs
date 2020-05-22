pub mod map;
use map::tiles::Tile;
use map::*;

use map::{ // List of map gen variants go here
    rectangles::rectangles,
    modifiers::*,
};

pub mod spawner;
use spawner::spawner;

use crate::{ Tcod, initialise_fov };
use crate::graphics::gui::Messages;
use crate::objects::{ Object, Character };
use crate::graphics::gen_colors;

use std::collections::HashMap;

use rand::*;

use serde::{ Serialize, Deserialize };

use tcod::map::FovAlgorithm;
use tcod::colors::*;

// Determines Field-Of-View
pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic; // Default FOV Algorithm
pub const FOV_LIGHT_WALLS: bool = true;
pub const TORCH_RADIUS: i32 = 10;

// Size of the map
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 43;

// Dungeon room limitations
const ROOM_MAX_SIZE: i32 = 12;
const ROOM_MIN_SIZE: i32 = 4;
const MAX_ROOMS: i32 = 18;

pub type Map = Vec<Vec<Tile>>;

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub map: Map,
    pub messages: Messages,
    pub dungeon_level: u32,
}

impl Game {
    pub fn new(mut characters: &mut Vec<Character>, mut items: &mut HashMap<i32, Object>, player: &mut Object) -> Game {
        let map = make_map(player, &mut characters, &mut items, 1);
        Game {
            map: map,
            messages: Messages::new(),
            dungeon_level: 1,
        }
    }
}

pub fn next_level(
    tcod: &mut Tcod,
    game: &mut Game,
    player: &mut Object,
    characters: &mut Vec<Character>,
    items: &mut HashMap<i32, Object>
) {
    game.messages.add(
        "You take a moment to rest, and recover your strength.",
        GREEN,
    );
    let heal_hp = player.fighter.map_or(0, |f| f.max_hp / 2);
    player.heal(heal_hp);

    game.messages.add(
        "After taking a moment to rest, you dive deeper into the caverns...",
        RED,
    );

    // Keeps track of dungeon depth, makes new dungeon map, and generates FOV map.
    game.dungeon_level += 1;
    game.map = make_map(player, characters, items, game.dungeon_level);
    initialise_fov(tcod, &game.map);
}

pub fn make_map(
    mut player: &mut Object,
    mut characters: &mut Vec<Character>,
    mut items: &mut HashMap<i32, Object>,
    level: u32,
) -> Map {
    // Generate dungeon floor colors alongside variation
    let colors = gen_colors();

    // Fill map with wall tiles
    let mut map = vec![vec![Tile::wall(&colors); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // Creates vector to store rooms
    let mut rooms = vec![];

    // Randomly decides which type of map to use, and generates it.
    let map_type = rand::thread_rng().gen_range(1, 5);
    //let map_type = 1;
    println!("{}", map_type);
    let needs_corridors = match map_type {
        // Standard rectangles map
        1 => {
            rectangles(&mut rooms, &mut map, &colors, &mut player);
            true
        },

        // Rectangles map with an open area in the middle
        2 => {
            rectangles(&mut rooms, &mut map, &colors, &mut player);
            caved_in(&mut map, &colors);
            true
        },

        // Rectangles map with the drunken miner modifier
        3 => {
            rectangles(&mut rooms, &mut map, &colors, &mut player);
            mine_drunkenly(&rooms, &mut map, &colors);
            true
        },

        // Rectangles map with the open area and drunken miner modifiers
        _ => {
            rectangles(&mut rooms, &mut map, &colors, &mut player);
            mine_drunkenly(&rooms, &mut map, &colors);
            caved_in(&mut map, &colors);
            true
        },
    };

    // Connects rooms together with horizontal/vertical tunnels.
    if needs_corridors {
        create_tunnels(&rooms, &mut map, &colors);
    }

    spawner(&rooms, &mut items, &map, &mut characters, level);

    map
}
