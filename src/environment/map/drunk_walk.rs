use crate::environment::*;

use rand::*;

pub fn drunk_walk(
    map: &mut Map,
    colors: &[Color; 7],
    player: &mut Object
) {

    // This is how many tiles will be removed per "carve"
    let brush = rand::thread_rng().gen_range(0, 2);

    let mut x = rand::thread_rng().gen_range(1 + brush, MAP_WIDTH - 1 - brush);
    let mut y = rand::thread_rng().gen_range(1 + brush, MAP_HEIGHT - 1 - brush);
    let mut tiles_carved = 0;
    let aimed_carve_total = ((MAP_WIDTH - 2) * (MAP_WIDTH - 2)) / 2;

    // First, it removes the center tile that it begins on.
    map[x as usize][y as usize] = Tile::empty(colors);

    while tiles_carved < aimed_carve_total {
        // Decides a random direction to move
        // If new position would be outside the map boundary, it returns to its previous position.
        let four_sided_dice = rand::thread_rng().gen_range(1, 5);
        match four_sided_dice {
            1 => {
                if (y - 1) > brush {
                    y -= 1;
                } else {
                    y = rand::thread_rng().gen_range(1 + brush, MAP_HEIGHT - 1 - brush);
                }
            },
            2 => {
                if (y + 1) < MAP_HEIGHT - brush {
                    y += 1;
                } else {
                    y = rand::thread_rng().gen_range(1 + brush, MAP_HEIGHT - 1 - brush);
                }
            },
            3 => {
                if (x - 1) > brush {
                    x -= 1;
                } else {
                    x = rand::thread_rng().gen_range(1 + brush, MAP_WIDTH - 1 - brush);
                }
            },
            _ => {
                if (x + 1) < MAP_WIDTH - brush {
                    x += 1;
                } else {
                    x = rand::thread_rng().gen_range(1 + brush, MAP_WIDTH - 1 - brush);
                }
            }
        }

        if map[x as usize][y as usize].empty == false {
            // Removes the tiles according to brush size based on the new position.
            if brush > 0 {
                for brush_x in (x - brush)..(x + brush) {
                    for brush_y in (y - brush)..(y + brush) {
                        map[brush_x as usize][brush_y as usize] = Tile::empty(colors);
                        tiles_carved += 1;
                    }
                }
            } else {
                if !(x >= MAP_WIDTH - 1 || y >= MAP_HEIGHT - 1) {
                    map[x as usize][y as usize] = Tile::empty(colors);
                    tiles_carved += 2;
                }
            }
        }
    }

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
