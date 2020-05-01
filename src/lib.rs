pub mod objects;
pub mod controls;
pub mod environment;
pub mod graphics;

use objects::Object;
use environment::*;
use controls::{ handle_keys, PlayerAction };
use graphics::render_all;

use std::collections::HashMap;

use tcod::console::*;
use tcod::colors::*;
use tcod::map::Map as FovMap;
use tcod::input::{ self, Event, Key, Mouse };

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const BAR_WIDTH: i32 = 20;
const PANEL_HEIGHT: i32 = 7;
const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

const MSG_X: i32 = BAR_WIDTH + 4;
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

const LIGHT_WALL_COLOR: usize = 0;
const LIGHT_GROUND_COLOR: usize = 3;
const V_ONE: usize = 1;
const V_TWO: usize = 2;
const DARKNESS_MODIFIER: usize = 6;

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

    // Creates game objects
    let mut characters = vec![];
    let mut items = HashMap::new();

    // Generate map to be rendered
    let mut game = Game::new(&mut characters, &mut items);

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

    while !tcod.root.window_closed() {
        tcod.con.clear();

        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => tcod.mouse = m,
            Some((_, Event::Key(k))) => tcod.key = k,
            _ => tcod.key = Default::default(),
        }

        // Renders the screen
        let fov_recompute = previous_player_position != (game.player.pos());
        render_all(&mut tcod, &mut game, &characters, &items, fov_recompute);

        tcod.root.flush();

        // Handles keys, and exits game if prompted
        previous_player_position = game.player.pos();
        let player_action = handle_keys(&mut tcod, &mut game, &mut characters, &mut items);
        if player_action == PlayerAction::Exit { break; }

        // Lets monsters take their turn
        if game.player.alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..characters.len() {
                if characters[id].ai.is_some() {
                    Object::ai_take_turn(id, &tcod, &mut game, &mut characters);
                }
            }
        }
    }
}
