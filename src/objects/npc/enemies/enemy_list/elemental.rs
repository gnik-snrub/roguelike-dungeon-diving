use super::*;

// Each monster has three difficulty levels which are encountered depending on the depth of the dungeon.
// First, universal elements of the monster are established
// Then, the three power levels are established.
// Finally, the relevant power level is added into the monster, and returned to the generator.
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
