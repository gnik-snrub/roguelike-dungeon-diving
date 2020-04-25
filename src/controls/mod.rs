use crate::environment::Game;

use crate::Tcod;
use crate::objects::*;

//use tcod::console::*;

pub fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    // todo: handle handle keys

    let key = tcod.root.wait_for_keypress(true);
    match key {
        // Movement keys
        Key { code: NumPad8, .. } => player.move_by(0, -1, game),
        Key { code: NumPad5, .. } => player.move_by(0, 1, game),
        Key { code: NumPad4, .. } => player.move_by(-1, 0, game),
        Key { code: NumPad6, .. } => player.move_by(1, 0, game),
//      This code is temporarily removed, as it breaks the laptop on which it is being written.
//      Note: The fact that it breaks this specific laptop is proof that it functions correctly.
//        Key { code: Enter,
//              alt: true, .. } => {
//            // Alt+Enter: Toggles fullscreen
//            let fullscreen = tcod.root.is_fullscreen();
//            tcod.root.set_fullscreen(!fullscreen);
//        }
        Key { code: Escape, .. } => return true, // Exits game

        _ => {}
    }

    false
}
