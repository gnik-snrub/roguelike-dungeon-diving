use crate::PLAYER;
use crate::environment::{ Game, Map };

pub mod npc;
use npc::*;
use npc::ai::*;

use std::cmp::max;
use rand::Rng;

use tcod::colors::*;
use tcod::console::*;

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

    // Function to allow fighter-enabled objects to attack other fighter-enabled objects.
    pub fn attack(&mut self, target: &mut Object, game: &mut Game) {
        // Damage formula.
        let mut rng = rand::thread_rng();
        let attack = (self.fighter.map_or(0, |f| f.power)) as f32 + rng.gen_range(-1.0, 1.0);
        let defense = (target.fighter.map_or(0, |f| f.defense)) as f32 + rng.gen_range(-1.0, 1.0);
        let level_mod =
            (self.fighter.unwrap().level as f32).sqrt().powf((self.fighter.unwrap().level as f32) / 2.0) /
            (self.fighter.unwrap().level as f32).sqrt().powf((self.fighter.unwrap().level as f32) * 0.25);
        let damage = (attack / defense * level_mod).round() as i32;
        if damage > 0 {
            // Target takes damage.
            game.messages.add(
                format!(
                    "{} attacks {} dealing {} damage.",
                    self.name, target.name, damage
                ),
                self.color,
            );
            target.take_damage(damage, game);
        } else {
            game.messages.add(
                format!(
                    "{} attacks {} but it has no effect!",
                    self.name, target.name
                ),
                WHITE,
            );
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
                level: 1,
                exp: 0,
                max_hp: 30,
                hp: 30,
                defense: 2,
                power: 5,
                on_death: DeathCallback::Player,
            }),
            ai: None,
            inventory: Some(vec![]),
        }
    }

    pub fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, objects: &mut [Object]) {
        // The coordinates the player is moving to / attacking
        let x = objects[PLAYER].x + dx;
        let y = objects[PLAYER].y + dy;

        // Try to find an attackable object there
        let target_id = objects.iter().position(|object| object.fighter.is_some() && object.pos() == (x, y));

        // Attack target if found, otherwise move
        match target_id {
            Some(target_id) => {
                let (player, target) = Object::mut_two(PLAYER, target_id, objects);
                player.attack(target, game);
            },
            None => {
                Object::move_by(PLAYER, dx, dy, &game.map, objects);
            }
        }
    }
}
