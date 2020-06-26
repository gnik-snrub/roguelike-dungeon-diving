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
    FearScroll,
    HpUp,
    PowUp,
    DefUp,
}

// Used to determine what happens to an item after it is used.
#[derive(Serialize, Deserialize)]
pub enum UseResult {
    UsedUp,
    Cancelled,
}

impl Object {
    // Generic item constructor.
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

    // Health Potion constructor.
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
        // Establish the healing value of the item.
        let heal_amount = 40;
        // Accesses the fighter component of the player.
        if let Some(fighter) = player.fighter {
            // If fighter hp is at max, the item usage is cancelled.
            if fighter.hp == fighter.max_hp {
                game.messages.add("You are already at full health.", RED);
                return UseResult::Cancelled;
            }
            // Otherwise, the player is healed by the heal amount.
            game.messages.add("Your wounds start to feel better!", LIGHT_GREEN);
            player.heal(heal_amount);
            return UseResult::UsedUp;
        }
        // If the players fighter component is inaccessible, the item usage is cancelled.
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
        // Establish damage variables, range, and closest enemy within range.
        let lightning_range = 5;
        let lightning_damage = 40;
        let monster_id = Object::closest_monster(player, tcod, characters, lightning_range);

        // If monster is found, continue the item effect
        if let Some(monster_id) = monster_id {
            // Damge effect messages.
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
            // Item is destroyed.
            UseResult::UsedUp
        // No monster is found in range.
        } else {
            // Displays message, and item usage is cancelled.
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
        // Checks to see that the tile which the player clicks on is in the specified range
        let (x, y) = match target_tile(tcod, game, characters, items, player, Some(confuse_range as f32)) {
            Some(tile_pos) => tile_pos,
            // If not in range, the item usage is cancelled.
            None => {
                game.messages.add("Nothing happens...", RED);
                return UseResult::Cancelled;
            }
        };

        // Character vector is searched through.
        for cha in characters {
            // If character position matches the tile which was clicked on, the item usage happens.
            if cha.object.pos() == (x, y) {
                // Removes the AI of the monster to be inserted into the "confused AI" state.
                // This is done, so that the confused state knows which AI to return to.
                let old_ai = cha.object.ai.take().unwrap_or(Ai::Basic); // If this fails, it defaults to Basic AI.
                // Replace the monster's AI with a "confused" state.
                // After some time, returns to previous AI.
                cha.object.ai = Some(Ai::Confused {
                    previous_ai: Box::new(old_ai),
                    num_turns: confuse_num_turns,
                });
                // Displays a message showing that the monster has become confused.
                game.messages.add(
                    format!(
                        "The eyes of {} appear vacant, as it begins to stumble around!",
                        cha.object.name
                    ),
                    LIGHTER_HAN,
                );
            }
        }
        // Item is used up, and removed from the inventory.
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
        let fireball_damage = 25;

        // Ask the player for a target tile to throw the fireball at.
        game.messages.add(
            "Left-click a target tile for the fireball, or right-click to cancel...",
            LIGHTER_FLAME,
        );
        // A check is done to ensure the player clicked within the right radius.
        let (x, y) = match target_tile(tcod, game, characters, items, player, None) {
            Some(tile_pos) => tile_pos,
            // If outside the radius, item usage is cancelled.
            None => return UseResult::Cancelled,
        };
        // Explosion message is stated.
        game.messages.add(
            format!(
                "The fireball explodes, burning everything within {} tiles!",
                fireball_radius
            ),
            FLAME,
        );

        // Establish variable to track exp to give to player.
        let mut exp_to_gain = 0;

        // Searches through character vector
        for cha in characters {
            // If character is within the radius of the explosion, the item effect happens to them.
            if cha.object.distance(x, y) <= fireball_radius as f32 && cha.object.fighter.is_some() {
                // Message to show that the relevant character was damaged.
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
        // Item is used up, and removed from inventory.
        UseResult::UsedUp
    }

    // Fear scroll creator.
    pub fn fear_scroll(x: i32, y: i32) -> Object {
        let mut fear_scroll = Object::new_item(x, y, '#', "Scroll of Fear", DARKER_PURPLE, false);
        fear_scroll.item = Some(Item::FearScroll);
        fear_scroll
    }
    // Fear scroll use function.
    pub fn use_fear_scroll(
        _inventory_id: usize,
        tcod: &mut Tcod,
        game: &mut Game,
        player: &mut Object,
        characters: &mut Vec<Character>,
        items: &mut HashMap<i32, Object>
    ) -> UseResult {
        // Set up spell variables.
        let fear_range = 8;
        let fear_num_turns = 10;
        // Asks the player to select an enemy to confuse.
        game.messages.add(
            "Left-click an enemy to instill with fear, or right-click to cancel...",
            DARKER_PURPLE,
        );
        // Checks to see that the tile which the player clicks on is in the specified range
        let (x, y) = match target_tile(tcod, game, characters, items, player, Some(fear_range as f32)) {
            Some(tile_pos) => tile_pos,
            // If not in range, the item usage is cancelled.
            None => {
                game.messages.add("Nothing happens...", RED);
                return UseResult::Cancelled;
            }
        };

        // Character vector is searched through.
        for cha in characters {
            // If character position matches the tile which was clicked on, the item usage happens.
            if cha.object.pos() == (x, y) {
                // Removes the AI of the monster to be inserted into the "confused AI" state.
                // This is done, so that the confused state knows which AI to return to.
                let old_ai = cha.object.ai.take().unwrap_or(Ai::Basic); // If this fails, it defaults to Basic AI.
                // Replace the monster's AI with a "confused" state.
                // After some time, returns to previous AI.
                cha.object.ai = Some(Ai::Fear {
                    previous_ai: Box::new(old_ai),
                    num_turns: fear_num_turns,
                });
                // Displays a message showing that the monster has become confused.
                game.messages.add(
                    format!(
                        "The eyes of {} grow wide, as it freezes in terror!",
                        cha.object.name
                    ),
                    DARKER_PURPLE,
                );
            }
        }
        // Item is used up, and removed from the inventory.
        UseResult::UsedUp
    }

    pub fn health_up(x: i32, y: i32) -> Object {
        let mut health_up = Object::new_item(x, y, '/', "Kale", LIGHTER_LIME, false);
        health_up.item = Some(Item::HpUp);
        health_up
    }
    // Health up use function.
    pub fn use_health_up(
        _inventory_id: usize,
        _tcod: &mut Tcod,
        game: &mut Game,
        player: &mut Object,
        _characters: &mut Vec<Character>,
        _items: &mut HashMap<i32, Object>
    ) -> UseResult {
        // Buff the player
        if let Some(ref mut fighter) = player.fighter {
            game.messages.add("You eat the kale, and immediately feel healthier.", LIGHTEST_LIME);
            fighter.max_hp += 5 * (game.dungeon_level / 5) as i32;
            return UseResult::UsedUp;
        }
        UseResult::Cancelled
    }

    pub fn power_up(x: i32, y: i32) -> Object {
        let mut power_up = Object::new_item(x, y, '+', "Creatine Powder", LIGHT_CRIMSON, false);
        power_up.item = Some(Item::PowUp);
        power_up
    }
    // Power up use function.
    pub fn use_power_up(
        _inventory_id: usize,
        _tcod: &mut Tcod,
        game: &mut Game,
        player: &mut Object,
        _characters: &mut Vec<Character>,
        _items: &mut HashMap<i32, Object>
    ) -> UseResult {
        // Buff the player
        if let Some(ref mut fighter) = player.fighter {
            game.messages.add("You consume the creatine, and your shirt tears a little bit.", LIGHTER_CRIMSON);
            fighter.power += 1 * (game.dungeon_level / 10) as i32;
            return UseResult::UsedUp;
        }
        UseResult::Cancelled
    }

    pub fn defense_up(x: i32, y: i32) -> Object {
        let mut defense_up = Object::new_item(x, y, '~', "Quinoa", PURPLE, false);
        defense_up.item = Some(Item::DefUp);
        defense_up
    }
    // Defense up use function.
    pub fn use_defense_up(
        _inventory_id: usize,
        _tcod: &mut Tcod,
        game: &mut Game,
        player: &mut Object,
        _characters: &mut Vec<Character>,
        _items: &mut HashMap<i32, Object>
    ) -> UseResult {
        // Buff the player
        if let Some(ref mut fighter) = player.fighter {
            game.messages.add("You eat the quinoa, and feel your energy strengthen.", LIGHT_PURPLE);
            fighter.defense += 1 * (game.dungeon_level / 10) as i32;
            return UseResult::UsedUp;
        }
        UseResult::Cancelled
    }
}
