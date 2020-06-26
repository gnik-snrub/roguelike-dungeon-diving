use super::*;

// Each monster has three difficulty levels which are encountered depending on the depth of the dungeon.
// First, universal elements of the monster are established
// Then, the three power levels are established.
// Finally, the relevant power level is added into the monster, and returned to the generator.
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
