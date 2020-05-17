pub mod rectangles;
pub mod drunken_miners;
pub mod open_rectangles;
pub mod open_drunken_miners;

pub mod tiles;

use crate::environment::{ Map, MAP_WIDTH, MAP_HEIGHT };
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

    pub fn is_in_range(&self, x: i32, y: i32) -> bool {
        // Returns true if coordinates are inside rectangle
        (self.x1 <= x)
        && (self.x2 >= x)
        && (self.y1 <= y)
        && (self.y2 >= y)
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

// Creates some randomness along the outside of a rect.
pub fn mine_drunkenly(room: Rect, map: &mut Map, colors: &[Color; 7]) {
    // Establishes mining variables
    // Miners must be separate from the miner_max, as the maximum will be used later, and
    // the amount of miners will change with each internal loop.
    let miner_max = rand::thread_rng().gen_range(1, 5);
    let mut miners = miner_max;
    let tiles_to_carve = rand::thread_rng().gen_range(20, 40);

    while miners > 0 {

        // Finds the center of the room.
        let (mut x, mut y) = room.center();
        let mut tiles_carved = 0;

        // Divides the tiles to carve amongst the miners doing the work.
        while tiles_carved < (tiles_to_carve / miner_max) {

            // If the miner is on an uncarved tile, it will carve it the tile.
            if !map[x as usize][y as usize].empty {
                map[x as usize][y as usize] = Tile::empty(colors);
                tiles_carved += 1
            } else { // Otherwise, it will move to a space within the map boundary.
                let four_sided_dice = rand::thread_rng().gen_range(1, 5);
                match four_sided_dice {
                    1 => { y += 1;
                        if y <= 1 || y >= MAP_HEIGHT - 1 { y -= 1; }
                    },
                    2 => { y -= 1;
                        if y <= 1 || y >= MAP_HEIGHT - 1 { y += 1; }
                    },
                    3 => { x += 1;
                        if x <= 1 || x >= MAP_WIDTH - 1 { x -= 1; }
                    },
                    _ => { x -= 1;
                        if x <= 1 || x >= MAP_WIDTH - 1 { x += 1; }
                    },
                }
            }
        }
        // Once the miner has completed his workload, the next miner begins.
        miners -= 1
    }
}

// Creates a random mirrored pattern from the center of the map.
pub fn butterfly(map: &mut Map, colors: &[Color; 7]) {
    // Creates two instances of the center point, and amount of tiles to be carved.
    let (mut left_x, mut left_y, mut right_x, mut right_y) =
        (MAP_WIDTH / 2, MAP_HEIGHT / 2, MAP_WIDTH / 2, MAP_HEIGHT / 2);
    let mut tiles_to_carve = 250;

    // This is how many tiles will be removed per "carve"
    let brush = 2;

    // First, it removes the center tile that it begins on.
    map[left_x as usize][left_y as usize] = Tile::empty(colors);

    while tiles_to_carve > 0 {

        // Decides a random direction to move
        // If new position would be outside the map boundary, it returns to its previous position.
        let four_sided_dice = rand::thread_rng().gen_range(1, 5);
        match four_sided_dice {
            1 => {
                left_y -= 1;
                if left_y <= 3 || left_y >= MAP_HEIGHT - 4 {
                    left_y += 1;
                } else {
                    right_y -= 1;
                    tiles_to_carve -= 1;
                }
            },
            2 => {
                left_y += 1;
                if left_y <= 3 || left_y >= MAP_HEIGHT - 4 {
                    left_y -= 1;
                } else {
                    right_y += 1;
                    tiles_to_carve -= 1;
                }
            },
            3 => {
                left_x -= 1;
                if left_x <= 3 || left_x >= MAP_WIDTH / 2 {
                    left_x += 1;
                } else {
                    right_x += 1;
                    tiles_to_carve -= 1;
                }
            },
            _ => {
                left_x += 1;
                if left_x <= 3 || left_x >= MAP_WIDTH / 2 {
                    left_x -= 1;
                } else {
                    right_x -= 1;
                    tiles_to_carve -= 1;
                }
            }
        }

        // Removes the tiles according to brush size based on the new position.
        for x in (left_x - brush)..(left_x + brush) {
            for y in (left_y - brush)..(left_y + brush) {
                map[x as usize][y as usize] = Tile::empty(colors);
            }
        }

        // Also removes the tiles on the mirrored side of the map.
        for x in (right_x - brush)..(right_x + brush) {
            for y in (right_y - brush)..(right_y + brush) {
                map[x as usize][y as usize] = Tile::empty(colors);
            }
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
        if rng % 3 == 0 {
            if map[x as usize][(y + 1) as usize].block_sight == true
            && map[x as usize][(y + 2) as usize].block_sight == false
            && map[x as usize][(y + 3) as usize].block_sight == false {
                map[x as usize][(y + 1) as usize] = Tile::hidden_passage(colors);
            }
            if map[x as usize][(y - 1) as usize].block_sight == true
            && map[x as usize][(y - 2) as usize].block_sight == false
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

        // Creates secret passages, and
        let max_chance = rand::thread_rng().gen_range(0, 100);
        if max_chance % 3 == 0 {
            if map[(x + 1) as usize][y as usize].block_sight == true
            && map[(x + 2) as usize][y as usize].block_sight == false
            && map[(x + 3) as usize][y as usize].block_sight == false {
                map[(x + 1) as usize][y as usize] = Tile::hidden_passage(colors);
            }
            if map[(x - 1) as usize][y as usize].block_sight == true
            && map[(x - 2) as usize][y as usize].block_sight == false
            && map[(x - 3) as usize][y as usize].block_sight == false {
                map[(x - 1) as usize][y as usize] = Tile::hidden_passage(colors);
            }
        }
    }
}

pub fn create_tunnels(rooms: &Vec<Rect>, mut map: &mut Map, colors: &[Color; 7]) {

    let mut keep_connecting = true;
    let mut room_num = 0;
    let total_rooms = rooms.len() - 1;

    while keep_connecting {
        if room_num + 1 > total_rooms {
            keep_connecting = false;
        } else {
            let (x1, y1) = rooms[room_num].center();
            let (x2, y2) = rooms[room_num + 1].center();

            if rand::random() {
                // Horizontal tunnel first
                create_h_tunnel(x1, x2, y1, &mut map, &colors);
                create_v_tunnel(y1, y2, x2, &mut map, &colors);
            } else {
                // Vertical tunnel first
                create_v_tunnel(y1, y2, x1, &mut map, &colors);
                create_h_tunnel(x1, x2, y2, &mut map, &colors);
            }
            room_num += 1;
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
