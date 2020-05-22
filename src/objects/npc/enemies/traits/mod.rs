use rand::*;
use super::*;

const TRAIT_MIN: i32 = 1;
const TRAIT_MAX: i32 = 6;

#[derive(Debug)]
pub struct Trait {
    pub name: String,
    pub exp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub color: Color,
    pub corpse_type: String,
}

pub fn get_trait(tier: i32) -> Trait {
    let trait_roll = rand::thread_rng().gen_range(TRAIT_MIN, TRAIT_MAX);
    let new_trait = match trait_roll {
        1 => fire_trait(tier),
        2 => water_trait(tier),
        3 => earth_trait(tier),
        4 => wind_trait(tier),
        _ => crystal_trait(tier),
    };

    new_trait
}

fn fire_trait(tier: i32) -> Trait {
    let corpse = String::from("embers");
    let color = tcod::colors::LIGHT_AMBER;

    let weak = Trait {
        name: String::from("Fire "),
        exp: 15,
        hp: 0,
        defense: 0,
        power: 2,
        color: color,
        corpse_type: corpse.clone(),
    };

    let mid = Trait {
        name: String::from("Blaze "),
        exp: 50,
        hp: 2,
        defense: 2,
        power: 3,
        color: color,
        corpse_type: corpse.clone(),
    };

    let strong = Trait {
        name: String::from("Volcano "),
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

fn earth_trait(tier: i32) -> Trait {
    let corpse = String::from("rubble");
    let color = tcod::colors::Color{ r:198 ,g:174 ,b:110 };

    let weak = Trait {
        name: String::from("Earth "),
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
        name: String::from("Core "),
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

fn wind_trait(tier: i32) -> Trait {
    let corpse = String::from("dust");
    let color = tcod::colors::LIGHTEST_YELLOW;

    let weak = Trait {
        name: String::from("Wind "),
        exp: 15,
        hp: 0,
        defense: 0,
        power: 2,
        color: color,
        corpse_type: corpse.clone(),
    };

    let mid = Trait {
        name: String::from("Tornado "),
        exp: 50,
        hp: 0,
        defense: 3,
        power: 4,
        color: color,
        corpse_type: corpse.clone(),
    };

    let strong = Trait {
        name: String::from("Hurricane "),
        exp: 150,
        hp: 0,
        defense: 5,
        power: 10,
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

fn water_trait(tier: i32) -> Trait {
    let corpse = String::from("puddle");
    let color = tcod::colors::LIGHT_BLUE;

    let weak = Trait {
        name: String::from("Water "),
        exp: 15,
        hp: 1,
        defense: 1,
        power: 1,
        color: color,
        corpse_type: corpse.clone(),
    };

    let mid = Trait {
        name: String::from("Deep "),
        exp: 50,
        hp: 3,
        defense: 2,
        power: 2,
        color: tcod::colors::BLUE,
        corpse_type: corpse.clone(),
    };

    let strong = Trait {
        name: String::from("Tsunami "),
        exp: 150,
        hp: 5,
        defense: 5,
        power: 5,
        color: BLUE,
        corpse_type: corpse.clone(),
    };

    let new_trait = match tier {
        1 => weak,
        2 => mid,
        _ => strong,
    };

    new_trait
}

fn crystal_trait(tier: i32) -> Trait {
    let corpse = String::from("shards");
    let color = tcod::colors::LIGHT_SKY;

    let weak = Trait {
        name: String::from("Crystal "),
        exp: 15,
        hp: 1,
        defense: 2,
        power: 0,
        color: color,
        corpse_type: corpse.clone(),
    };

    let mid = Trait {
        name: String::from("Quartz "),
        exp: 50,
        hp: 3,
        defense: 4,
        power: 1,
        color: color,
        corpse_type: corpse.clone(),
    };

    let strong = Trait {
        name: String::from("Diamond "),
        exp: 150,
        hp: 7,
        defense: 6,
        power: 4,
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
