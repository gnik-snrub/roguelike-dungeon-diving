use crate::objects::Character;
use crate::objects::npc::Fighter;
use super::*;

pub fn crystal_lizard(x: i32, y: i32, tier: i32) -> Character {
    let mut crystal_lizard = Object::new_enemy(x, y, 'C', tcod::colors::LIGHT_SKY, "Crystal Lizard", true, " shards");

    let weak_fighter = Fighter {
        exp: 0,
        max_hp: 25,
        hp: 25,
        defense: 2,
        power: 2,
        on_death: DeathCallback::Monster,
    };

    let mid_fighter = Fighter {
        exp: 0,
        max_hp: 30,
        hp: 30,
        defense: 2,
        power: 8,
        on_death: DeathCallback::Monster,
    };

    let strong_fighter = Fighter {
        exp: 0,
        max_hp: 45,
        hp: 45,
        defense: 8,
        power: 12,
        on_death: DeathCallback::Monster,
    };

    match tier {
        1 => crystal_lizard.object.fighter = Some(weak_fighter),
        2 => crystal_lizard.object.fighter = Some(mid_fighter),
        3 => crystal_lizard.object.fighter = Some(strong_fighter),
        _ => {},
    }

    crystal_lizard
}
