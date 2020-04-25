use crate::environment::Game;

use tcod::colors::*;
use tcod::console::*;

#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub char: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
}

impl Object {
    // -----OBJECT MANAGEMENT FUNCTIONS-----
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Object {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
            name: name.into(),
            blocks: blocks,
            alive: false,
        }
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn move_by(id: usize, dx: i32, dy: i32, game: &Game, objects: &mut [Object]) {
        let (x, y) = objects[id].pos();
        if !Object::is_blocked(x + dx, y + dy, game, objects) {
            objects[id].set_pos(x + dx, y + dy);
        }
    }

    pub fn is_blocked(x: i32, y: i32, game: &Game, objects: &[Object]) -> bool {
        // First test the map tile
        if game.map[x as usize][y as usize].blocked {
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

    // -----FUNCTION CREATES NEW PLAYER-----
    pub fn player() -> Object {
        let player = Object::new(0, 0, '@', tcod::colors::WHITE, "Player", true);
        player
    }

    // -----LIST OF MONSTERS STARTS HERE-----
    pub fn fire_elemental(x: i32, y: i32) -> Object {
        let fire_elemental = Object::new(x, y, 'f', tcod::colors::ORANGE, "Fire Elemental", true);
        fire_elemental
    }

    pub fn crystal_lizard(x: i32, y: i32) -> Object {
        let crystal_lizard = Object::new(x, y, 'C', tcod::colors::LIGHTER_SKY, "Crystal Lizard", true);
        crystal_lizard
    }
}
