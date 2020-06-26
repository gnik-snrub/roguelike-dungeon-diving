use crate::graphics::render_map;
use crate::Tcod;
use crate::environment::*;

use rand::*;

pub fn drunk_walk(
    start_points: &mut Vec<(i32, i32)>,
    map: &mut Map,
    colors: &[Color; 7],
    player: &mut Object,
    tcod: &mut Tcod,
    should_render: bool,
) {

    // This is how many tiles will be removed per "carve"
    let brush = rand::thread_rng().gen_range(0, 3);

    // Decides a random starting point.
    let mut x = rand::thread_rng().gen_range(1 + brush, MAP_WIDTH - 1 - brush);
    let mut y = rand::thread_rng().gen_range(1 + brush, MAP_HEIGHT - 1 - brush);

    // Sets the amount of tiles to carve, and establishes a variable to track how many have been carved so far.
    let mut tiles_carved = 0;
    let aimed_carve_total = (((MAP_WIDTH - 2) * (MAP_WIDTH - 2)) as f32 * 0.35) as u32;

    // Starting point is inserted into the point vector
    // Point vector is used later to ensure every area is accessible.
    start_points.push((x, y));

    // Removes the center tile that it begins on.
    map[x as usize][y as usize] = Tile::empty(colors);

    while tiles_carved < aimed_carve_total {
        // Decides a random direction to move
        // If new position would be outside the map boundary, it moves to a random position.
        // Also adds the the new random position to the point vector, and shows the map visualizer if required.
        let four_sided_dice = rand::thread_rng().gen_range(1, 5);
        match four_sided_dice {
            1 => {
                if (y - 1) > brush {
                    y -= 1;
                } else {
                    y = rand::thread_rng().gen_range(1 + brush, MAP_HEIGHT - 1 - brush);
                    start_points.push((x, y));

                    if should_render {
                        render_map(tcod, map, 5);
                    }
                }
            },
            2 => {
                if (y + 1) < (MAP_HEIGHT - brush) && (y + 1) < (MAP_HEIGHT - 1) {
                    y += 1;
                } else {
                    y = rand::thread_rng().gen_range(1 + brush, MAP_HEIGHT - 1 - brush);
                    start_points.push((x, y));

                    if should_render {
                        render_map(tcod, map, 5);
                    }
                }
            },
            3 => {
                if (x - 1) > brush {
                    x -= 1;
                } else {
                    x = rand::thread_rng().gen_range(1 + brush, MAP_WIDTH - 1 - brush);
                    start_points.push((x, y));

                    if should_render {
                        render_map(tcod, map, 5);
                    }
                }
            },
            _ => {
                if (x + 1) < (MAP_WIDTH - brush) && (x + 1) < (MAP_WIDTH - 1) {
                    x += 1;
                } else {
                    x = rand::thread_rng().gen_range(1 + brush, MAP_WIDTH - 1 - brush);
                    start_points.push((x, y));

                    if should_render {
                        render_map(tcod, map, 5);
                    }
                }
            }
        }

        //Checks that the tile at the position isn't empty
        if map[x as usize][y as usize].wall {
            // Removes the tiles according to brush size based on the new position.
            if brush > 0 {
                for brush_x in (x - brush)..(x + brush) {
                    for brush_y in (y - brush)..(y + brush) {
                        map[brush_x as usize][brush_y as usize] = Tile::empty(colors);
                        tiles_carved += 1;
                    }
                }
            } else {
                map[x as usize][y as usize] = Tile::empty(colors);
                tiles_carved += 2;
            }
        }
    }

    // Places the player in a random empty tile on the map.
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
