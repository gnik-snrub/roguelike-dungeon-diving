use crate::environment::Map;
use crate::environment::map::Rect;
use crate::objects::{ Object, Character, items::Item };
use crate::objects::npc::enemies::generate_monster;

use std::collections::HashMap;

use rand::*;
use rand::distributions::{ IndependentSample, Weighted, WeightedChoice };

use tcod::colors::*;

pub fn spawner(
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

    let weak_monster_chance = from_dungeon_level(
        &[
            Transition {
                level: 1,
                value: 80,
            },
            Transition {
                level: 5,
                value: 60,
            },
            Transition {
                level: 7,
                value: 45,
            },
        ],
        level,
    );

    let medium_monster_chance = from_dungeon_level(
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

    let powerful_monster_chance = from_dungeon_level(
        &[
            Transition {
                level: 6,
                value: 15,
            },
            Transition {
                level: 9,
                value: 30,
            },
            Transition {
                level: 12,
                value: 80,
            },
        ],
        level,
    );

    let monster_chances = &mut [
        Weighted {
            weight: weak_monster_chance,
            item: "weak_monster",
        },
        Weighted {
            weight: medium_monster_chance,
            item: "medium_monster",
        },
        Weighted {
            weight: powerful_monster_chance,
            item: "powerful_monster",
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
                "weak_monster" => generate_monster(x, y, 1, level, level_up),
                "medium_monster" => generate_monster(x, y, 2, level, level_up),
                "powerful_monster" => generate_monster(x, y, 3, level, level_up),
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

pub fn create_stairs(x: i32, y: i32) -> Object {
    Object {
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
    }
}
