use crate::environment::spawner::{ Transition, from_dungeon_level };

use super::*;
use rand::distributions::{ IndependentSample, Weighted, WeightedChoice };

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

    let new_trait = match monster_choice.ind_sample(&mut rand::thread_rng()) {
        "weak_monster" => elemental(x, y, tier),
        "medium_monster" => lizard(x, y, tier),
        "powerful_monster" => blob(x, y, tier),
        _ => unreachable!(),
    };
    new_trait
}

pub fn elemental(x: i32, y: i32, tier: i32) -> Character {
    let mut elemental = Object::new_enemy(x, y, 'f', tcod::colors::LIGHT_AMBER, "Elemental", true, " ");

    let weak_fighter = Fighter {
        exp: 35,
        max_hp: 20,
        hp: 20,
        defense: 0,
        power: 3,
        on_death: DeathCallback::Monster,
    };

    let mid_fighter = Fighter {
        exp: 125,
        max_hp: 25,
        hp: 25,
        defense: 0,
        power: 10,
        on_death: DeathCallback::Monster,
    };

    let strong_fighter = Fighter {
        exp: 300,
        max_hp: 35,
        hp: 35,
        defense: 4,
        power: 16,
        on_death: DeathCallback::Monster,
    };

    match tier {
        1 => elemental.object.fighter = Some(weak_fighter),
        2 => elemental.object.fighter = Some(mid_fighter),
        3 => elemental.object.fighter = Some(strong_fighter),
        _ => {},
    }

    elemental
}

pub fn lizard(x: i32, y: i32, tier: i32) -> Character {
    let mut lizard = Object::new_enemy(x, y, 'C', tcod::colors::LIGHT_SKY, "Lizard", true, " ");

    let weak_fighter = Fighter {
        exp: 60,
        max_hp: 25,
        hp: 25,
        defense: 2,
        power: 2,
        on_death: DeathCallback::Monster,
    };

    let mid_fighter = Fighter {
        exp: 100,
        max_hp: 30,
        hp: 30,
        defense: 2,
        power: 8,
        on_death: DeathCallback::Monster,
    };

    let strong_fighter = Fighter {
        exp: 250,
        max_hp: 45,
        hp: 45,
        defense: 8,
        power: 12,
        on_death: DeathCallback::Monster,
    };

    match tier {
        1 => lizard.object.fighter = Some(weak_fighter),
        2 => lizard.object.fighter = Some(mid_fighter),
        3 => lizard.object.fighter = Some(strong_fighter),
        _ => {},
    }

    lizard
}

pub fn blob(x: i32, y: i32, tier: i32) -> Character {
    let mut blob = Object::new_enemy(x, y, 'B', tcod::colors::LIGHTEST_GREEN, "blob", true, " ");

    let weak_fighter = Fighter {
        exp: 150,
        max_hp: 30,
        hp: 30,
        defense: 5,
        power: 5,
        on_death: DeathCallback::Monster,
    };

    let mid_fighter = Fighter {
        exp: 300,
        max_hp: 45,
        hp: 45,
        defense: 10,
        power: 10,
        on_death: DeathCallback::Monster,
    };

    let strong_fighter = Fighter {
        exp: 555,
        max_hp: 65,
        hp: 65,
        defense: 15,
        power: 15,
        on_death: DeathCallback::Monster,
    };

    match tier {
        1 => blob.object.fighter = Some(weak_fighter),
        2 => blob.object.fighter = Some(mid_fighter),
        3 => blob.object.fighter = Some(strong_fighter),
        _ => {},
    }

    blob
}
