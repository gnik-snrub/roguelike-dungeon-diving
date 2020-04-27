pub mod ai;
use ai::*;

use super::Object;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub level: i32,
    pub exp: i32,
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

impl Object {
    pub fn fire_elemental(x: i32, y: i32) -> Object {
        let mut fire_elemental = Object::new(x, y, 'f', tcod::colors::ORANGE, "Fire Elemental", true);
        fire_elemental.fighter = Some(Fighter {
            level: 1,
            exp: 0,
            max_hp: 10,
            hp: 10,
            defense: 2,
            power: 5,
        });
        fire_elemental.ai = Some(Ai::Basic);
        fire_elemental
    }

    pub fn crystal_lizard(x: i32, y: i32) -> Object {
        let mut crystal_lizard = Object::new(x, y, 'C', tcod::colors::LIGHTER_SKY, "Crystal Lizard", true);
        crystal_lizard.fighter = Some(Fighter {
            level: 1,
            exp: 0,
            max_hp: 16,
            hp: 16,
            defense: 3,
            power: 3,
        });
        crystal_lizard.ai = Some(Ai::Basic);
        crystal_lizard
    }

}
