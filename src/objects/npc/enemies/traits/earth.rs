use super::*;

// Each trait has three difficulty levels which are encountered depending on the depth of the dungeon.
// First, universal elements of the trait are established
// Then, the three power levels are established.
// Finally, the relevant power level is added into the trait, and returned to the generator.
pub fn earth_trait(tier: i32) -> Trait {
    let corpse = String::from("rubble");
    let color = tcod::colors::Color{ r:98 ,g:74 ,b:10 };

    let weak = Trait {
        name: String::from("Dirt "),
        exp: 15,
        hp: 0,
        defense: 2,
        power: 0,
        color: color,
        corpse_type: corpse.clone(),
    };

    let mid = Trait {
        name: String::from("Cave "),
        exp: 50,
        hp: 2,
        defense: 3,
        power: 2,
        color: color,
        corpse_type: corpse.clone(),
    };

    let strong = Trait {
        name: String::from("Mountain "),
        exp: 150,
        hp: 5,
        defense: 7,
        power: 3,
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
