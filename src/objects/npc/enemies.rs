use crate::objects::Character;
use super::ai::*;
use super::*;

use rand::Rng;

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

    fn monster_level_up(mut fighter: Fighter) {
        let hp_addition = fighter.max_hp / 4;
        let rng = rand::thread_rng().gen_range(0, 3);
        match rng {
            0 => {
                fighter.max_hp += hp_addition;
                fighter.hp += hp_addition;
            },
            1 => fighter.defense += 1,
            2 => fighter.power += 1,
            _ => {},
        }
    }

    pub fn fire_elemental(x: i32, y: i32, mut level_up: u32) -> Character {
        let mut fire_elemental = Object::new_enemy(x, y, 'f', tcod::colors::LIGHT_AMBER, "Fire Elemental", true, " ashes");
        let fighter = Fighter {
            exp: 35,
            max_hp: 20,
            hp: 20,
            defense: 0,
            power: 4,
            on_death: DeathCallback::Monster,
        };
        while level_up > 0 {
            Object::monster_level_up(fighter);
            level_up -= 1;
        }
        fire_elemental.object.fighter = Some(fighter);
        fire_elemental
    }

    pub fn crystal_lizard(x: i32, y: i32, mut level_up: u32) -> Character {
        let mut crystal_lizard = Object::new_enemy(x, y, 'C', tcod::colors::LIGHT_SKY, "Crystal Lizard", true, " shards");
        let fighter = Fighter {
            exp: 100,
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 8,
            on_death: DeathCallback::Monster,
        };
        while level_up > 2 {
            if level_up % 2 == 0 {
                Object::monster_level_up(fighter);
            }
            level_up -= 1;
        }
        crystal_lizard.object.fighter = Some(fighter);
        crystal_lizard
    }
}
