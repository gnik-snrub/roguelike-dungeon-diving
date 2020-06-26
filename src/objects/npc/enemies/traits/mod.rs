pub mod fire;
pub mod earth;
pub mod light;
pub mod water;
pub mod crystal;
pub mod death;
pub mod nature;

use crate::environment::MapTheme;

use rand::distributions::{ IndependentSample, Weighted, WeightedChoice };
use super::*;

// Trait definition.
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

// Randomly selects, and returns a trait type.
// Used by random monster generator.
pub fn get_trait(theme: MapTheme, tier: i32) -> Trait {

    let mut trait_chances = [
        Weighted {
            weight: if theme == MapTheme::Fire {
                140
            } else  if theme == MapTheme::Water {
                10
            } else {
                1
            },
            item: MapTheme::Fire,
        },
        Weighted {
            weight: if theme == MapTheme::Nature {
                140
            } else  if theme == MapTheme::Crystal {
                10
            } else {
                1
            },
            item: MapTheme::Nature,
        },
        Weighted {
            weight: if theme == MapTheme::Water {
                140
            } else  if theme == MapTheme::Nature {
                10
            } else {
                1
            },
            item: MapTheme::Water,
        },
        Weighted {
            weight: if theme == MapTheme::Light {
                140
            } else  if theme == MapTheme::Death {
                10
            } else {
                1
            },
            item: MapTheme::Light,
        },
        Weighted {
            weight: if theme == MapTheme::Death {
                140
            } else  if theme == MapTheme::Light {
                10
            } else {
                1
            },
            item: MapTheme::Death,
        },
        Weighted {
            weight: if theme == MapTheme::Crystal {
                140
            } else  if theme == MapTheme::Earth {
                10
            } else {
                1
            },
            item: MapTheme::Crystal,
        },
        Weighted {
            weight: if theme == MapTheme::Earth {
                140
            } else  if theme == MapTheme::Fire {
                10
            } else {
                1
            },
            item: MapTheme::Earth,
        },
    ];
    let trait_choice = WeightedChoice::new(&mut trait_chances);

    let new_trait = match trait_choice.ind_sample(&mut rand::thread_rng()) {
        MapTheme::Fire => fire::fire_trait(tier),
        MapTheme::Nature => nature::nature_trait(tier),
        MapTheme::Water => water::water_trait(tier),
        MapTheme::Light => light::light_trait(tier),
        MapTheme::Death => death::death_trait(tier),
        MapTheme::Crystal => crystal::crystal_trait(tier),
        MapTheme::Earth => earth::earth_trait(tier),
    };

    new_trait
}
