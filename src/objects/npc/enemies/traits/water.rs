use super::*;

// Each trait has three difficulty levels which are encountered depending on the depth of the dungeon.
// First, universal elements of the trait are established
// Then, the three power levels are established.
// Finally, the relevant power level is added into the trait, and returned to the generator.
pub fn water_trait(tier: i32) -> Trait {
    let corpse = String::from("puddle");
    let color = tcod::colors::LIGHTER_AZURE;

    let weak = Trait {
        name: String::from("Wet "),
        exp: 15,
        hp: 1,
        defense: 1,
        power: 1,
        color: color,
        corpse_type: corpse.clone(),
    };

    let mid = Trait {
        name: String::from("Acquatic "),
        exp: 50,
        hp: 3,
        defense: 2,
        power: 2,
        color: color,
        corpse_type: corpse.clone(),
    };

    let strong = Trait {
        name: String::from("Deep "),
        exp: 150,
        hp: 5,
        defense: 5,
        power: 5,
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
