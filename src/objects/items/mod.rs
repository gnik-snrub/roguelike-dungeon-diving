use crate::Tcod;
use crate::environment::Game;
use crate::graphics::gui::target_tile;

use super::{ Object, Character };
use crate::objects::npc::ai::Ai;

use std::collections::HashMap;
use tcod::colors::*;

use serde::{ Serialize, Deserialize };

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Heal,
    LightningBoltScroll,
    ConfusionScroll,
    FireballScroll,
}

#[derive(Serialize, Deserialize)]
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
            level: 1,
            always_visible: true,
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
        _items: &mut HashMap<i32, Object>
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
        _items: &mut HashMap<i32, Object>
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
            // Damage enemy, and give experience points to player if killed.
            if let Some(exp) = characters[monster_id].object.take_damage(lightning_damage, game) {
                player.fighter.as_mut().unwrap().exp += exp;
            }
            UseResult::UsedUp
        } else {
            // No enemy found within the max range.
            game.messages.add("No enemy is close enough to strike.", RED);
            UseResult::Cancelled
        }
    }

    // Confusion scroll creator.
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
        items: &mut HashMap<i32, Object>
    ) -> UseResult {
        // Set up spell variables.
        let confuse_range = 8;
        let confuse_num_turns = 10;
        // Asks the player to select an enemy to confuse.
        game.messages.add(
            "Left-click an enemy to confuse them, or right-click to cancel...",
            LIGHTER_HAN,
        );
        let (x, y) = match target_tile(tcod, game, characters, items, player, Some(confuse_range as f32)) {
            Some(tile_pos) => tile_pos,
            None => {
                game.messages.add("Nothing happens...", RED);
                return UseResult::Cancelled;
            }
        };

        for cha in characters {
            if cha.object.pos() == (x, y) {
                let old_ai = cha.object.ai.take().unwrap_or(Ai::Basic);
                // Replace the monster's AI with a "confused" state.
                // After some time, returns to previous AI.
                cha.object.ai = Some(Ai::Confused {
                    previous_ai: Box::new(old_ai),
                    num_turns: confuse_num_turns,
                });
                game.messages.add(
                    format!(
                        "The eyes of {} appear vacant, as it begins to stumble around!",
                        cha.object.name
                    ),
                    LIGHTER_HAN,
                );
            }
        }
        UseResult::UsedUp
    }

    // Fireball scroll creator.
    pub fn fireball_scroll(x: i32, y: i32) -> Object {
        let mut fireball_scroll = Object::new_item(x, y, '#', "Scroll of Fireball", FLAME, false);
        fireball_scroll.item = Some(Item::FireballScroll);
        fireball_scroll
    }
    // Fireball scroll use function.
    pub fn use_fireball_scroll(
        _inventory_id: usize,
        tcod: &mut Tcod,
        game: &mut Game,
        player: &mut Object,
        characters: &mut Vec<Character>,
        items: &mut HashMap<i32, Object>
    ) -> UseResult {
        // Set up spell variables.
        let fireball_radius = 3;
        let fireball_damage = 12;

        // Ask the player for a target tile to throw the fireball at.
        game.messages.add(
            "Left-click a target tile for the fireball, or right-click to cancel...",
            LIGHTER_FLAME,
        );
        let (x, y) = match target_tile(tcod, game, characters, items, player, None) {
            Some(tile_pos) => tile_pos,
            None => return UseResult::Cancelled,
        };
        game.messages.add(
            format!(
                "The fireball explodes, burning everything within {} tiles!",
                fireball_radius
            ),
            FLAME,
        );

        // Establish variable to track exp to give to player.
        let mut exp_to_gain = 0;

        // Damages the enemies in range.
        for cha in characters {
            if cha.object.distance(x, y) <= fireball_radius as f32 && cha.object.fighter.is_some() {
                game.messages.add(
                    format!(
                        "The {} us burned for {} hit points!",
                        cha.object.name, fireball_damage
                    ),
                    FLAME,
                );
                // Damage enemy, and aggregate experience points.
                if let Some(exp) = cha.object.take_damage(fireball_damage, game) {
                    exp_to_gain += exp;
                }
            }
        }
        // Give experience points to player.
        player.fighter.as_mut().unwrap().exp += exp_to_gain;

        // Also damages player, if in range.
        if player.distance(x, y) <= fireball_radius as f32 {
            game.messages.add(
                format!(
                    "You were unable to avoid the flames, and took {} damage...",
                    fireball_damage
                ),
                DARK_FLAME,
            );
            Object::player_damage(fireball_damage, game, player);
        }
        UseResult::UsedUp
    }
}
