use crate::Tcod;
use crate::environment::Game;
use super::{ Object, HEAL_AMOUNT };

use tcod::colors::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Heal,
}

pub enum UseResult {
    UsedUp,
    Cancelled,
}

impl Object {
    // Generic item creator.
    fn new_item(x: i32, y: i32, char: char, name: &str, color: Color, blocks: bool) -> Object {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
            name: name.into(),
            blocks: blocks,
            alive: false,
            corpse_type: name.into(),
            fighter: None,
            ai: None,
            inventory: None,
            item: None,
        }
    }

    // Health Potion creator.
    pub fn health_pot(x: i32, y: i32) -> Object {
        let mut health_pot = Object::new_item(x, y, '!', "Health potion", LIGHT_GREEN, false);
        health_pot.item = Some(Item::Heal);
        health_pot
    }

    // Health potion function.
    pub fn cast_heal(
        _inventory_id: usize,
        _tcod: &mut Tcod,
        game: &mut Game,
        characters: &mut Vec<Object>
    ) -> UseResult {
        // Heal the player.
        if let Some(fighter) = game.player.fighter {
            if fighter.hp == fighter.max_hp {
                game.messages.add("You are already at full health.", RED);
                return UseResult::Cancelled;
            }
            game.messages.add("Your open wounds begin to close up!", LIGHT_GREEN);
            game.player.heal(HEAL_AMOUNT);
            return UseResult::UsedUp;
        }
        UseResult::Cancelled
    }
}
