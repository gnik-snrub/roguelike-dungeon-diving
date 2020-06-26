use super::*;

// Each trait has three difficulty levels which are encountered depending on the depth of the dungeon.
// First, universal elements of the trait are established
// Then, the three power levels are established.
// Finally, the relevant power level is added into the trait, and returned to the generator.
pub fn death_trait(tier: i32) -> Trait {
    let corpse = String::from("bones");
    let color = tcod::colors::DARKER_FUCHSIA;

    let weak = Trait {
        name: String::from("Necro "),
        exp: 15,
        hp: 1,
        defense: 2,
        power: 0,
        color: color,
        corpse_type: corpse.clone(),
    };

    let mid = Trait {
        name: String::from("Zombie "),
        exp: 50,
        hp: 3,
        defense: 4,
        power: 0,
        color: color,
        corpse_type: corpse.clone(),
    };

    let strong = Trait {
        name: String::from("Death "),
        exp: 150,
        hp: 7,
        defense: 2,
        power: 8,
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
