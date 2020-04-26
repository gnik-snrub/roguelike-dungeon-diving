use crate::PLAYER;
use crate::environment::{ Game, Map };

pub mod npc;
pub mod npc::ai;
use npc::*;
use ai::*;

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
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
}

impl Object {
    // -------------------------------------
    // -----OBJECT MANAGEMENT FUNCTIONS-----
    // -------------------------------------
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Object {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
            name: name.into(),
            blocks: blocks,
            alive: false,
            fighter: None,
            ai: None,
        }
    }

    // Places object on the screen
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn move_by(id: usize, dx: i32, dy: i32, game: &Game, objects: &mut [Object]) {
        let (x, y) = objects[id].pos();
        if !Object::is_blocked(x + dx, y + dy, &game.map, objects) {
            objects[id].set_pos(x + dx, y + dy);
        }
    }

    pub fn is_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool {
        // First test the map tile
        if map[x as usize][y as usize].blocked {
            return true;
        }
        // Checks for any blocking objects
        objects.iter().any(|object| object.blocks && object.pos() == (x, y))
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    // ----------------------------------
    // -----PLAYER RELATED FUNCTIONS-----
    // ----------------------------------
    pub fn player() -> Object {
        let mut player = Object::new(0, 0, '@', tcod::colors::WHITE, "Player", true);
        player.alive = true;
        player.fighter = Some(Fighter {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        });
        player
    }

    pub fn player_move_or_attack(dx: i32, dy: i32, game: &Game, objects: &mut [Object]) {
        // The coordinates the player is moving to / attacking
        let x = objects[PLAYER].x + dx;
        let y = objects[PLAYER].y + dy;

        // Try to find an attackable object there
        let target_id = objects.iter().position(|object| object.pos() == (x, y));

        // Attack target if found, otherwise move
        match target_id {
            Some(target_id) => {
                println!("The {} resists your futile attacks!", objects[target_id].name);
            },
            None => {
                Object::move_by(PLAYER, dx, dy, &game, objects);
            }
        }
    }

    // --------------------------------------
    // -----LIST OF MONSTERS STARTS HERE-----
    // --------------------------------------
    pub fn fire_elemental(x: i32, y: i32) -> Object {
        let fire_elemental = Object::new(x, y, 'f', tcod::colors::ORANGE, "Fire Elemental", true);
        fire_elemental.fighter = Some(Fighter {
            max_hp: 10,
            hp: 10,
            defense: 0,
            power: 4,
        });
        fire_elemental.ai = Some(Ai::Basic);
        fire_elemental
    }

    pub fn crystal_lizard(x: i32, y: i32) -> Object {
        let crystal_lizard = Object::new(x, y, 'C', tcod::colors::LIGHTER_SKY, "Crystal Lizard", true);
        crystal_lizard.fighter = Some(Fighter {
            max_hp: 16,
            hp: 16,
            defense: 3,
            power: 3.
        });
        crystal_lizard.ai = Some(Ai::Basic);
        crystal_lizard
    }
}
