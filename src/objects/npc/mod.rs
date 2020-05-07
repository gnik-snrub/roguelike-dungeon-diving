pub mod enemies;
pub mod ai;

use crate::environment::Game;
use super::Object;

use serde::{ Serialize, Deserialize };

use tcod::colors::*;

// Creates struct that can be applied to fighter-type npcs.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fighter {
    pub exp: i32,
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub on_death: DeathCallback,
}

// Allows for different death effects based on the enemy killed.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    pub fn callback(self, object: &mut Object, game: &mut Game) {
        let callback: fn(&mut Object, &mut Game) = match self {
            DeathCallback::Player => Object::fake_player_death,
            DeathCallback::Monster => Object::monster_death,
        };
        callback(object, game);
    }
}

impl Object {
    fn fake_player_death(_object: &mut Object, _game: &mut Game) {}

    fn monster_death(monster: &mut Object, game: &mut Game) {
        // Turns monster into a corpse.
        // No longer blocks, attacks, or moves.
        game.messages.add(
            format!(
                "{} is dead! You gain {} experience points.",
                monster.name,
                monster.fighter.unwrap().exp
            ),
            DARK_RED);
        monster.color = DARK_RED;
        monster.blocks = false;
        monster.fighter = None;
        monster.ai = None;
        monster.name = format!("{}{}", monster.name, monster.corpse_type);
    }
}
