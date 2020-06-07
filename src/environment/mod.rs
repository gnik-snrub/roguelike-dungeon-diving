pub mod map;
use map::tiles::Tile;
use map::*;

use map::{ // List of map gen variants go here
    rectangles::rectangles,
    drunk_walk::drunk_walk,
    cellular_automata::cellular_automata,
    modifiers::*,
};

pub mod spawner;
use spawner::{ rooms_spawner, no_rooms_spawner };

use crate::{ Tcod, initialise_fov };
use crate::graphics::gui::Messages;
use crate::objects::{ Object, Character };
use crate::graphics::gen_colors;

//use crate::Point;
//use crate::pathing::bfs::broad_first_search;

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
    pub fn new(
        mut characters: &mut Vec<Character>,
        mut items: &mut HashMap<i32, Object>,
        player: &mut Object,
        tcod: &mut Tcod
    ) -> Game {
        let map = make_map(player, &mut characters, &mut items, 1, tcod);
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
    items: &mut HashMap<i32, Object>,
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
    game.map = make_map(player, characters, items, game.dungeon_level, tcod);
    initialise_fov(tcod, &game.map);
}

enum MapType {
    Rectangles,
    DrunkenWalk,
    CellularAutomata,
}

pub fn make_map(
    mut player: &mut Object,
    mut characters: &mut Vec<Character>,
    mut items: &mut HashMap<i32, Object>,
    level: u32,
    tcod: &mut Tcod,
) -> Map {
    // Generate dungeon floor colors alongside variation
    let colors = gen_colors();

    // Fill map with wall tiles
    let mut map = vec![vec![Tile::wall(&colors); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // Creates vector to store rooms
    // Another vector to store important points in non-room-based map gen.
    let mut rects = vec![];
    let mut points = vec![];

    let render = true;

    // Randomly decides which type of map to use, and generates it.
    let map_gen = rand::thread_rng().gen_range(1, 7);
//    let map_gen = 6;
    let map_type = match map_gen {
        // Standard rectangles map
        1 => {
            rectangles(&mut rects, &mut map, &colors, &mut player, tcod, render);
            MapType::Rectangles
        },

        // Rectangles map with an open area in the middle
        2 => {
            rectangles(&mut rects, &mut map, &colors, &mut player, tcod, render);
            mine_drunkenly(&rects, &mut map, &colors, tcod, render);
            MapType::Rectangles
        },

        // Rectangles map with the drunken miner modifier
        3 => {
            rectangles(&mut rects, &mut map, &colors, &mut player, tcod, render);
            caved_in(&mut map, &colors, tcod, render);
            MapType::Rectangles
        },

        // Rectangles map with the open area and drunken miner modifiers
        4 => {
            rectangles(&mut rects, &mut map, &colors, &mut player, tcod, render);
            mine_drunkenly(&rects, &mut map, &colors, tcod, render);
            caved_in(&mut map, &colors, tcod, render);
            MapType::Rectangles
        },

        5 => {
            drunk_walk(&mut points, &mut map, &colors, &mut player, tcod, render);
            MapType::DrunkenWalk
        }

        _ => {
            cellular_automata(&mut points, &mut map, &colors, &mut player, tcod, render);
            MapType::CellularAutomata
        }
    };

    // Randomly adds in corner pillars, or debris, if map has rooms.
    match map_type {
        MapType::Rectangles => {
            match rand::thread_rng().gen_range(1, 4) {
                1 => pillars(&rects, &mut map, &colors, tcod, render),
                2 => rubble(&rects, &mut map, &colors, tcod, render),
                _ => {},
            }
            purge_loners(&mut map, &colors, 2); // Widens corridor entrances

            create_tunnels(&mut rects, &mut map, &colors, tcod, render);
            rooms_spawner(&rects, &mut items, &map, &mut characters, level);
        },
        MapType::DrunkenWalk => {
//            let max_walls = 0;
//            purge_loners(&mut map, &colors, max_walls);

            joiner(&mut points, &mut map, &colors, tcod, render);
            no_rooms_spawner(&mut items, &map, &mut characters, level);
        },
        MapType::CellularAutomata => {
            no_rooms_spawner(&mut items, &map, &mut characters, level);
        },
    }

    map
}
