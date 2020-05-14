pub mod tiles;

use crate::environment::Map;
use crate::environment::tiles::Tile;
use crate::objects::Object;

use std::cmp;
use tcod::colors::*;
use rand::*;

use serde::{ Serialize, Deserialize };

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        // Returns true if this rectangle intersects with another
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

pub fn create_room(room: Rect, map: &mut Map, colors: &[Color; 7]) {
    // Go through the tiles in the rectangle and make them passable.
    // Note: the +1's are to allow to for a wall around the rectangle.
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty(colors);
        }
    }
}

pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map, colors: &[Color; 7]) {
    // Horizontal tunnel. 'min()' and 'max()' are used in case 'x1 > x2'
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        // Creates core tunnel.
        map[x as usize][y as usize] = Tile::empty(colors);

        // Generates secret passages around tunnel, if possible.
        let rng = rand::thread_rng().gen_range(0, 100);
        if rng % 2 == 0 {
            if map[x as usize][(y + 2) as usize].block_sight == false
            && map[x as usize][(y + 3) as usize].block_sight == false {
                map[x as usize][(y + 1) as usize] = Tile::hidden_passage(colors);
            }
            if map[x as usize][(y - 2) as usize].block_sight == false
            && map[x as usize][(y - 3) as usize].block_sight == false {
                map[x as usize][(y - 1) as usize] = Tile::hidden_passage(colors);
            }
        }
    }
}

pub fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map, colors: &[Color; 7]) {
    // Vertical tunnel. Functions essentially the same as the horizontal tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty(colors);

        let max_chance = rand::thread_rng().gen_range(0, 100);
        if max_chance % 2 == 0 {
            if map[(x + 2) as usize][y as usize].block_sight == false
            && map[(x + 3) as usize][y as usize].block_sight == false {
                map[(x + 1) as usize][y as usize] = Tile::hidden_passage(colors);
            }
            if map[(x - 2) as usize][y as usize].block_sight == false
            && map[(x - 3) as usize][y as usize].block_sight == false {
                map[(x - 1) as usize][y as usize] = Tile::hidden_passage(colors);
            }
        }
    }
}

pub fn create_stairs(x: i32, y: i32) -> Object {
    Object {
        x: x,
        y: y,
        char: '<',
        color: WHITE,
        name: "Stairs".into(),
        blocks: false,
        alive: false,
        corpse_type: "Stairs".into(),
        fighter: None,
        ai: None,
        item: None,
        level: 1,
        always_visible: true,
    }
}
