use crate::graphics::render_map;
use crate::environment::*;
use crate::Tcod;

use rand::*;
use tcod::console::*;

pub fn cellular_automata(
    start_points: &mut Vec<(i32, i32)>,
    map: &mut Map,
    colors: &[Color; 7],
    player: &mut Object,
    tcod: &mut Tcod,
    should_render: bool,
) {

    for x in 1..(MAP_WIDTH - 1) {
        for y in 1..(MAP_HEIGHT - 1) {
            if rand::thread_rng().gen::<f32>() > 0.55 {
                map[x as usize][y as usize] = Tile::empty(colors);
            }
        }
    }
//7
    for _ in 1..12 {

        let mut purge_map = vec![vec![false; MAP_HEIGHT as usize]; MAP_WIDTH as usize];

        for x in 1..MAP_WIDTH - 1 {
            for y in 1..MAP_HEIGHT - 1 {
                let mut wall_count = 0;

                if map[(x - 1) as usize][y as usize].wall { wall_count += 1; }
                if map[x as usize][(y - 1) as usize].wall { wall_count += 1; }
                if map[(x - 1) as usize][(y - 1) as usize].wall { wall_count += 1; }
                if map[(x + 1) as usize][(y - 1) as usize].wall { wall_count += 1; }
                if map[(x + 1) as usize][y as usize].wall { wall_count += 1; }
                if map[x as usize][(y + 1) as usize].wall { wall_count += 1; }
                if map[(x - 1) as usize][(y + 1)as usize].wall { wall_count += 1; }
                if map[(x + 1) as usize][(y + 1) as usize].wall { wall_count += 1; }

                if wall_count == 0
                || wall_count > 4 {
                    purge_map[x as usize][y as usize] = true;
                }
            }
        }

        for x in 1..MAP_WIDTH - 1 {
            for y in 1..MAP_HEIGHT - 1 {
                match purge_map[x as usize][y as usize] {
                    true => { map[x as usize][y as usize] = Tile::wall(colors) },
                    false => { map[x as usize][y as usize] = Tile::empty(colors) },
                }
            }
        }

        if should_render {
            render_map(tcod, map, 8);
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
