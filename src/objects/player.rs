use crate::{ LEVEL_UP_FACTOR, LEVEL_UP_BASE, LEVEL_SCREEN_WIDTH };
use crate::Tcod;
use crate::environment::{ Game };

use super::{ Object, Character };
use super::npc::{ Fighter, DeathCallback };
use super::items::*;
use crate::graphics::gui::menu::menu;

use std::collections::HashMap;
use rand::Rng;

use tcod::colors::*;

impl Object {
    pub fn new_player() -> Character {
        Character {
            object: Object {
                x: 0,
                y: 0,
                char: '@',
                color: WHITE,
                name: "Player".into(),
                blocks: true,
                alive: true,
                corpse_type: "'s bloody corpse".into(),
                fighter: Some(Fighter {
                    exp: 0,
                    max_hp: 100,
                    hp: 100,
                    defense: 200,
                    power: 500,
                    on_death: DeathCallback::Player,
                }),
                ai: None,
                item: None,
                level: 10000,
                always_visible: false,
            },
            inventory: Some(Vec::new()),
        }
    }
    // Decides if the player object should move, or attack when inputs are entered.
    pub fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, characters: &mut [Character], player: &mut Object) {
        // The coordinates the player is moving to / attacking
        let x = player.x + dx;
        let y = player.y + dy;

        // Try to find an attackable object there
        let target_id = characters.iter().position(|cha| cha.object.fighter.is_some() && cha.object.pos() == (x, y));

        // Attack target if found, otherwise move
        match target_id {
            Some(target_id) => {
                let damage = Object::player_attack(&mut characters[target_id].object, player);
                if damage > 0 {
                    // Target takes damage.
                    game.messages.add(
                        format!(
                            "{} attacks {} dealing {} damage.",
                            player.name, characters[target_id].object.name, damage
                        ),
                        player.color,
                    );
                    if let Some(exp) = characters[target_id].object.take_damage(damage, game) {
                        player.fighter.as_mut().unwrap().exp += exp;
                    }
                } else {
                    game.messages.add(
                        format!(
                            "{} attacks {} but it has no effect!",
                            player.name, characters[target_id].object.name
                        ),
                        WHITE,
                    );
                }
            },
            None => {
                if !Object::is_blocked(x, y, &game.map, characters) {
                    player.set_pos(x, y);
                }
            }
        }
    }

    // Function to allow fighter-enabled objects to attack other fighter-enabled objects.
    fn player_attack(target: &mut Object, player: &Object) -> i32{
        // Damage formula.
        let mut rng = rand::thread_rng();
        let attack = (player.fighter.map_or(0, |f| f.power)) as f32 + rng.gen_range(-1.0, 1.0);
        let defense = (target.fighter.map_or(0, |f| f.defense)) as f32 + rng.gen_range(-1.0, 1.0);
        let mut level_mod = ((player.level - target.level) / 3) as f32;
        if level_mod <= 0.0 { level_mod = 1.0; }

        let damage = ((attack * level_mod) - defense).round() as i32;
        damage
    }

    pub fn player_damage(damage: i32, game: &mut Game, player: &mut Object) {
        // Apply damage if possible.
        if let Some(fighter) = player.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }

        // Check for death, and possibly call death function.
        if let Some(fighter) = player.fighter {
            if fighter.hp <= 0 {
                player.alive = false;
                Object::player_death(player, game)
            }
        }
    }

    fn player_death(player: &mut Object, game: &mut Game) {
        // The game ended!
        game.messages.add("You died, lmao!", RED);
        player.char = '%';
        player.color = DARK_RED;
        player.name = format!("{}{}", player.name, player.corpse_type);
    }

    pub fn level_up(tcod: &mut Tcod, game: &mut Game, player: &mut Object) {
        let level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;

        // Check if the player has enough exp to level up.
        if player.fighter.as_ref().map_or(0, |f| f.exp) >= level_up_xp {
            // Success - Level up!
            player.level += 1;
            game.messages.add(
                format!(
                    "Your power grows - You have reached level {}!",
                    player.level
                ),
                GOLD,
            );
            let fighter = player.fighter.as_mut().unwrap();
            let mut choice = None;
            while choice.is_none() {
                // Continuously requests for a choice to be made, until it is made.
                choice = menu(
                    "Level up! Choose a state to raise:\n",
                    &[
                        format!("Constitution (+20 HP, from {})", fighter.max_hp),
                        format!("Strength (+1 Attack, from {})", fighter.power),
                        format!("Agility (+1 Defense, from {})", fighter.defense),
                    ],
                    LEVEL_SCREEN_WIDTH,
                    &mut tcod.root,
                );
            }
            fighter.exp -= level_up_xp;
            match choice.unwrap() {
                0 => {
                    fighter.max_hp += 20;
                    fighter.hp += 20;
                },
                1 => {
                    fighter.power += 1;
                },
                2 => {
                    fighter.defense += 1;
                },
                _ => unreachable!(),
            }
        }
    }

    // Adds item to player's inventory, and removes from the map.
    pub fn pick_item_up(object_id: i32, game: &mut Game, items: &mut HashMap<i32, Object>, player: &mut Character) {
        match &mut player.inventory {
            Some(inventory) => if inventory.len() >= 26 {
                game.messages.add(
                    format!("Your inventory is full!"),
                    RED,
                );
            } else {
                let wrapped = items.remove(&object_id);
                match wrapped {
                    Some(pick_up_item) => {
                        game.messages.add(
                            format!("You picked found a {}", pick_up_item.name),
                            pick_up_item.color,
                        );
                        inventory.push(pick_up_item);
                    },
                    _ => (),
                }
            }
            None => game.messages.add(
                format!("You don't have access to your inventory"),
                RED,
            )
        }
    }

    pub fn use_item(
        inventory_id: usize,
        tcod: &mut Tcod,
        game: &mut Game,
        characters: &mut Vec<Character>,
        player: &mut Character,
        items: &mut HashMap<i32, Object>
    ) {
        match &mut player.inventory {
            Some(inventory) => {
                if let Some(item) = inventory[inventory_id].item {
                    let on_use = match item {
                        Item::Heal => Object::use_health_potion,
                        Item::LightningBoltScroll => Object::use_lightning_bolt_scroll,
                        Item::ConfusionScroll => Object::use_confusion_scroll,
                        Item::FireballScroll => Object::use_fireball_scroll,
                        Item::HpUp => Object::use_health_up,
                        Item::PowUp => Object::use_power_up,
                        Item::DefUp => Object::use_defense_up,
                    };
                    match on_use(inventory_id, tcod, game, &mut player.object, characters, items) {
                        UseResult::UsedUp => {
                            // Destroy after use, unless it was cancelled for some reason.
                            inventory.remove(inventory_id);
                        },
                        UseResult::Cancelled => {
                            game.messages.add("Cancelled", WHITE);
                        }
                    }
                }
            },
            _ => game.messages.add("The item cannot be used.", WHITE),
        }
    }

    pub fn drop_item(
        inventory_id: usize,
        game: &mut Game,
        items: &mut HashMap<i32, Object>,
        player: &mut Character
    ) {
        // Finds player location so that the item appears on the same tile.
        let (x, y) = player.object.pos();

        // Pull the inventory from the "Some" allowing access to the item.
        match &mut player.inventory {
            Some(inventory) => {
                // Removes item from inventory.
                let mut item = inventory.remove(inventory_id);

                // Sets item position to the player position.
                item.set_pos(x, y);
                game.messages.add(format!("You dropped a {}.", item.name), YELLOW);

                // Loops to find an empty key in the item hashmap.
                let mut new_id = 1;
                for _ in 0..items.len() {
                    if items.contains_key(&new_id) {
                        new_id += 1;
                    } else {
                        break;
                    }
                }
                // Inserts the item into the hashmap with its new id.
                items.insert(new_id, item);
            },
            // Do nothing if the inventory is inaccessible.
            _ => (()),
        }
    }

    // Find closest enemy, up to a max range, within the player FOV.
    pub fn closest_monster(player: &Object, tcod: &Tcod, objects: &[Character], max_range: i32) -> Option<usize> {
        let mut closest_enemy = None;
        let mut closest_dist = (max_range + 1) as f32; // Start with clightly more than max range.

        for (id, character) in objects.iter().enumerate() {
            let obj_ref = &character.object;
            if obj_ref.fighter.is_some() &&
            obj_ref.ai.is_some() &&
            tcod.fov.is_in_fov(obj_ref.x, obj_ref.y) {
                // Calculates distance between this object and player.
                let dist = player.distance_to(obj_ref);
                if dist < closest_dist {
                    // It's closer, so remember this one.
                    closest_enemy = Some(id);
                    closest_dist = dist;
                }
            }
        }
        closest_enemy
    }

    // Calculates distance from one object to specific x/y coordinates
    pub fn distance_from_object(&self, x: i32, y: i32) -> f32 {
        let dx = self.x - x;
        let dy = self.y - y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }
}
