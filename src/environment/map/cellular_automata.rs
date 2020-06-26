use crate::graphics::render_map;
use crate::environment::*;
use crate::Tcod;

use rand::*;

pub fn cellular_automata(
    map: &mut Map,
    colors: &[Color; 7],
    player: &mut Object,
    tcod: &mut Tcod,
    should_render: bool,
) {

    // Gives each tile a 55% chance to become an empty tile, rather than a wall.
    for x in 1..(MAP_WIDTH - 1) {
        for y in 1..(MAP_HEIGHT - 1) {
            if rand::thread_rng().gen::<f32>() > 0.55 {
                map[x as usize][y as usize] = Tile::empty(colors);
            }
        }
    }

    // Establishes a 2d vector of the same size as the map.
    // If a tile in this vector is true, the tile at the same point on the game map will be "purged" and turned into a wall.
    let mut purge_map = vec![vec![false; MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // Defines how many iterations of the cellular automata rules should be applied
    let cell_cycles = 7;

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

    // Place the player on a random empty tile.
    loop {
        let player_x = rand::thread_rng().gen_range(1, MAP_WIDTH - 1);
        let player_y = rand::thread_rng().gen_range(1, MAP_HEIGHT - 1);
        if map[player_x as usize][player_y as usize].empty == true {
            // Places player in the center of the room.
            player.set_pos(player_x, player_y);
            break;
        }
    }
}
