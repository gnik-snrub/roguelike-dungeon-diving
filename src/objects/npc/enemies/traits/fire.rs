use super::*;

// Each trait has three difficulty levels which are encountered depending on the depth of the dungeon.
// First, universal elements of the trait are established
// Then, the three power levels are established.
// Finally, the relevant power level is added into the trait, and returned to the generator.
pub fn fire_trait(tier: i32) -> Trait {
    let corpse = String::from("embers");
    let color = tcod::colors::AMBER;

    let weak = Trait {
        name: String::from("Warm "),
        exp: 15,
        hp: 0,
        defense: 0,
        power: 2,
        color: color,
        corpse_type: corpse.clone(),
    };

    let mid = Trait {
        name: String::from("Flaming "),
        exp: 50,
        hp: 2,
        defense: 2,
        power: 3,
        color: color,
        corpse_type: corpse.clone(),
    };

    let strong = Trait {
        name: String::from("Blazing "),
        exp: 150,
        hp: 5,
        defense: 3,
        power: 7,
        color: color,
        corpse_type: corpse.clone(),
    };

    let new_trait = match tier {
        1 => weak,
        2 => mid,
        _ => strong,
    };

    new_trait
}
