use crate::Tcod;
use crate::environment::Game;
use super::npc::Fighter;

use super::Object;

use tcod::colors::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Heal,
    LightningBoltScroll,
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
    // Health potion use function.
    pub fn use_health_potion(
        _inventory_id: usize,
        _tcod: &mut Tcod,
        game: &mut Game,
        p_fighter: &mut Option<Fighter>,
        _player_x: i32,
        _player_y: i32,
        _characters: &mut Vec<Object>,
    ) -> UseResult {
        // Heal the player.
        let heal_amount = 4;
        if let Some(fighter) = p_fighter {
            if fighter.hp == fighter.max_hp {
                game.messages.add("You are already at full health.", RED);
                return UseResult::Cancelled;
            }
            game.messages.add("Your open wounds begin to close up!", LIGHT_GREEN);
            fighter.heal(heal_amount);
            return UseResult::UsedUp;
        }
        UseResult::Cancelled
    }

    // Lightning bolt scroll creator.
    pub fn lightning_bolt_scroll(x: i32, y: i32) -> Object {
        let mut lightning_bolt_scroll = Object::new_item(x, y, '#', "Scroll of lightning bolt", LIGHT_YELLOW, false);
        lightning_bolt_scroll.item = Some(Item::LightningBoltScroll);
        lightning_bolt_scroll
    }
    // Lightning bolt scroll use function.
    pub fn use_lightning_bolt_scroll(
        _inventory_id: usize,
        tcod: &mut Tcod,
        game: &mut Game,
        _p_fighter: &mut Option<Fighter>,
        player_x: i32,
        player_y: i32,
        characters: &mut Vec<Object>,
    ) -> UseResult {
        // Find the closest enemy (within the max range).
        let lightning_range = 5;
        let lightning_damage = 40;
        let monster_id = Object::closest_monster(player_x, player_y, tcod, characters, lightning_range);
        if let Some(monster_id) = monster_id {
            // ZzzzzzzzzaAAP!~~
            game.messages.add(
                format!(
                    "A lightning bolt strikes the {} with a loud thunder! \n
                    The damage is {} hit points.",
                    characters[monster_id].name, lightning_damage
                ),
                LIGHT_BLUE,
            );
            characters[monster_id].take_damage(lightning_damage, game);
            UseResult::UsedUp
        } else {
            // No enemy found within the max range.
            game.messages.add("No enemy is close enough to strike.", RED);
            UseResult::Cancelled
        }
    }
}
