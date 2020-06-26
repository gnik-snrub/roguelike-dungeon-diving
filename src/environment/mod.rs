pub mod map;
use map::tiles::Tile;
use map::*;

use map::{ // List of map gen variants go here
    rectangles::rectangles,
    drunk_walk::drunk_walk,
    cellular_automata::cellular_automata,
    maze::maze,
    modifiers::*,
};

pub mod spawner;
use spawner::{
    rooms_spawner,
    no_rooms_spawner,
    maze_spawner,
};

use crate::{ Tcod, initialise_fov };
use crate::graphics::gui::Messages;
use crate::objects::{ Object, Character };
use crate::graphics::gen_colors;
use crate::pathing::remove_inaccessible_tiles;
use crate::environment::spawner::{ Transition, from_dungeon_level };

use std::collections::HashMap;

use rand::*;
use rand::distributions::{ IndependentSample, Weighted, WeightedChoice };

use serde::{ Serialize, Deserialize };

use tcod::map::FovAlgorithm;
use tcod::colors::*;

// Determines Field-Of-View
pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic; // Default FOV Algorithm
pub const FOV_LIGHT_WALLS: bool = true;
pub const TORCH_RADIUS: i32 = 10;

// Size of the map
pub const MAP_WIDTH: i32 = 81;
pub const MAP_HEIGHT: i32 = 43;

// Dungeon room limitations
const ROOM_MAX_SIZE: i32 = 12;
const ROOM_MIN_SIZE: i32 = 4;
const MAX_ROOMS: i32 = 18;

const GROUND_COLOR: usize = 3;

// Bool value to decide whether or not the map generation should be rendered or not.
const RENDER: bool = false;

// Map type definition.
pub type Map = Vec<Vec<Tile>>;

// Game struct definition.
#[derive(Serialize, Deserialize)]
pub struct Game {
    pub map: Map,
    pub messages: Messages,
    pub dungeon_level: u32,
}

impl Game {
    // A new game is made by just creating a new map alongside an empty Messages list
    // And setting the dungeon level to 1.
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
    // Heals half of the players HP, and displays a message about it.
    game.messages.add(
        "You take a moment to rest, and recover your strength.",
        GREEN,
    );
    let heal_hp = player.fighter.map_or(0, |f| f.max_hp / 2);
    player.heal(heal_hp);

    // Sends the player deeper down
    game.messages.add(
        "After taking a moment to rest, you dive deeper into the caverns...",
        RED,
    );

    // Updates the dungeon depth, makes new dungeon map, and re-generates FOV map.
    game.dungeon_level += 1;
    game.map = make_map(player, characters, items, game.dungeon_level, tcod);
    initialise_fov(tcod, &game.map);
}

#[derive(Copy, Clone, PartialEq)]
pub enum MapTheme {
    Fire, //Red
    Nature, //Green
    Water, //Blue
    Light, //Yellow
    Death, //Purple
    Crystal, //Cyan
    Earth, //Other
}

fn greater(higher: u8, lower_one: u8, lower_two: u8) -> bool {
    if higher > (lower_one + 25) && higher > (lower_two + 25) {
        return true
    }
    return false
}
fn two_greater(higher_one: u8, higher_two: u8, lower: u8) -> bool {
    if higher_one > (lower + 25) && higher_two > (lower + 25) {
        return true
    }
    return false
}

fn set_map_theme(color: Color) -> MapTheme {
    let r = color.r;
    let b = color.b;
    let g = color.g;

    match (r, g, b) {
        (r, g, b) if greater(r, g, b) => MapTheme::Fire,
        (r, g, b) if greater(g, r, b) => MapTheme::Nature,
        (r, g, b) if greater(b, r, g) => MapTheme::Water,
        (r, g, b) if two_greater(r, g, b) => MapTheme::Light,
        (r, g, b) if two_greater(r, b, g) => MapTheme::Death,
        (r, g, b) if two_greater(g, b, r) => MapTheme::Crystal,
        _ => MapTheme::Earth,
    }
}

// Different map generation algorithms require different spawning systems, modifiers, corridors, etc...
// The MapType enum allows for an easy way to keep track of all of this.
enum MapType {
    Rectangles,
    DrunkenWalk,
    CellularAutomata,
    Maze,
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
    let map_theme = set_map_theme(colors[GROUND_COLOR]);

    // Fill map with wall tiles
    let mut map = vec![vec![Tile::wall(&colors); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // Creates vector to store rooms
    // Another vector to store important points in non-room-based map gen.
    let mut rects = vec![];
    let mut points = vec![];

    // Randomly decides which type of map to use, and generates it.
//    let map_gen = rand::thread_rng().gen_range(1, 8);
//    let map_gen = 1;

    let mut map_chances = [
        Weighted { // Weighting for basic rectangle room map gen.
            weight: from_dungeon_level(
                &[
                    Transition { level: 1, value: 30, }, // Is the only option at the first floor.
                    Transition { level: 3, value: 10, }, // Chance is lower from the 3rd floor onwards.
                    Transition { level: 9, value: 0, }, // Becomes unavailable on the floor prior to the maze.
                ],
                level,
            ),
            item: 1,
        },
        Weighted { // Weighting for rectangle room gen with an open area in the middle.
            weight: from_dungeon_level(
                &[
                    Transition { level: 3, value: 30, }, // Becomes available from the third floor
                    Transition { level: 5, value: 10, }, // Chance lowered from the fifth floor.
                    Transition { level: 9, value: 0, }, // Becomes unavailable on the floor prior to the maze.
                ],
                level,
            ),
            item: 2,
        },
        Weighted { // Weighting for rectangle drunken mining map gen.
            weight: from_dungeon_level(
                &[
                    Transition { level: 4, value: 30, }, //  Becomes available from the fourth floor.
                    Transition { level: 5, value: 10, }, // Chance lowered from the fifth floor onward.
                    Transition { level: 7, value: 20, }, // Chances are raised to take into account the raising of another weight. (The next category)
                    Transition { level: 9, value: 0, }, // Chance is zero to force the maze to appear for one floor.
                    Transition { level: 10, value: 5, }, // A small chance remains for this to appear afterwards.
                ],
                level,
            ),
            item: 3,
        },
        Weighted { // Weighting for rectangle drunken mining map gen with an open area in the middle.
            weight: from_dungeon_level(
                &[
                    Transition { level: 5, value: 30, }, //  Becomes available from the fifth floor.
                    Transition { level: 7, value: 20, }, // Chance lowered from the seventh floor onward.
                    Transition { level: 9, value: 0, }, // Chance is zero to force the maze to appear for one floor.
                    Transition { level: 10, value: 5, }, // A small chance remains for this to appear afterwards.
                ],
                level,
            ),
            item: 4,
        },
        Weighted { // Weighting for the drunken walk map generation
            weight: from_dungeon_level(
                &[
                    Transition { level: 9, value: 0, }, // Maze floor
                    Transition { level: 10, value: 30, }, // On the tenth floor, it becomes available
                ],
                level,
            ),
            item: 5,
        },
        Weighted { // Weighting for the cellular automata map gen.
            weight: from_dungeon_level(
                &[
                    Transition { level: 9, value: 0, }, // Maze floor
                    Transition { level: 11, value: 15, }, // On the eleventh floor it becomes available at a lower rate than the drunken walk.
                ],
                level,
            ),
            item: 6,
        },
        Weighted { // Weighting for the maze map gen
            weight: from_dungeon_level(
                &[
                    Transition { level: 9, value: 50, }, // Maze appears on the ninth floor.
                    Transition { level: 10, value: 0, }, // Maze can't appear afterwards
                ],
                level,
            ),
            item: 7
        },
    ];
    let map_gen = WeightedChoice::new(&mut map_chances);

    let map_type = match map_gen.ind_sample(&mut rand::thread_rng()) {
        // Standard rectangles map
        1 => {
            rectangles(&mut rects, &mut map, &colors, &mut player, tcod, RENDER);
            MapType::Rectangles
        },

        // Rectangles map with the drunken miner modifier
        2 => {
            rectangles(&mut rects, &mut map, &colors, &mut player, tcod, RENDER);
            caved_in(&mut map, &colors, tcod, RENDER);
            MapType::Rectangles
        },

        // Rectangles map with an open area in the middle
        3 => {
            rectangles(&mut rects, &mut map, &colors, &mut player, tcod, RENDER);
            mine_drunkenly(&rects, &mut map, &colors, tcod, RENDER);
            MapType::Rectangles
        },

        // Rectangles map with the open area and drunken miner modifiers
        4 => {
            rectangles(&mut rects, &mut map, &colors, &mut player, tcod, RENDER);
            mine_drunkenly(&rects, &mut map, &colors, tcod, RENDER);
            caved_in(&mut map, &colors, tcod, RENDER);
            MapType::Rectangles
        },

        // Creates a map entirely using the walking drunkard algorithm.
        5 => {
            drunk_walk(&mut points, &mut map, &colors, &mut player, tcod, RENDER);
            MapType::DrunkenWalk
        },

        // Creates a map following the rules of cellular automata.
        6 => {
            cellular_automata(&mut map, &colors, &mut player, tcod, RENDER);
            MapType::CellularAutomata
        },

        _ => {
            maze(&mut map, &colors, &mut player, tcod, RENDER);
            MapType::Maze
        },
    };

    // Adds in map modifiers, tunnels, and spawns stuff.
    // Specifics of this is determined by the map type which was generated.
    match map_type {
        MapType::Rectangles => {
            // Map modifiers
            match rand::thread_rng().gen_range(1, 4) {
                1 => pillars(&rects, &mut map, &colors, tcod, RENDER),
                2 => rubble(&rects, &mut map, &colors, tcod, RENDER),
                _ => {},
            }

            // Sorts the rooms.
            room_sorter(&mut rects);

            // Tunnels and spawns
            create_tunnels(&mut rects, &mut map, &colors, tcod, RENDER);
            rooms_spawner(&rects, &mut items, &map, &mut characters, level, map_theme);
        },

        MapType::DrunkenWalk => {
            // Sorts the point vector.
            room_sorter(&mut points);

            joiner(&mut points, &mut map, &colors, tcod, RENDER);
            no_rooms_spawner(&mut items, &map, &mut characters, level, map_theme);
        },

        MapType::CellularAutomata => {
            remove_inaccessible_tiles(&mut map, &player, &colors);
            no_rooms_spawner(&mut items, &map, &mut characters, level, map_theme);
        },

        MapType::Maze => {
            maze_spawner(&mut items, &map, &mut characters, level, map_theme);
        },
    }

    // Returns finished map.
    map
}
