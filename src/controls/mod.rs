use crate::*;
use crate::environment::Game;

use crate::Tcod;
use crate::objects::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

pub fn handle_keys(tcod: &mut Tcod, game: &mut Game, objects: &mut [Object]) -> PlayerAction {
    use tcod::input::KeyCode::*;

    use PlayerAction::*;

    let player_alive = objects[PLAYER].alive;
    match (tcod.key, tcod.key.text(), player_alive) {
        // Movement keys
        ( Key { code: NumPad7, .. }, _, true) => {
            Object::player_move_or_attack(-1, -1, game, objects);
            TookTurn
        },
        ( Key { code: NumPad8, .. }, _, true) => {
            Object::player_move_or_attack(0, -1, game, objects);
            TookTurn
        },
        ( Key { code: NumPad9, .. }, _, true) => {
            Object::player_move_or_attack(1, -1, game, objects);
            TookTurn
        },
        ( Key { code: NumPad4, .. }, _, true) => {
            Object::player_move_or_attack(-1, 0, game, objects);
            TookTurn
        },
        ( Key { code: NumPad5, .. }, _, true) => {
            TookTurn // Wait a turn
        },
        ( Key { code: NumPad6, .. }, _, true) => {
            Object::player_move_or_attack(1, 0, game, objects);
            TookTurn
        },
        ( Key { code: NumPad1, .. }, _, true) => {
            Object::player_move_or_attack(-1, 1, game, objects);
            TookTurn
        },
        ( Key { code: NumPad2, .. }, _, true) => {
            Object::player_move_or_attack(0, 1, game, objects);
            TookTurn
        },
        ( Key { code: NumPad3, .. }, _, true) => {
            Object::player_move_or_attack(1, 1, game, objects);
            TookTurn
        },
//      This code is temporarily removed, as it breaks the laptop on which it is being written.
//      Note: The fact that it breaks this specific laptop is proof that it functions correctly.
//        ( Key { code: Enter, alt: true, .. }, _, _, ) => {
            // Alt+Enter: Toggles fullscreen
//            let fullscreen = tcod.root.is_fullscreen();
//            tcod.root.set_fullscreen(!fullscreen);
//            DidntTakeTurn
//        },
        ( Key { code: Escape, .. }, _, _) => Exit, // Exits game

        _ => DidntTakeTurn,
    }
}
