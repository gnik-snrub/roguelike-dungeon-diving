pub mod map;
use map::tiles::Tile;
use map::{
    rectangles::rectangles,
    drunken_miners::drunken_miners,
    open_rectangles::open_rectangles,
    open_drunken_miners::open_drunken_miners,
};
use map::*;

use crate::{ Tcod, initialise_fov };
use crate::graphics::gui::Messages;
use crate::objects::{ Object, Character, items::Item };
use crate::graphics::gen_colors;

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
        "You take a moment to rest, and recover your strenght.",
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
    characters: &mut Vec<Character>,
    items: &mut HashMap<i32, Object>,
    level: u32,
) -> Map {
    // Generate dungeon floor colors alongside variation
    let colors = gen_colors();

    // Fill map with wall tiles
    let mut map = vec![vec![Tile::wall(&colors); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // Creates vector to store rooms
    let mut rooms: Vec<Rect> = vec![];

    // Randomly decides which type of map to use, and generates it.
    let map_type = rand::thread_rng().gen_range(1, 5);
    println!("{}", map_type);
    match map_type {
        1 => rectangles(&mut rooms, &mut map, &colors, &mut player),
        2 => drunken_miners(&mut rooms, &mut map, &colors, &mut player),
        3 => open_rectangles(&mut rooms, &mut map, &colors, &mut player),
        _ => open_drunken_miners(&mut rooms, &mut map, &colors, &mut player),
    }

    // Connects rooms together with horizontal/vertical tunnels.
    create_tunnels(&rooms, &mut map, &colors);

    // Ensures that there are no existing entities in the character, or item collections.
    characters.clear();
    items.clear();

    // Keeps track of total items spawned on a map.
    let mut item_counter = 1;

    for room in &rooms {
        place_characters(*room, &map, characters, level);
        place_items(*room, items, &map, characters, &mut item_counter, level);
    }

    // Create stairs at the center of the last room.
    let (last_room_x, last_room_y) = rooms[rooms.len() - 1].center();
    let stairs = create_stairs(last_room_x, last_room_y);
    let mut stairs_id = 1; // Sets up id for stairs to use in items hashmap.
    for _ in 0..items.len() {
        if items.contains_key(&stairs_id) {
            stairs_id += 1;
        } else {
            break;
        }
    }

    items.insert(stairs_id, stairs); // Finally, inserts stairs into the items hashmap.

    map
}

struct Transition {
    level: u32,
    value: u32,
}

fn from_dungeon_level(table: &[Transition], level: u32) -> u32 {
    table.iter().rev()
        .find(|transition| level >= transition.level)
        .map_or(0, |transition| transition.value)
}

fn place_characters(room: Rect, map: &Map, characters: &mut Vec<Character>, level: u32) {
    // Creates maximum number of monsters per room.
    let max_monsters = from_dungeon_level(
        &[
            Transition { level: 1, value: 2 },
            Transition { level: 4, value: 3 },
            Transition { level: 6, value: 5 },
        ],
        level,
    );

    // Choose random number of monsters
    let num_monsters = rand::thread_rng().gen_range(0, max_monsters + 1);

    let crystal_lizard_chance = from_dungeon_level(
        &[
            Transition {
                level: 3,
                value: 15,
            },
            Transition {
                level: 5,
                value: 30,
            },
            Transition {
                level: 7,
                value: 60,
            },
        ],
        level,
    );

    let monster_chances = &mut [
        Weighted {
            weight: 80,
            item: "fire_elemental",
        },
        Weighted {
            weight: crystal_lizard_chance,
            item: "crystal_lizard",
        },
    ];
    let monster_choice = WeightedChoice::new(monster_chances);

    for _ in 0..num_monsters {
        let level_up = level - 1;

        // Choose random spot for the monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if !Object::is_blocked(x, y, map, characters) {
            let mut monster = match monster_choice.ind_sample(&mut rand::thread_rng()) {
                "fire_elemental" => Object::fire_elemental(x, y, level_up),
                "crystal_lizard" => Object::crystal_lizard(x, y, level_up),
                _ => unreachable!(),
            };
            monster.object.alive = true;
            characters.push(monster);
        }
    }
}

fn place_items(
    room: Rect,
    items: &mut HashMap<i32, Object>,
    map: &Map,
    characters: &mut Vec<Character>,
    item_counter: &mut i32,
    level: u32
) {
    // Decides maximum number of items per room.
    let max_items = from_dungeon_level(
        &[
            Transition { level: 1, value: 1 },
            Transition { level: 4, value: 2 },
        ],
        level,
    );

    let item_chances = &mut [
        // Healing potion will always show up, regardless of other item chances.
        Weighted {
            weight: 35,
            item: Item::Heal,
        },
        Weighted {
            weight: from_dungeon_level(
                &[
                    Transition { level: 4, value: 25, }
                ],
                level,
            ),
            item: Item::LightningBoltScroll,
        },
        Weighted {
            weight: from_dungeon_level(
                &[
                    Transition { level: 6, value: 25, },
                    Transition { level: 8, value: 50, },
                    Transition { level: 10, value: 10, },
                ],
                level,
            ),
            item: Item::FireballScroll,
        },
        Weighted {
            weight: from_dungeon_level(
                &[
                    Transition { level: 2, value: 10, },
                    Transition { level: 12, value: 20, },
                ],
                level,
            ),
            item: Item::ConfusionScroll,
        },
        Weighted {
            weight: from_dungeon_level(
                &[
                    Transition { level: 5, value: 10, },
                ],
                level,
            ),
            item: Item::HpUp,
        },
        Weighted {
            weight: from_dungeon_level(
                &[
                    Transition { level: 7, value: 10, },
                ],
                level,
            ),
            item: Item::PowUp,
        },
        Weighted {
            weight: from_dungeon_level(
                &[
                    Transition { level: 10, value: 10, },
                ],
                level,
            ),
            item: Item::DefUp,
        },
    ];
    let item_choice = WeightedChoice::new(item_chances);

    // Choose random number of items.
    let num_items = rand::thread_rng().gen_range(0, max_items + 1);

    for _ in 0..num_items {
        // Select random spot for the item.
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if !Object::is_blocked(x, y, map, characters) {
            let item = match item_choice.ind_sample(&mut rand::thread_rng()) {
                Item::Heal => {
                    // Create a health potion.
                    Object::health_pot(x, y)
                },
                Item::LightningBoltScroll => {
                    // Creates a lightning bolt scroll.
                    Object::lightning_bolt_scroll(x, y)
                },
                Item::FireballScroll => {
                    // Creates a fireball scroll
                    Object::fireball_scroll(x, y)
                },
                Item::ConfusionScroll => {
                    // Creates a confusion scroll
                    Object::confusion_scroll(x, y)
                },
                Item::HpUp => {
                    // Creates a Health upgrade
                    Object::health_up(x, y)
                },
                Item::PowUp => {
                    // Creates a Power upgrade
                    Object::power_up(x, y)
                },
                Item::DefUp => {
                    // Creates a Defense upgrade
                    Object::defense_up(x, y)
                },
            };
            items.insert(*item_counter, item);
            *item_counter += 1;
        }
    }
}
