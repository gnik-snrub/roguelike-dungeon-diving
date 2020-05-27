use crate::environment::{ Map, MAP_WIDTH, MAP_HEIGHT };
use crate::environment::tiles::Tile;
use crate::environment::map::Rect;

use tcod::colors::*;
use rand::*;

// Creates some randomness along the outside of a rect.
pub fn mine_drunkenly(rooms: &Vec<Rect>, map: &mut Map, colors: &[Color; 7]) {
    for room in rooms {
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
                            if y >= MAP_HEIGHT - 1 { y -= 1; }
                        },
                        2 => { y -= 1;
                            if y <= 1 { y += 1; }
                        },
                        3 => { x += 1;
                            if x >= MAP_WIDTH - 1 { x -= 1; }
                        },
                        _ => { x -= 1;
                            if x <= 1 { x += 1; }
                        },
                    }
                }
            }
            // Once the miner has completed his workload, the next miner begins.
            miners -= 1
        }
    }
}

// Below are various forms of similar modifiers
pub fn caved_in(map: &mut Map, colors: &[Color; 7]) {
    // Randomly decides what type of cave-in occurs.
    if rand::random() {
        butterfly(map, &colors);
    } else {
        random_hole(map, &colors);
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
                if left_y <= (brush + 1) {
                    left_y += 1;
                } else {
                    right_y -= 1;
                    tiles_to_carve -= 1;
                }
            },
            2 => {
                left_y += 1;
                if left_y >= MAP_HEIGHT - (brush + 1) {
                    left_y -= 1;
                } else {
                    right_y += 1;
                    tiles_to_carve -= 1;
                }
            },
            3 => {
                left_x -= 1;
                if left_x <= (brush + 1) {
                    left_x += 1;
                } else {
                    right_x += 1;
                    tiles_to_carve -= 1;
                }
            },
            _ => {
                left_x += 1;
                if left_x >= MAP_WIDTH / 2 {
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

// Creates a random pattern from the center of the map.
pub fn random_hole(map: &mut Map, colors: &[Color; 7]) {
    // Creates two instances of the center point, and amount of tiles to be carved.
    let mut x = MAP_WIDTH / 2;
    let mut y = MAP_HEIGHT / 2;
    let mut tiles_to_carve = 500;

    // This is how many tiles will be removed per "carve"
    let brush = 2;

    // First, it removes the center tile that it begins on.
    map[x as usize][y as usize] = Tile::empty(colors);

    while tiles_to_carve > 0 {

        // Decides a random direction to move
        // If new position would be outside the map boundary, it returns to its previous position.
        let four_sided_dice = rand::thread_rng().gen_range(1, 5);
        match four_sided_dice {
            1 => {
                y -= 1;
                if y <= (brush + 1) {
                    y = MAP_HEIGHT / 2;
                } else {
                    tiles_to_carve -= 1;
                }
            },
            2 => {
                y += 1;
                if y >= MAP_HEIGHT - (brush + 1) {
                    y = MAP_HEIGHT / 2;
                } else {
                    tiles_to_carve -= 1;
                }
            },
            3 => {
                x -= 1;
                if x <= (brush + 1) {
                    x = MAP_WIDTH / 2;
                } else {
                    tiles_to_carve -= 1;
                }
            },
            _ => {
                x += 1;
                if x >= MAP_WIDTH - (brush + 1) {
                    x = MAP_WIDTH / 2;
                } else {
                    tiles_to_carve -= 1;
                }
            }
        }

        // Removes the tiles according to brush size based on the new position.
        for x in (x - brush)..(x + brush) {
            for y in (y - brush)..(y + brush) {
                map[x as usize][y as usize] = Tile::empty(colors);
            }
        }
    }
}

pub fn rubble(rooms: &Vec<Rect>, map: &mut Map, colors: &[Color; 7]) {
    let final_room = &rooms[rooms.len() - 2];
    for room in rooms {
        let tiles = (room.x2 - room.x1) * (room.y2 - room.y1);
        let possible_debris = tiles / 5;
        let mut piles = 0;

        while possible_debris > piles {
            let x = rand::thread_rng().gen_range(room.x1 + 2, room.x2 - 1);
            let y = rand::thread_rng().gen_range(room.y1 + 2, room.y2 - 1);

            if rand::random() {
                map[x as usize][y as usize] = Tile::wall(colors);
            }

            piles += 1;
        }
        if room == final_room {
            break;
        }
    }
}

pub fn pillars(rooms: &Vec<Rect>, map: &mut Map, colors: &[Color; 7]) {
    let stair_room = rooms.len() - 1;
    let mut room_count = 0;
    for room in rooms {
        let tiles = ((room.x2-1) - (room.x1+1)) * ((room.y2-1) - (room.y1+1));
        let possible_pillars = tiles / 5;

        if possible_pillars < 16 && rand::random() {
            map[(room.x1 + 2) as usize][(room.y1 + 2) as usize] = Tile::wall(colors);
            map[(room.x2 - 2) as usize][(room.y1 + 2) as usize] = Tile::wall(colors);
            map[(room.x1 + 2) as usize][(room.y2 - 2) as usize] = Tile::wall(colors);
            map[(room.x2 - 2) as usize][(room.y2 - 2) as usize] = Tile::wall(colors);
        }

        room_count += 1;

        if room_count == stair_room {
            break;
        }
    }
}

pub fn purge_loners(map: &mut Map, colors: &[Color; 7], max_walls: u32) {
    let mut purge_map = vec![vec![false; MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    for x in 1..MAP_WIDTH - 1 {
        for y in 1..MAP_HEIGHT - 1 {
            let mut wall_count = 0;

            if map[(x - 1) as usize][y as usize].empty == false { wall_count += 1; }
            if map[(x + 1) as usize][y as usize].empty == false { wall_count += 1; }
            if map[x as usize][(y - 1) as usize].empty == false { wall_count += 1; }
            if map[x as usize][(y + 1) as usize].empty == false { wall_count += 1; }

            if wall_count == max_walls {
                purge_map[x as usize][y as usize] = true;
            }
        }
    }
    for x in 1..MAP_WIDTH - 1 {
        for y in 1..MAP_HEIGHT - 1 {
            if purge_map[x as usize][y as usize] == true {
                map[x as usize][y as usize] = Tile::empty(colors);
            }
        }
    }
}
