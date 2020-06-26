use super::*;

// Each monster has three difficulty levels which are encountered depending on the depth of the dungeon.
// First, universal elements of the monster are established
// Then, the three power levels are established.
// Finally, the relevant power level is added into the monster, and returned to the generator.
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
