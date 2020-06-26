pub mod elemental;
pub mod lizard;
pub mod blob;

use crate::environment::spawner::{ Transition, from_dungeon_level };

use super::*;
use rand::distributions::{ IndependentSample, Weighted, WeightedChoice };

// Selects, generates, and returns a random monster based on the depth level.
// Function is used by the random monster generator.
pub fn get_monster(x: i32, y: i32, level: u32, tier: i32) -> Character {

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

    let new_monster = match monster_choice.ind_sample(&mut rand::thread_rng()) {
        "weak_monster" => elemental::elemental(x, y, tier),
        "medium_monster" => lizard::lizard(x, y, tier),
        "powerful_monster" => blob::blob(x, y, tier),
        _ => unreachable!(),
    };
    new_monster
}
