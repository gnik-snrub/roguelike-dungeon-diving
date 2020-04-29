pub mod enemies;
pub mod ai;

use crate::environment::Game;
use super::Object;

use tcod::colors::*;

// Creates struct that can be applied to fighter-type npcs.
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

// Allows for different death effects based on the enemy killed.
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
