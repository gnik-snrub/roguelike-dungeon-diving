use crate::objects::Character;
use crate::objects::npc::Fighter;
use super::*;

pub fn fire_elemental(x: i32, y: i32, tier: i32) -> Character {
    let mut fire_elemental = Object::new_enemy(x, y, 'f', tcod::colors::LIGHT_AMBER, "Fire Elemental", true, " ashes");

    let weak_fighter = Fighter {
        exp: 0,
        max_hp: 20,
        hp: 20,
        defense: 0,
        power: 4,
        on_death: DeathCallback::Monster,
    };

    let mid_fighter = Fighter {
        exp: 0,
        max_hp: 25,
        hp: 25,
        defense: 0,
        power: 10,
        on_death: DeathCallback::Monster,
    };

    let strong_fighter = Fighter {
        exp: 0,
        max_hp: 35,
        hp: 35,
        defense: 4,
        power: 16,
        on_death: DeathCallback::Monster,
    };

    match tier {
        1 => fire_elemental.object.fighter = Some(weak_fighter),
        2 => fire_elemental.object.fighter = Some(mid_fighter),
        3 => fire_elemental.object.fighter = Some(strong_fighter),
        _ => {},
    }

    fire_elemental
}
