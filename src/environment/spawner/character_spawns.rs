use crate::environment::{ Map, MapTheme };
use crate::environment::map::Rect;
use crate::objects::{ Object, Character };
use crate::objects::npc::enemies::{ generate_monster, monster_level_up };
use super::*;

use rand::*;
use rand::distributions::{ IndependentSample, Weighted, WeightedChoice };

fn monster_strength_weighting(level: u32) -> [Weighted<&'static str>; 3] {
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

    [
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
    ]
}

pub fn room_characters(room: Rect, map: &Map, characters: &mut Vec<Character>, level: u32, theme: MapTheme) {
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

    let mut monster_chances = monster_strength_weighting(level);
    let monster_choice = WeightedChoice::new(&mut monster_chances);

    for _ in 0..num_monsters {

        // Choose random spot for the monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if !Object::is_blocked(x, y, map, characters) {
            let mut monster = match monster_choice.ind_sample(&mut rand::thread_rng()) {
                "weak_monster" => generate_monster(x, y, 1, level, theme),
                "medium_monster" => generate_monster(x, y, 2, level, theme),
                "powerful_monster" => generate_monster(x, y, 3, level, theme),
                _ => unreachable!(),
            };
            monster.object.alive = true;

            // Level up the monster to increase the difficulty.
            let mut level_up = level - 1;
            while level_up > 0 {
                monster.object.fighter.as_mut().map(|mut f| monster_level_up(&mut f));
                level_up -= 1;
            }

            characters.push(monster);
        }
    }
}

pub fn no_room_characters(map: &Map, characters: &mut Vec<Character>, level: u32, theme: MapTheme) {

    // Creates maximum number of monsters per room.
    let max_monsters = from_dungeon_level(
        &[
            Transition { level: 1, value: 2 },
            Transition { level: 4, value: 3 },
            Transition { level: 6, value: 5 },
        ],
        level,
    );

    let mut monster_chances = monster_strength_weighting(level);
    let monster_choice = WeightedChoice::new(&mut monster_chances);

    let map_regions = 7;
    let mut map_region_start = 0;

    for _ in 0..map_regions {
        // Choose random number of monsters
        let num_monsters = rand::thread_rng().gen_range(0, max_monsters + 1);

        let mut monsters_placed = 0;
        let mut attempts = 0;
        let max_tries = 100;

        while monsters_placed < num_monsters {

            // Choose random spot for the monster
            let x = rand::thread_rng().gen_range(map_region_start, map_region_start + 10);
            let y = rand::thread_rng().gen_range(1, MAP_HEIGHT - 1);

            if x >= MAP_WIDTH - 1 { break; }

            if !Object::is_blocked(x, y, map, characters) {
                let mut monster = match monster_choice.ind_sample(&mut rand::thread_rng()) {
                    "weak_monster" => generate_monster(x, y, 1, level, theme),
                    "medium_monster" => generate_monster(x, y, 2, level, theme),
                    "powerful_monster" => generate_monster(x, y, 3, level, theme),
                    _ => unreachable!(),
                };
                monster.object.alive = true;

                // Level up the monster to increase the difficulty.
                let mut level_up = level - 1;
                while level_up > 0 {
                    monster.object.fighter.as_mut().map(|mut f| monster_level_up(&mut f));
                    level_up -= 1;
                }

                characters.push(monster);
                monsters_placed += 1;
            } else {
                attempts += 1;
                if attempts >= max_tries {
                    break;
                }
            }
        }
//        println!("Increment map region");
        map_region_start += 10;
    }
}
