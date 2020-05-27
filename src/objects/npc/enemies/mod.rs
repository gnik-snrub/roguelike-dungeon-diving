pub mod enemy_list;
pub mod traits;

use traits::get_trait;
use enemy_list::get_monster;

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
}

pub fn monster_level_up(mut fighter: &mut Fighter) {
    let rng = rand::thread_rng().gen_range(0, 3);
    match rng {
        0 => {
            fighter.max_hp += 5;
            fighter.hp += 5;
        },
        1 => fighter.defense += 1,
        2 => fighter.power += 1,
        _ => {},
    }
}

pub fn generate_monster(x: i32, y: i32, tier: i32, level: u32) -> Character {

    // Selects random base monster and trait.
    let enemy_trait = get_trait(tier);
    let mut monster = get_monster(x, y, level, tier);

    // Changes base monster variables to reflect the trait.
    monster.object.name = format!("{}{}", enemy_trait.name, monster.object.name);
    monster.object.corpse_type.push_str(&enemy_trait.corpse_type);
    monster.object.color = enemy_trait.color;

    // Adjust combat capabilities of the monster to reflect the trait.
    monster.object.fighter.as_mut().map(|f| {
        f.exp += enemy_trait.exp;
        f.max_hp += enemy_trait.hp;
        f.hp += enemy_trait.hp;
        f.defense += enemy_trait.defense;
        f.power += enemy_trait.power;
    });

    monster
}
