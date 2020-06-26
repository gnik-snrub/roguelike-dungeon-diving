pub mod rectangles;
pub mod modifiers;
pub mod drunk_walk;
pub mod cellular_automata;
pub mod maze;

pub mod tiles;

use crate::graphics::render_map;
use crate::Tcod;
use crate::environment::{ Map, MAP_WIDTH, MAP_HEIGHT };
use crate::environment::tiles::Tile;

use crate::Point;

use std::collections::HashMap;
use std::cmp::Ordering;

use std::cmp;
use tcod::colors::*;

use serde::{ Serialize, Deserialize };

// Struct definition for Rectangles
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

// Ord, PartialOrd, and PartialEq are implemented to allow for Rects to be sorted.
impl Ord for Rect {
    fn cmp(&self, other: &Self) -> Ordering {
        self.center().cmp(&other.center())
    }
}
impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.center() == other.center()
    }
}

impl Rect {
    // Constructor for Rects
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // Returns the center point of a Rect.
    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    // Checks to see if one Rect intersects with another.
    pub fn intersects_with(&self, other: &Rect) -> bool {
        // Returns true if this rectangle intersects with another
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

// Places a rect onto a map.
pub fn create_room(room: Rect, map: &mut Map, colors: &[Color; 7]) {
    // Go through the tiles in the rectangle and make them passable.
    // Note: the +1's are to allow to for a wall around the rectangle.
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty(colors);
        }
    }
}

// Draws a horizontal line of empty tiles on the map.
pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map, colors: &[Color; 7]) {
    // Horizontal tunnel. 'min()' and 'max()' are used in case 'x1 > x2'
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty(colors);
    }
}

// Draws a vertical line of empty tiles on the map.
pub fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map, colors: &[Color; 7]) {
    // Vertical tunnel. Functions essentially the same as the horizontal tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty(colors);
    }
}

// Draws a horizontal line of secret path tiles on the map.
pub fn create_secret_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map, colors: &[Color; 7]) {
    // Horizontal tunnel. 'min()' and 'max()' are used in case 'x1 > x2'
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        if map[x as usize][y as usize].wall {
            map[x as usize][y as usize] = Tile::hidden_passage(colors);
        }
    }
}

// Draws a vertical line of secret path tiles on the map.
pub fn create_secret_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map, colors: &[Color; 7]) {
    // Vertical tunnel. Functions essentially the same as the horizontal tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        if map[x as usize][y as usize].wall {
            map[x as usize][y as usize] = Tile::hidden_passage(colors);
        }
    }
}

pub fn create_tunnels(rooms: &mut Vec<Rect>, mut map: &mut Map, colors: &[Color; 7], tcod: &mut Tcod, should_render: bool) {

    // Variables to keep track of depth in the rooms vector, and if the loop should continue.
    let mut keep_connecting = true;
    let mut room_num = 0;
    let total_rooms = rooms.len() - 1;

    while keep_connecting {
        let (x1, y1) = rooms[room_num].center();
        let (x2, y2) = rooms[room_num + 1].center();

        if rand::random() {
            // Horizontal tunnel first
            create_h_tunnel(x1, x2, y1, &mut map, &colors);
            create_v_tunnel(y1, y2, x2, &mut map, &colors);

            if rand::random() {
                // Horizontal secret tunnel first
                create_secret_v_tunnel(y1, y2, x1, &mut map, &colors);
                create_secret_h_tunnel(x1, x2, y2, &mut map, &colors);
            }
        } else {
            // Vertical tunnel first
            create_v_tunnel(y1, y2, x1, &mut map, &colors);
            create_h_tunnel(x1, x2, y2, &mut map, &colors);

            if rand::random() {
                // Vertical secret tunnel first
                create_secret_h_tunnel(x1, x2, y1, &mut map, &colors);
                create_secret_v_tunnel(y1, y2, x2, &mut map, &colors);
            }
        }

        if should_render {
            render_map(tcod, map, 4);
        }

        room_num += 1;

        // If the previous room was the last, end the loop.
        if room_num + 1 > total_rooms {
            keep_connecting = false;
        }
    }
}

// Generic sorting algorithm. As far as I can think, it's just for rooms so far though.
pub fn room_sorter<T: Ord>(rooms: &mut Vec<T>) {
    // Iterates through each item in the vector
    for i in 1..rooms.len() {
        // Searches vector until point i
        for j in (1..i + 1).rev() {
            // If previous item is smaller than the current, it does nothing
            if rooms[j - 1] <= rooms[j] { break; }
            // Otherwise, it swaps the items.
            rooms.swap(j - 1, j);
        }
    }
}

pub fn joiner(points: &mut Vec<(i32, i32)>, mut map: &mut Map, colors: &[Color; 7], tcod: &mut Tcod, should_render: bool) {

    // Variables to keep track of the depth of the vector, and if the loop should continue.
    let mut keep_connecting = true;
    let mut point_num = 0;
    let total_points = points.len() - 1;

    while keep_connecting {
        let (x1, y1) = points[point_num];
        let (x2, y2) = points[point_num + 1];

        if rand::random() {
            // Horizontal tunnel first
            create_h_tunnel(x1, x2, y1, &mut map, &colors);
            create_v_tunnel(y1, y2, x2, &mut map, &colors);
        } else {
            // Vertical tunnel first
            create_v_tunnel(y1, y2, x1, &mut map, &colors);
            create_h_tunnel(x1, x2, y2, &mut map, &colors);
        }

        if should_render {
            render_map(tcod, map, 4);
        }

        point_num += 1;

        if point_num + 1 > total_points {
            keep_connecting = false;
        }
    }
}

pub fn cull_tiles(map: &mut Map, colors: &[Color; 7], points: &HashMap<Point, Point>) {
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            match points.keys().find(|found| **found == (x as u32, y as u32)) {
                Some((x, y)) => {
                    map[*x as usize][*y as usize] = Tile::wall(colors);
                },
                None => {},
            }

//            if !map[x as usize][y as usize].found {
//                map[x as usize][y as usize] = Tile::wall(colors);
//            }
        }
    }
}
