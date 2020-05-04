use crate::Tcod;
use crate::environment::{ Game };
use crate::graphics::gui::target_tile;

use super::{ Object, Character };
use super::npc::{ Fighter, DeathCallback };
use super::items::*;

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
                    level: 1,
                    exp: 0,
                    //level_up: 5,
                    max_hp: 30,
                    hp: 30,
                    defense: 2,
                    power: 5,
                    on_death: DeathCallback::Player,
                }),
                ai: None,
                item: None,
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
                    characters[target_id].object.take_damage(damage, game);
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
        let level_mod =
            (player.fighter.unwrap().level as f32).sqrt().powf((player.fighter.unwrap().level as f32) / 2.0) /
            (player.fighter.unwrap().level as f32).sqrt().powf((player.fighter.unwrap().level as f32) * 0.25);
        let damage = (attack / defense * level_mod).round() as i32;
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

    pub fn use_item(inventory_id: usize, tcod: &mut Tcod, game: &mut Game, characters: &mut Vec<Character>, player: &mut Character) {
        match &mut player.inventory {
            Some(inventory) => {
                if let Some(item) = inventory[inventory_id].item {
                    let on_use = match item {
                        Item::Heal => Object::use_health_potion,
                        Item::LightningBoltScroll => Object::use_lightning_bolt_scroll,
                        Item::ConfusionScroll => Object::use_confusion_scroll,
                    };
                    match on_use(inventory_id, tcod, game, &mut player.object, characters) {
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

    pub fn target_monster(
        tcod: &mut Tcod,
        game: &mut Game,
        characters: &[Character],
        items: &HashMap<i32, Object>,
        player: &Object,
        max_range: Option<f32>) -> Option<usize> {
        loop {
            match target_tile(tcod, game, characters, items, player, max_range) {
                Some((x, y)) => {
                    // Return the first monster clicked, or keep looping.
                    for (id, cha) in characters.iter().enumerate() {
                        if cha.object.pos() == (x, y) && cha.object.fighter.is_some() {
                            return Some(id);
                        }
                    }
                },
                None => return None,
            }
        }
    }
}
