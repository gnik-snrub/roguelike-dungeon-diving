use crate::environment::Game;
use super::Object;

pub mod ai;
use ai::*;

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
        let mut fire_elemental = Object::new(x, y, 'f', tcod::colors::DARK_AMBER, "Fire Elemental", true);
        fire_elemental.fighter = Some(Fighter {
            level: 1,
            exp: 0,
            max_hp: 12,
            hp: 12,
            defense: 1,
            power: 4,
            on_death: DeathCallback::Monster,
        });
        fire_elemental.ai = Some(Ai::Basic);
        fire_elemental.corpse_type = " ashes".into();
        fire_elemental
    }

    pub fn crystal_lizard(x: i32, y: i32) -> Object {
        let mut crystal_lizard = Object::new(x, y, 'C', tcod::colors::DARK_SKY, "Crystal Lizard", true);
        crystal_lizard.fighter = Some(Fighter {
            level: 1,
            exp: 0,
            max_hp: 8,
            hp: 8,
            defense: 3,
            power: 2,
            on_death: DeathCallback::Monster,
        });
        crystal_lizard.ai = Some(Ai::Basic);
        crystal_lizard.corpse_type = " shards".into();
        crystal_lizard
    }

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    pub fn callback(self, object: &mut Object, game: &mut Game) {
        let callback: fn(&mut Object, &mut Game) = match self {
            DeathCallback::Player => Object::player_death,
            DeathCallback::Monster => Object::monster_death,
        };
        callback(object, game);
    }
}

impl Object {
    fn player_death(player: &mut Object, game: &mut Game) {
        // The game ended!
        game.messages.add("You died, lmao!", RED);
        player.char = '%';
        player.color = DARK_RED;
        player.name = format!("{}{}", player.name, player.corpse_type);
    }

    fn monster_death(monster: &mut Object, game: &mut Game) {
        // Turns monster into a corpse.
        // No longer blocks, attacks, or moves.
        game.messages.add(format!("{} is dead!", monster.name), DARK_RED);
        monster.color = DARK_RED;
        monster.blocks = false;
        monster.fighter = None;
        monster.ai = None;
        monster.name = format!("{}{}", monster.name, monster.corpse_type);
    }
}
