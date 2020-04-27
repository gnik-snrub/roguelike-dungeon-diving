pub mod ai;
use ai::*;

use super::Object;

use tcod::colors::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    pub level: i32,
    pub exp: i32,
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub on_death: DeathCallback,
}

impl Object {
    pub fn fire_elemental(x: i32, y: i32) -> Object {
        let mut fire_elemental = Object::new(x, y, 'f', tcod::colors::ORANGE, "Fire Elemental", true);
        fire_elemental.fighter = Some(Fighter {
            level: 1,
            exp: 0,
            max_hp: 10,
            hp: 10,
            defense: 0,
            power: 6,
            on_death: DeathCallback::Monster,
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
            defense: 4,
            power: 1,
            on_death: DeathCallback::Monster,
        });
        crystal_lizard.ai = Some(Ai::Basic);
        crystal_lizard
    }

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    pub fn callback(self, object: &mut Object) {
        let callback: fn(&mut Object) = match self {
            DeathCallback::Player => Object::player_death,
            DeathCallback::Monster => Object::monster_death,
        };
        callback(object);
    }
}

impl Object {
    fn player_death(player: &mut Object) {
        // The game ended!
        println!("You died...");
        player.char = '%';
        player.color = DARK_RED;
    }

    fn monster_death(monster: &mut Object) {
        // Turns monster into a corpse.
        // No longer blocks, attacks, or moves.
        println!("{} is dead!", monster.name);
        monster.color = DARK_RED;
        monster.blocks = false;
        monster.fighter = None;
        monster.ai = None;
        monster.name = format!("Remains of {}", monster.name);
    }
}
