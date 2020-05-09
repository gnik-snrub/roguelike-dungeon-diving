use crate::objects::Character;
use super::ai::*;
use super::*;

impl Object {
    fn new_enemy(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool, corpse_type: &str) -> Character {
        Character {
            object: Object {
            x: x,
            y: y,
            char: char,
            color: color,
            name: name.into(),
            blocks: blocks,
            alive: false,
            corpse_type: corpse_type.into(),
            fighter: None,
            ai: Some(Ai::Basic),
            item: None,
            level: 1,
            always_visible: false,
            },
            inventory: None,
        }
    }

    pub fn fire_elemental(x: i32, y: i32) -> Character {
        let mut fire_elemental = Object::new_enemy(x, y, 'f', tcod::colors::LIGHT_AMBER, "Fire Elemental", true, " ashes");
        fire_elemental.object.fighter = Some(Fighter {
            exp: 35,
            max_hp: 20,
            hp: 20,
            defense: 0,
            power: 4,
            on_death: DeathCallback::Monster,
        });
        fire_elemental
    }

    pub fn crystal_lizard(x: i32, y: i32) -> Character {
        let mut crystal_lizard = Object::new_enemy(x, y, 'C', tcod::colors::LIGHT_SKY, "Crystal Lizard", true, " shards");
        crystal_lizard.object.fighter = Some(Fighter {
            exp: 100,
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 8,
            on_death: DeathCallback::Monster,
        });
        crystal_lizard
    }
}
