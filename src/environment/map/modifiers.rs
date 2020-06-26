use crate::graphics::render_map;
use crate::Tcod;
use crate::environment::{ Map, MAP_WIDTH, MAP_HEIGHT };
use crate::environment::tiles::Tile;
use crate::environment::map::Rect;

use tcod::colors::*;
use rand::*;

// Creates some randomness along the outside of a rect.
pub fn mine_drunkenly(rooms: &Vec<Rect>, map: &mut Map, colors: &[Color; 7], tcod: &mut Tcod, should_render: bool) {
    for room in rooms {
        // Creates a random amount of miners.
        let miner_max = rand::thread_rng().gen_range(1, 5);

        // The amount of tiles carved out also varies.
        let tiles_to_carve = rand::thread_rng().gen_range(20, 40);

        let tiles_per_miner = tiles_to_carve / miner_max;

        // Gives each miner their turn to carve.
        for _ in 1..miner_max {
            // Finds the center of the room.
            let (mut x, mut y) = room.center();
            let mut tiles_carved = 0;

            // Divides the tiles to carve amongst the miners doing the work.
            while tiles_carved < tiles_per_miner {

                // If the miner is on a wall, it is made empty, and the tiles carved will increment.
                if !map[x as usize][y as usize].empty {
                    map[x as usize][y as usize] = Tile::empty(colors);
                    tiles_carved += 1
                } else { // Otherwise, it will move to a space within the map boundary.
                    let four_sided_dice = rand::thread_rng().gen_range(1, 5);
                    match four_sided_dice {
                        1 => { y += 1; // Moves down
                            if y >= MAP_HEIGHT - 1 { y -= 1; } // If too close to edge, it moves up
                        },
                        2 => { y -= 1; // Moves up
                            if y <= 1 { y += 1; } // If too close to edge, it moves down
                        },
                        3 => { x += 1; // Moves right
                            if x >= MAP_WIDTH - 1 { x -= 1; } // If too close to edge, it moves left
                        },
                        _ => { x -= 1; // Moves left
                            if x <= 1 { x += 1; } // If too close to edge, it moves right.
                        },
                    }
                }
            }

            // Map is rendered after each miner finishes their work.
            if should_render {
                render_map(tcod, map, 4);
            }
        }
    }
}

// Below are various forms of similar modifiers
pub fn caved_in(map: &mut Map, colors: &[Color; 7], tcod: &mut Tcod, should_render: bool) {
    // Randomly decides what type of cave-in occurs.
    if rand::random() {
        butterfly(map, &colors, tcod, should_render);
    } else {
        random_hole(map, &colors, tcod, should_render);
    }
}

// Creates a random mirrored pattern from the center of the map.
pub fn butterfly(map: &mut Map, colors: &[Color; 7], tcod: &mut Tcod, should_render: bool) {
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

        if should_render && tiles_to_carve % 100 == 0 {
            render_map(tcod, map, 5);
        }
    }
}

// Creates a random pattern from the center of the map.
pub fn random_hole(map: &mut Map, colors: &[Color; 7], tcod: &mut Tcod, should_render: bool) {
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

        if should_render && tiles_to_carve % 100 == 0 {
            render_map(tcod, map, 5);
        }
    }
}

// Scatters some random wall tiles into rooms, without impeding entrance/stairs
pub fn rubble(rooms: &Vec<Rect>, map: &mut Map, colors: &[Color; 7], tcod: &mut Tcod, should_render: bool) {
    // Designates the room prior to the stairs.
    // This allows you to stop before the stair room, so the tile is always accessible.
    let final_room = &rooms[rooms.len() - 2];

    // Iterate through the rooms.
    for room in rooms {
        // Calculates room size, and appropriate amount of debris.
        let tiles = (room.x2 - room.x1) * (room.y2 - room.y1);
        let possible_debris = tiles / 5;

        for _ in 1..possible_debris {
            // Finds random tile in range of respective room.
            let x = rand::thread_rng().gen_range(room.x1 + 2, room.x2 - 1);
            let y = rand::thread_rng().gen_range(room.y1 + 2, room.y2 - 1);

            // Flips a coin. If heads, debris is placed. Otherwise, nothing happens.
            if rand::random() {
                map[x as usize][y as usize] = Tile::wall(colors);
            }
        }

        // Toggles on/off depending on map-generation visualization option in make_map().
        if should_render {
            render_map(tcod, map, 4);
        }

        // If the current room is the room before the stair room, break the loop.
        if room == final_room {
            break;
        }
    }
}

// Places wall tiles as pillars in the four corners of each room, without impeding the entrances/stairs.
pub fn pillars(rooms: &Vec<Rect>, map: &mut Map, colors: &[Color; 7], tcod: &mut Tcod, should_render: bool) {
    // Designates the room prior to the stairs.
    // This allows you to stop before the stair room, so the tile is always accessible.
    let final_room = &rooms[rooms.len() - 2];

    // Iterate through the rooms.
    for room in rooms {
        // Checks to see if the room is large enough to have pillars, and still allow for movement.
        let tiles = ((room.x2-1) - (room.x1+1)) * ((room.y2-1) - (room.y1+1));
        let possible_pillars = tiles / 5;

        // If so, place pillars in the four corners of the room.
        if possible_pillars < 16 && rand::random() {
            map[(room.x1 + 2) as usize][(room.y1 + 2) as usize] = Tile::wall(colors);
            map[(room.x2 - 2) as usize][(room.y1 + 2) as usize] = Tile::wall(colors);
            map[(room.x1 + 2) as usize][(room.y2 - 2) as usize] = Tile::wall(colors);
            map[(room.x2 - 2) as usize][(room.y2 - 2) as usize] = Tile::wall(colors);
        }

        // Toggles on/off depending on map-generation visualization option in make_map().
        if should_render {
            render_map(tcod, map, 4);
        }

        // If the current room is the room before the stair room, break the loop.
        if room == final_room {
            break;
        }
    }
}

pub fn mini_automata(
    map: &mut Map,
    colors: &[Color; 7],
    tcod: &mut Tcod,
    should_render: bool,
) {
    // Establishes a 2d vector of the same size as the map.
    // If a tile in this vector is true, the tile at the same point on the game map will be "purged" and turned into a wall.
    let mut purge_map = vec![vec![false; MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // Defines how many iterations of the cellular automata rules should be applied
    let cell_cycles = 5;

    // Iterates through the cellular automata cycles.
    for _ in 1..cell_cycles {

        // Iterates through all the tiles on the map.
        for x in 1..MAP_WIDTH - 1 {
            for y in 1..MAP_HEIGHT - 1 {
                // Variable keeps track of how many walls are attached to a tile.
                let mut wall_count = 0;

                // Checks attached tiles. If they are a tile, increments the wall_count variable.
                if map[(x - 1) as usize][y as usize].wall { wall_count += 1; }
                if map[x as usize][(y - 1) as usize].wall { wall_count += 1; }
                if map[(x - 1) as usize][(y - 1) as usize].wall { wall_count += 1; }
                if map[(x + 1) as usize][(y - 1) as usize].wall { wall_count += 1; }
                if map[(x + 1) as usize][y as usize].wall { wall_count += 1; }
                if map[x as usize][(y + 1) as usize].wall { wall_count += 1; }
                if map[(x - 1) as usize][(y + 1)as usize].wall { wall_count += 1; }
                if map[(x + 1) as usize][(y + 1) as usize].wall { wall_count += 1; }

                // If the walls count is above 4, or 0, set that point on purge map to true.
                // In other words, If a tile has above 4, or 0 attached walls, it will become a wall itself.
                if wall_count == 0
                || wall_count > 4 {
                    purge_map[x as usize][y as usize] = true;
                }
            }
        }

        // Iterates once more through the map.
        for x in 1..MAP_WIDTH - 1 {
            for y in 1..MAP_HEIGHT - 1 {
                // Check the value of the purge map at each point
                match purge_map[x as usize][y as usize] {
                    // If purge map is true, set that tile to a wall, and reset that point of the purge map.
                    true => {
                        map[x as usize][y as usize] = Tile::wall(colors);
                        purge_map[x as usize][y as usize] = false;
                    },
                    // Otherwise, set that tile to an empty tile.
                    false => { map[x as usize][y as usize] = Tile::empty(colors); },
                }
            }
        }

        // Displays the map at each iteration of the map.
        if should_render {
            render_map(tcod, map, 10);
        }
    }
}
