pub mod objects;
pub mod controls;
pub mod environment;
pub mod graphics;

use objects::Object;
use environment::*;
use controls::{ handle_keys, PlayerAction };
use graphics::{ render_all, gen_colors };

use tcod::console::*;
use tcod::colors::*;
use tcod::map::Map as FovMap;
use tcod::input::{ self, Event, Key, Mouse };

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const PLAYER: usize = 0;

const BAR_WIDTH: i32 = 20;
const PANEL_HEIGHT: i32 = 7;
const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

const MSG_X: i32 = BAR_WIDTH + 2;
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

const DARK_WALL_COLOR: usize = 0;
const LIGHT_WALL_COLOR: usize = 1;
const DARK_GROUND_COLOR: usize = 2;
const LIGHT_GROUND_COLOR: usize = 3;

pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
    pub panel: Offscreen,
    pub fov: FovMap,
    pub key: Key,
    pub mouse: Mouse,
}

impl Tcod {
    pub fn new() -> Tcod {
        let root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust/libtcod tutorial")
            .init();

        let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
        let panel = Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT);
        let fov = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
        let key = Default::default();
        let mouse = Default::default();

        Tcod { root, con, panel, fov, key, mouse }
    }
}

pub fn game(mut tcod: &mut Tcod) {

    // Creates object representing player
    let player = Object::player();
    let mut objects = vec![player];

    // Generate map to be rendered
    let mut game = Game::new(&mut objects);

    // Populates the FOV map, based on the generated map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x, y,
                !game.map[x as usize][y as usize].block_sight,
                !game.map[x as usize][y as usize].blocked,
            );
        }
    }

    // Intro message
    game.messages.add(
        "Dive deep. Gain power. Try not to die in these ancient tombs...",
        GOLD,
    );

    // Force FOV "recompute" first time through the game loop
    let mut previous_player_position = (-1, -1);

    let colors = gen_colors();

    while !tcod.root.window_closed() {
        tcod.con.clear();

        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => tcod.mouse = m,
            Some((_, Event::Key(k))) => tcod.key = k,
            _ => tcod.key = Default::default(),
        }

        // Renders the screen
        let fov_recompute = previous_player_position != (objects[PLAYER].pos());
        render_all(&mut tcod, &mut game, &objects, fov_recompute, &colors);

        // FOV-Disabled render for debug purposes
        //debug_render_all(&mut tcod, &game, &objects);

        tcod.root.flush();

        // Handles keys, and exits game if prompted
        previous_player_position = objects[PLAYER].pos();
        let player_action = handle_keys(&mut tcod, &mut game, &mut objects);
        if player_action == PlayerAction::Exit { break; }

        // Lets monsters take their turn
        if objects[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    Object::ai_take_turn(id, &tcod, &mut game, &mut objects);
                }
            }
        }
    }
}
