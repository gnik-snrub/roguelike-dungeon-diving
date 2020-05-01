//use crate::Tcod;
use crate::environment::{ Game, Map };

pub mod npc;
use npc::*;
use npc::ai::*;

pub mod items;
use items::*;

use std::cmp::max;
use std::collections::HashMap;
use rand::Rng;

use tcod::colors::*;
use tcod::console::*;

const HEAL_AMOUNT: i32 = 4;

// Object struct definition.
#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub char: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
    pub corpse_type: String,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
    pub inventory: Option<Vec<Object>>,
    pub item: Option<Item>,
}

impl Object {
    // -------------------------------------
    // -----OBJECT MANAGEMENT FUNCTIONS-----
    // -------------------------------------

    // Places object on the screen
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    // Checks to see if an object is meant to block other objects.
    pub fn is_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool {
        // First test the map tile
        if map[x as usize][y as usize].blocked {
            return true;
        }
        // Checks for any blocking objects
        objects.iter().any(|object| object.blocks && object.pos() == (x, y))
    }

    // Returns the x/y coordinates of the object.
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    // Sets the x/y coordinates of the object.
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    // Moves unit in a direction if the tile isn't blocked
    pub fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
        let (x, y) = objects[id].pos();
        if !Object::is_blocked(x + dx, y + dy, &map, objects) {
            objects[id].set_pos(x + dx, y + dy);
        }
    }

    // Function to allow fighter-enabled objects to take damage
    fn take_damage(&mut self, damage: i32, game: &mut Game) {
        // Apply damage if possible.
        if let Some(fighter) = self.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }

        // Check for death, and possibly call death function.
        if let Some(fighter) = self.fighter {
            if fighter.hp <= 0 {
                self.alive = false;
                fighter.on_death.callback(self, game);
            }
        }
    }

    pub fn heal(&mut self, amount: i32) {
        if let Some(ref mut fighter) = self.fighter {
            fighter.hp += amount;
            if fighter.hp > fighter.max_hp {
                fighter.hp = fighter.max_hp;
            }
        }
    }

    fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
        assert!(first_index != second_index);
        let split_at_index = max(first_index, second_index);
        let (first_slice, second_slice) = items.split_at_mut(split_at_index);
        if first_index < second_index {
            (&mut first_slice[first_index], &mut second_slice[0])
        } else {
            (&mut second_slice[0], &mut first_slice[second_index])
        }
    }

    // ----------------------------------
    // -----PLAYER RELATED FUNCTIONS-----
    // ----------------------------------

    // Creates player object.
    pub fn player() -> Object {
        Object {
            x: 0,
            y: 0,
            char: '@',
            color: WHITE,
            name: "Player".into(),
            blocks: true,
            alive: true,
            corpse_type: "'s bloody corpse".into(),
            fighter: Some(Fighter {
                level: 25,
                exp: 0,
                //level_up: 5,
                max_hp: 30,
                hp: 30,
                defense: 2,
                power: 5,
                on_death: DeathCallback::Player,
            }),
            ai: None,
            inventory: Some(Vec::new()),
            item: None,
        }
    }

    // Decides if the player object should move, or attack when inputs are entered.
    pub fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, objects: &mut [Object]) {
        // The coordinates the player is moving to / attacking
        let x = game.player.x + dx;
        let y = game.player.y + dy;

        // Try to find an attackable object there
        let target_id = objects.iter().position(|object| object.fighter.is_some() && object.pos() == (x, y));

        // Attack target if found, otherwise move
        match target_id {
            Some(target_id) => {
                let damage = Object::player_attack(&mut objects[target_id], game);
                if damage > 0 {
                    // Target takes damage.
                    game.messages.add(
                        format!(
                            "{} attacks {} dealing {} damage.",
                            game.player.name, objects[target_id].name, damage
                        ),
                        game.player.color,
                    );
                    objects[target_id].take_damage(damage, game);
                } else {
                    game.messages.add(
                        format!(
                            "{} attacks {} but it has no effect!",
                            game.player.name, objects[target_id].name
                        ),
                        WHITE,
                    );
                }
            },
            None => {
                if !Object::is_blocked(x, y, &game.map, objects) {
                    game.player.set_pos(x, y);
                }
            }
        }
    }

    // Function to allow fighter-enabled objects to attack other fighter-enabled objects.
    fn player_attack(target: &mut Object, game: &mut Game) -> i32{
        // Damage formula.
        let mut rng = rand::thread_rng();
        let attack = (game.player.fighter.map_or(0, |f| f.power)) as f32 + rng.gen_range(-1.0, 1.0);
        let defense = (target.fighter.map_or(0, |f| f.defense)) as f32 + rng.gen_range(-1.0, 1.0);
        let level_mod =
            (game.player.fighter.unwrap().level as f32).sqrt().powf((game.player.fighter.unwrap().level as f32) / 2.0) /
            (game.player.fighter.unwrap().level as f32).sqrt().powf((game.player.fighter.unwrap().level as f32) * 0.25);
        let damage = (attack / defense * level_mod).round() as i32;
        damage
    }

    pub fn player_damage(damage: i32, game: &mut Game) {
        // Apply damage if possible.
        if let Some(fighter) = game.player.fighter.as_mut() {
            if damage > 0 {
                fighter.hp -= damage;
            }
        }

        // Check for death, and possibly call death function.
        if let Some(fighter) = game.player.fighter {
            if fighter.hp <= 0 {
                game.player.alive = false;
                Object::player_death(game);
            }
        }
    }

    fn player_death(game: &mut Game) {
        // The game ended!
        game.messages.add("You died, lmao!", RED);
        game.player.char = '%';
        game.player.color = DARK_RED;
        game.player.name = format!("{}{}", game.player.name, game.player.corpse_type);
    }

    // Adds item to player's inventory, and removes from the map.
    pub fn pick_item_up(object_id: i32, game: &mut Game, characters: &mut Vec<Object>, items: &mut HashMap<i32, Object>) {
        match &mut game.player.inventory {
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
                            GREEN,
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

//    pub fn use_item(inventory_id: usize, tcod: &mut Tcod, game: &mut Game, characters: &mut Vec<Object>) {
//        let inventory = match &mut characters[PLAYER].inventory {
//            Some(stash_of_items) => stash_of_items,
//            _ => {
//                game.messages.add(
//                    "Your inventory is missing.",
//                    RED,
//                );
//                return ()
//            },
//        };
//
//        if let Some(item) = inventory[inventory_id].item {
//            let on_use = match item {
//                Item::Heal => Object::cast_heal,
//            };
//            match on_use(inventory_id, tcod, game, characters) {
//                UseResult::UsedUp => {
//                    // Destroy after use, unless it was cancelled for some reason.
//                    inventory.remove(inventory_id);
//                },
//                UseResult::Cancelled => {
//                    game.messages.add("Cancelled", WHITE);
//                }
//            }
//        } else {
//            game.messages.add(
//                format!("The {} cannot be used.", inventory[inventory_id].name),
//                WHITE,
//            );
//        }
//    }
}
