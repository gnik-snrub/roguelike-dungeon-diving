use crate::environment::{ Game, Map };

pub mod player;
pub mod npc;
use npc::*;
use npc::ai::*;

pub mod items;
use items::*;

use std::cmp::max;

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
    pub item: Option<Item>,
}

impl Object {
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

    // Find distance between self, and another target.
    pub fn distance(&self, x: i32, y: i32) -> f32 {
        (((x - self.x).pow(2) + (y - self.y).pow(2)) as f32).sqrt()
    }

    fn _mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
        assert!(first_index != second_index);
        let split_at_index = max(first_index, second_index);
        let (first_slice, second_slice) = items.split_at_mut(split_at_index);
        if first_index < second_index {
            (&mut first_slice[first_index], &mut second_slice[0])
        } else {
            (&mut second_slice[0], &mut first_slice[second_index])
        }
    }
}
