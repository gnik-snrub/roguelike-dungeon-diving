use crate::Tcod;
use crate::environment::Game;

use super::{ Object, Character };
use crate::objects::npc::ai::Ai;

use tcod::colors::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Heal,
    LightningBoltScroll,
    ConfusionScroll,
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
        player: &mut Object,
        _characters: &mut Vec<Character>,
    ) -> UseResult {
        // Establish the healing value.
        let heal_amount = 4;
        // Heal the player
        if let Some(fighter) = player.fighter {
            if fighter.hp == fighter.max_hp {
                game.messages.add("You are already at full health.", RED);
                return UseResult::Cancelled;
            }
            game.messages.add("Your wounds start to feel better!", LIGHT_GREEN);
            player.heal(heal_amount);
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
        player: &mut Object,
        characters: &mut Vec<Character>,
    ) -> UseResult {
        // Find the closest enemy (within the max range).
        let lightning_range = 5;
        let lightning_damage = 40;
        let monster_id = Object::closest_monster(player, tcod, characters, lightning_range);
        if let Some(monster_id) = monster_id {
            // ZzzzzzzzzaAAP!~~
            game.messages.add(
                format!(
                    "A lightning bolt strikes the {} with a loud thunder!",
                    characters[monster_id].object.name
                ),
                LIGHT_CYAN,
            );
            game.messages.add(
                format!(
                    "The damage is {} hit points.",
                    lightning_damage
                ),
                LIGHT_CYAN,
            );
            characters[monster_id].object.take_damage(lightning_damage, game);
            UseResult::UsedUp
        } else {
            // No enemy found within the max range.
            game.messages.add("No enemy is close enough to strike.", RED);
            UseResult::Cancelled
        }
    }

    // Lightning bolt scroll creator.
    pub fn confusion_scroll(x: i32, y: i32) -> Object {
        let mut confusion_scroll = Object::new_item(x, y, '#', "Scroll of confusion", LIGHT_HAN, false);
        confusion_scroll.item = Some(Item::ConfusionScroll);
        confusion_scroll
    }
    // Lightning bolt scroll use function.
    pub fn use_confusion_scroll(
        _inventory_id: usize,
        tcod: &mut Tcod,
        game: &mut Game,
        player: &mut Object,
        characters: &mut Vec<Character>,
    ) -> UseResult {
        // Set up spell variables.
        let confuse_range = 8;
        let confuse_num_turns = 10;
        // Find closest enemy in range, and confuse it.
        let monster_id = Object::closest_monster(player, tcod, characters, confuse_range);
        if let Some(monster_id) = monster_id {
            let old_ai = characters[monster_id].object.ai.take().unwrap_or(Ai::Basic);
            // Replace the monster's AI with a "confused" state.
            // After some time, returns to previous AI.
            characters[monster_id].object.ai = Some(Ai::Confused {
                previous_ai: Box::new(old_ai),
                num_turns: confuse_num_turns,
            });
            game.messages.add(
                format!(
                    "The eyes of {} appear vacant, as it begins to stumble around!",
                    characters[monster_id].object.name
                ),
                LIGHT_HAN,
            );
            UseResult::UsedUp
        } else {
            game.messages.add("No enemy is close enough to be confused.", RED);
            UseResult::Cancelled
        }
    }
}
