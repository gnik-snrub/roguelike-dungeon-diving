pub mod objects;
pub mod controls;
pub mod environment;

use objects::Object;
use environment::*;
use controls::{ handle_keys, PlayerAction };

use rand::*;

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

fn gen_colors() -> [Color; 4] {
    let light_wall_color: Color = Color {
        r: ((rand::thread_rng().gen_range(50, 100))), // Range = 50/100
        g: ((rand::thread_rng().gen_range(50, 100))),
        b: ((rand::thread_rng().gen_range(50, 100)))
    };
    let light_ground_color: Color = Color {
        r: ((rand::thread_rng().gen_range(50, 150))), // Range = 100/200
        g: ((rand::thread_rng().gen_range(75, 175))),
        b: ((rand::thread_rng().gen_range(25, 175)))
    };
    let dark_ground_color: Color = light_ground_color - Color {
        r: ((rand::thread_rng().gen_range(75, 100))), // Range = 0/125
        g: ((rand::thread_rng().gen_range(75, 100))),
        b: ((rand::thread_rng().gen_range(75, 100)))
    };
    let dark_wall_color: Color = light_wall_color - Color {
        r: ((rand::thread_rng().gen_range(35, 50))), // Range = 0/50
        g: ((rand::thread_rng().gen_range(35, 50))),
        b: ((rand::thread_rng().gen_range(35, 50)))
    };

    let colors = [dark_wall_color, light_wall_color, dark_ground_color, light_ground_color];
    colors
}

fn render_all(tcod: &mut Tcod, game: &mut Game, objects: &[Object], fov_recompute: bool, colors: &[Color; 4]) {
    if fov_recompute {
        //Recomputes FOV is needed, such as player movement
        let player = &objects[PLAYER];
        tcod.fov
            .compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {

            let visible = tcod.fov.is_in_fov(x, y);
            let wall = game.map[x as usize][y as usize].block_sight;
            let color = match(visible, wall) {

                // Outside of field of view:
                (false, true) => colors[DARK_WALL_COLOR],
                (false, false) => colors[DARK_GROUND_COLOR],

                // Inside the field of view:
                (true, true) => colors[LIGHT_WALL_COLOR],
                (true, false) => colors[LIGHT_GROUND_COLOR],
            };

            let explored = &mut game.map[x as usize][y as usize].explored;
            if visible {
                //Since it's visible, explore it
                *explored = true;
            }

            if *explored {
                // Show explored tiles only! (Any visible tile is explored already)
                tcod.con.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }

    // Sorts items to place non-blocking items first.
    // This allows blocking items to appear on top of them.
    let mut to_draw: Vec<_> = objects
        .iter()
        .filter(|o| tcod.fov.is_in_fov(o.x, o.y))
        .collect();
    to_draw.sort_by(|o1, o2| o1.blocks.cmp(&o2.blocks));

    // Draw all objects in the list
    for object in &to_draw {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(&mut tcod.con);
        }
    }

    // Blit the contents of "con" to the root console and present it
    blit(
        &tcod.con,
        (0, 0),
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );

    // Prepares the GUI panel.
    tcod.panel.set_default_background(BLACK);
    tcod.panel.clear();

    // Print the game messages, line by line.
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in game.messages.iter().rev() {
        let msg_height = tcod.panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        tcod.panel.set_default_foreground(color);
        tcod.panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }

    // Show the player's stats.
    let hp = objects[PLAYER].fighter.unwrap().hp;
    let max_hp = objects[PLAYER].fighter.unwrap().max_hp;
    render_bar(
        &mut tcod.panel,
        1,
        0,
        BAR_WIDTH,
        "HP",
        hp,
        max_hp,
        LIGHT_RED,
        DARKER_RED,
    );

    tcod.panel.set_default_foreground(LIGHT_GREY);
    tcod.panel.print_ex(
        1,
        1,
        BackgroundFlag::None,
        TextAlignment::Left,
        get_names_under_mouse(tcod.mouse, objects, &tcod.fov),
    );

    // Blit the contents of 'panel' to the root console.
    blit(
        &tcod.panel,
        (0, 0),
        (SCREEN_WIDTH, PANEL_HEIGHT),
        &mut tcod.root,
        (0, PANEL_Y),
        1.0,
        1.0,
    );
}

// Renders a bar of some sort.
fn render_bar(
    panel: &mut Offscreen,
    x: i32,
    y: i32,
    total_width: i32,
    name: &str,
    value: i32,
    maximum: i32,
    bar_color: Color,
    back_color: Color,)
    {
        // Calculates the width of the bar.
        let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

        // Renders background of bar.
        panel.set_default_background(back_color);
        panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

        // Renders the bar over the top of it.
        panel.set_default_background(bar_color);
        if bar_width > 0 {
            panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
        }

        //Centered text with relevant values.
        panel.set_default_foreground(WHITE);
        panel.print_ex(
            x + total_width / 2,
            y,
            BackgroundFlag::None,
            TextAlignment::Center,
            &format!("{}: {}/{}", name, value, maximum),
        );
}

pub struct Messages {
    messages: Vec<(String, Color)>,
}

impl Messages {
    pub fn new() -> Messages {
        Messages { messages: vec![] }
    }

    // Add the new message as a tuple containing the text and the color.
    pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
        self.messages.push((message.into(), color));
    }

    // Creates a 'DoubleEndedIterator' over the messages.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, Color)> {
        self.messages.iter()
    }
}

fn get_names_under_mouse(mouse: Mouse, objects: &[Object], fov_map: &FovMap) -> String {
    let (x, y) = (mouse.cx as i32, mouse.cy as i32);

    // Creates a list with the names of all objects at mouse's coordinates in FOV.
    let names = objects
        .iter()
        .filter(|obj| obj.pos() == (x, y) && fov_map.is_in_fov(obj.x, obj.y))
        .map(|obj| obj.name.clone())
        .collect::<Vec<_>>();

    names.join("\n") // Separates names new lines.
}
