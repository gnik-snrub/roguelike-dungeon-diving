pub mod objects;
pub mod controls;
pub mod environment;
pub mod graphics;

use objects::{ Object, Character };
use environment::*;
use controls::{ handle_keys, PlayerAction };
use graphics::render_all;
use graphics::gui::menu::{ menu, msgbox };

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{ Read, Write };

use tcod::console::*;
use tcod::colors::*;
use tcod::map::Map as FovMap;
use tcod::input::{ self, Event, Key, Mouse };

const LIMIT_FPS: i32 = 60; // 20 frames-per-second maximum

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

const LEVEL_UP_BASE: i32 = 200;
const LEVEL_UP_FACTOR: i32 = 150;
const LEVEL_SCREEN_WIDTH: i32 = 40;

const CHARACTER_SCREEN_WIDTH: i32 = 30;

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

pub fn run_game() {
    let mut tcod = Tcod::new();
    tcod::system::set_fps(LIMIT_FPS);

    main_menu(&mut tcod);
}

fn main_menu(mut tcod: &mut Tcod) {
    let img = tcod::image::Image::from_file("menu_background.png")
    .ok()
    .expect("Background image not found");

    while !tcod.root.window_closed() {
        // Show the background image, at twice the regular console resolution.
        tcod::image::blit_2x(&img, (0, 0), (-1, -1), &mut tcod.root, (0, 0));

        tcod.root.set_default_foreground(LIGHT_YELLOW);
        tcod.root.print_ex(
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT / 2 - 4,
            BackgroundFlag::None,
            TextAlignment::Center,
            "TOMBS OF THE ANCIENT KINGS",
        );
        tcod.root.print_ex(
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT - 2,
            BackgroundFlag::None,
            TextAlignment::Center,
            "BOTTOM TEXT",
        );

        // Show options, and wait for the player's choice.
        let choices = &["Play a new game", "Continue last game", "Quit"];
        let choice = menu("", choices, 24, &mut tcod.root);

        match choice {
            Some(0) => {
                // New game
                let (mut game, mut characters, mut items, mut player) = new_game(&mut tcod);
                play_game(&mut tcod, &mut game, &mut characters, &mut items, &mut player);
            },
            Some(1) => {
                // Loads game
                match load_game() {
                    Ok((mut game, mut characters, mut items, mut player)) => {
                        initialise_fov(tcod, &game.map);
                        play_game(&mut tcod, &mut game, &mut characters, &mut items, &mut player);
                    },
                    Err(_e) => {
                        msgbox("\nNo saved game to load.\n", 24, &mut tcod.root);
                        continue;
                    }
                }

            }
            Some(2) => {
                // Quit
                break;
            },
            _ => {},
        }
    }
}

fn new_game(tcod: &mut Tcod) -> (Game, Vec<Character>, HashMap<i32, Object>, Character) {
    // Creates game objects
    let mut characters: Vec<Character> = vec![];
    let mut items = HashMap::new();
    let mut player = Object::new_player();

    // Generate map to be rendered
    let mut game = Game::new(&mut characters, &mut items, &mut player.object);

    initialise_fov(tcod, &game.map);

    // Intro message
    game.messages.add(
        "Dive deep. Gain power. Try not to die in these ancient tombs...",
        GOLD,
    );

    (game, characters, items, player)
}

fn initialise_fov(tcod: &mut Tcod, map: &Map) {
    // Populates the FOV map, based on the generated map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            tcod.fov.set(
                x, y,
                !map[x as usize][y as usize].block_sight,
                !map[x as usize][y as usize].blocked,
            );
        }
    }

    // Unexplored areas start black (Default background color)
    tcod.con.clear();
}

fn save_game(
    game: &Game,
    characters: &mut Vec<Character>,
    items: &mut HashMap<i32, Object>,
    player: &mut Character,
) -> Result<(), Box<dyn Error>> {
    let save_data = serde_json::to_string(&(game, characters, items, player))?;
    let mut file = File::create("savegame")?;
    file.write_all(save_data.as_bytes())?;
    Ok(())
}

fn load_game() -> Result<(Game, Vec<Character>, HashMap<i32, Object>, Character), Box<dyn Error>> {
    let mut json_save_state = String::new();
    let mut file = File::open("savegame")?;
    file.read_to_string(&mut json_save_state)?;
    let result = serde_json::from_str::<(Game, Vec<Character>, HashMap<i32, Object>, Character)>(&json_save_state)?;
    Ok(result)
}

fn play_game(
    mut tcod: &mut Tcod,
    mut game: &mut Game,
    mut characters: &mut Vec<Character>,
    mut items: &mut HashMap<i32, Object>,
    mut player: &mut Character,
) {
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
        let fov_recompute = previous_player_position != (player.object.pos());
        render_all(&mut tcod, &mut game, &characters[..], &items, fov_recompute, &mut player.object);

        tcod.root.flush();

        // Level up if needed.
        Object::level_up(tcod, game, &mut player.object);

        // Handles keys, and exits game if prompted
        previous_player_position = player.object.pos();
        let player_action = handle_keys(&mut tcod, &mut game, &mut characters, &mut items, &mut player);
        if player_action == PlayerAction::Exit {
            save_game(game, characters, items, player).unwrap();
            break;
        }

        // Lets monsters take their turn
        if player.object.alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..characters.len() {
                if characters[id].object.ai.is_some() {
                    Object::ai_take_turn(id, &tcod, &mut game, &mut characters[..], &mut player.object);
                }
            }
        }
    }
}
