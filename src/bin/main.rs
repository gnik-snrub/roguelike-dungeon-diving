extern crate roguelike;
use roguelike::*;

const LIMIT_FPS: i32 = 60; // 20 frames-per-second maximum

fn main() {

    let mut tcod = Tcod::new();
    tcod::system::set_fps(LIMIT_FPS);

    roguelike::game(&mut tcod);

}
