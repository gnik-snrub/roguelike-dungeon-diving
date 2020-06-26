pub mod gui;

use crate::*;
use crate::objects::*;
use crate::environment::*;
use gui::render_gui;

use rand::*;

pub fn render_all(
    tcod: &mut Tcod,
    game: &mut Game,
    characters: &[Character],
    items: &HashMap<i32, Object>,
    fov_recompute: bool,
    player: &Object,
) {
    if fov_recompute {
        //Recomputes FOV is needed, such as player movement
        tcod.fov.compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    // Scans the map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {

            // Checks to see if each tile is in the player's FOV.
            let visible = tcod.fov.is_in_fov(x, y);

            // Color of the tile at present moment is determined.
            let color = match visible {
                // Outside of field of view:
                false => game.map[x as usize][y as usize].color_dark,

                // Inside the field of view:
                true => game.map[x as usize][y as usize].color_light,
            };

            // If tile is currently in FOV, set the tiles "explored" variable to true.
            let explored = &mut game.map[x as usize][y as usize].explored;
            if visible {
                //Since it's visible, explore it
                *explored = true;
            }

            // If a tiles "explored" variable is true, it will become visible.
//            if *explored {
                tcod.con.set_char_background(x, y, color, BackgroundFlag::Set);
//            }
        }
    }

    // Calls functions to render objects, and the GUI.
    draw_objects(tcod, game, items, characters, player);
    render_gui(tcod, game, characters, items, player);
}

fn draw_objects(tcod: &mut Tcod, game: &mut Game, items: &HashMap<i32, Object>, characters: &[Character], player: &Object) {

    // Draws items, then draws characters.
    // Characters are second, so that they have visibility priority over items.
    draw_items(tcod, game, items);
    draw_chars(tcod, game, characters);

    // Finally, it renders the player.
    player.draw(&mut tcod.con);

    // Blit the contents (items + characters) of "con" to the root console and present it
    blit(
        &tcod.con,
        (0, 0),
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn draw_items(tcod: &mut Tcod, game: &mut Game, items: &HashMap<i32, Object>) {
    // Searches through items hashmap.
    for item in items.values() {
        // If item is in FOV, or "always_visible" variable is true in the location of an explored tile.
        // Draw the item.
        if tcod.fov.is_in_fov(item.x, item.y) ||
            item.always_visible && game.map[item.x as usize][item.y as usize].explored {
            item.draw(&mut tcod.con);
        }
    }
}

fn draw_chars(tcod: &mut Tcod, game: &mut Game, characters: &[Character]) {
    // Sorts character list to place non-blocking (corpses) first.
    // This allows living characters to appear on top of them.
    let mut to_draw: Vec<_> = characters
        .iter()
        .filter(|c| {
            tcod.fov.is_in_fov(c.object.x, c.object.y) ||
            (c.object.always_visible && game.map[c.object.x as usize][c.object.y as usize].explored)
        })
        .collect();
    to_draw.sort_by(|c1, c2| c1.object.blocks.cmp(&c2.object.blocks));

    // Draw all characters in the list
    for character in &to_draw {
        character.object.draw(&mut tcod.con);
    }
}

pub fn gen_colors() -> [Color; 7] {
    // Light wall color is established.
    let light_wall_color: Color = Color {
        r: ((rand::thread_rng().gen_range(80, 130))),
        g: ((rand::thread_rng().gen_range(80, 130))),
        b: ((rand::thread_rng().gen_range(80, 130)))
    };

    // Light ground color is established.
    let light_ground_color: Color = Color {
        r: ((rand::thread_rng().gen_range(65, 175))),
        g: ((rand::thread_rng().gen_range(65, 175))),
        b: ((rand::thread_rng().gen_range(65, 175)))
    };

    // Variant is established, which just functions as a modifier to provide variation.
    let variant: Color = Color  {
        r: ((rand::thread_rng().gen_range(0, 30))),
        g: ((rand::thread_rng().gen_range(0, 30))),
        b: ((rand::thread_rng().gen_range(0, 30)))
    };

    // Creates higher, and lower variants based on the above colors.
    let light_wall_variant_one: Color = light_wall_color - variant;
    let light_wall_variant_two: Color = light_wall_color + variant;
    let light_ground_variant_one: Color = light_ground_color - variant;
    let light_ground_variant_two: Color = light_ground_color + variant;

    // A darkness modifier is created, which gets subtracted from the base tile color whenever it is in darkness.
    let dark_modifier: Color = Color {
        r: ((rand::thread_rng().gen_range(25, 35))),
        g: ((rand::thread_rng().gen_range(25, 35))),
        b: ((rand::thread_rng().gen_range(5, 15))),
    };

    // Returns an array consisting of all of the color variants + darkness modifier.
    let colors = [
        light_wall_color,
        light_wall_variant_one,
        light_wall_variant_two,
        light_ground_color,
        light_ground_variant_one,
        light_ground_variant_two,
        dark_modifier,
        ];

    colors
}

pub fn render_map(
    tcod: &mut Tcod,
    map: &mut Map,
    frames: u32,
) {
    // Functions the same as the regular map rendering, although it has some differences.
    // Always shows all tiles.
    // Also shows the map for a short period of time, determined by the "frames" variable.
    // It should only be used during map gen, to visualize what the algorithm is doing.
    tcod.fov.compute_fov(1, 1, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);

    for _ in 1..frames {

        tcod.root.clear();

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {

                let visible = tcod.fov.is_in_fov(x, y);
                let wall = map[x as usize][y as usize].block_sight;
                let color = match(visible, wall) {

                    // Outside of field of view:
                    (false, true) => map[x as usize][y as usize].color_dark,
                    (false, false) => map[x as usize][y as usize].color_dark,

                    // Inside the field of view:
                    (true, true) => map[x as usize][y as usize].color_light,
                    (true, false) => map[x as usize][y as usize].color_light,
                };

                tcod.root.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }

        tcod.root.flush();
    }
}
