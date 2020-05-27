use crate::environment::Map;
use crate::environment::map::Rect;
use crate::objects::{ Object, Character, items::Item };
use super::*;

use std::collections::HashMap;

use rand::*;
use rand::distributions::{ IndependentSample, Weighted, WeightedChoice };

pub fn room_items(
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

pub fn no_room_items(
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
            Transition { level: 10, value: 3 },
        ],
        level,
    );
    // Choose random number of items.
    let num_items = rand::thread_rng().gen_range(0, max_items + 1);

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

    let map_regions = 7;
    let mut map_region_start = 0;

//    println!("Pre-spawn item loop");

    for _ in 1..map_regions {
//        println!("Outer item loop");
        let mut region_items = 0;
        let mut attempts = 0;
        let max_tries = 25;
        while region_items <= num_items {
//            println!("Inner item loop");
            // Select random spot for the item.
            let x = rand::thread_rng().gen_range(map_region_start, map_region_start + 10);
            let y = rand::thread_rng().gen_range(1, MAP_HEIGHT - 1);

            if x >= MAP_WIDTH { break; }

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
                region_items += 1;
            } else {
                attempts += 1;
            }

            if attempts >= max_tries {
                map_region_start += 10;
                attempts = 0;
            }
        }
        map_region_start += 10;
    }
}
