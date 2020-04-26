use crate::{ PLAYER, Tcod };
use crate::environment::{ Game, Map };

pub mod npc;
use npc::*;
use npc::ai::Ai;

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

    pub fn move_by(id: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
        let (x, y) = objects[id].pos();
        if !Object::is_blocked(x + dx, y + dy, &map, objects) {
            objects[id].set_pos(x + dx, y + dy);
        }
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
                Object::move_by(PLAYER, dx, dy, &game.map, objects);
            }
        }
    }

    // ------------------------------------------
    // -----LIST OF MONSTERS GEN STARTS HERE-----
    // ------------------------------------------
    pub fn fire_elemental(x: i32, y: i32) -> Object {
        let mut fire_elemental = Object::new(x, y, 'f', tcod::colors::ORANGE, "Fire Elemental", true);
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
        let mut crystal_lizard = Object::new(x, y, 'C', tcod::colors::LIGHTER_SKY, "Crystal Lizard", true);
        crystal_lizard.fighter = Some(Fighter {
            max_hp: 16,
            hp: 16,
            defense: 3,
            power: 3,
        });
        crystal_lizard.ai = Some(Ai::Basic);
        crystal_lizard
    }

    // ----------------------------------------------
    // -----LIST OF MONSTER FUNCTIONS START HERE-----
    // ----------------------------------------------
    pub fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
        // Vector from this object to the target, and the distance.
        let dx = target_x - objects[id].x;
        let dy = target_y - objects[id].y;
        let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

        // Normalize to length 1 while keeping direction.
        // Then round, and convert to an integer so movement stays to map grid.
        let dx = (dx as f32 / distance).round() as i32;
        let dy = (dy as f32 / distance).round() as i32;
        Object::move_by(id, dx, dy, map, objects);
    }

    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &Game, objects: &mut [Object]) {
        // A basic monster takes its turn. If you can see it, it can also see you!
        let (monster_x, monster_y) = objects[monster_id].pos();
        if tcod.fov.is_in_fov(monster_x, monster_y) {
            if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
                // Move towards player if far away.
                let (player_x, player_y) = objects[PLAYER].pos();
                Object::move_towards(monster_id, player_x, player_y, &game.map, objects);
            } else if objects[PLAYER].fighter.map_or(false, |f| f.hp > 0) {
                // Close enough - Attack! (If player is alive)
                let monster = &objects[monster_id];
                println!(" The attack of the {} bounces off your mighty armor!", monster.name);
            }
        }
    }
}
