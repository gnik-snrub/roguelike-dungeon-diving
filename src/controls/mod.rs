use crate::*;
use crate::environment::Game;
use crate::Tcod;
use crate::objects::*;
use crate::graphics::gui::menu::inventory_menu;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

pub fn handle_keys(tcod: &mut Tcod, game: &mut Game, characters: &mut Vec<Object>, items: &mut HashMap<i32, Object>) -> PlayerAction {
    use tcod::input::KeyCode::*;
    use PlayerAction::*;

    let player_alive = game.player.alive;
    match (tcod.key, tcod.key.text(), player_alive) {
        // Movement keys
        ( Key { code: NumPad7, .. }, _, true) => {
            Object::player_move_or_attack(-1, -1, game, characters);
            TookTurn
        },
        ( Key { code: NumPad8, .. }, _, true) => {
            Object::player_move_or_attack(0, -1, game, characters);
            TookTurn
        },
        ( Key { code: NumPad9, .. }, _, true) => {
            Object::player_move_or_attack(1, -1, game, characters);
            TookTurn
        },
        ( Key { code: NumPad4, .. }, _, true) => {
            Object::player_move_or_attack(-1, 0, game, characters);
            TookTurn
        },
        ( Key { code: NumPad6, .. }, _, true) => {
            Object::player_move_or_attack(1, 0, game, characters);
            TookTurn
        },
        ( Key { code: NumPad1, .. }, _, true) => {
            Object::player_move_or_attack(-1, 1, game, characters);
            TookTurn
        },
        ( Key { code: NumPad2, .. }, _, true) => {
            Object::player_move_or_attack(0, 1, game, characters);
            TookTurn
        },
        ( Key { code: NumPad3, .. }, _, true) => {
            Object::player_move_or_attack(1, 1, game, characters);
            TookTurn
        },

        // Wait a turn
        ( Key { code: NumPad5, .. }, _, true) => {
            TookTurn // Wait a turn
        },

        // Action keys
        // Grab the item at your position.
        ( Key { code: Text, .. }, "g", true) => {
            // Pick up an item
            let mut item_id: i32 = -1;

            for (key, item) in items.iter() {
                if (item.pos() == game.player.pos()) && (item.item.is_some()) {
                    item_id = *key
                }
            }

            if item_id > 0 {
                println!("{:?}", item_id);
                Object::pick_item_up(item_id, game, characters, items);
                TookTurn
            } else {
                game.messages.add(
                    format!("There's no item to grab..."),
                    RED
                );
                DidntTakeTurn
            }
        }

        ( Key { code: Text, .. }, "i", true) => {
            // Show the inventory.
            let inventory_index = inventory_menu(
                game,
                "Press the key next to an item to use it, or any other to cancel.\n",
                &mut tcod.root
            );
//            if let Some(inventory_index) = inventory_index {
//                use_item(inventory_index, tcod, game, objects);
//            }
            TookTurn
        }

        // DEBUG-KEYS
        ( Key { code: Text, .. }, "z", true) => { // Prints the list of items on the floor.
            println!("{:?}", items);
            DidntTakeTurn
        }
        ( Key { code: Text, .. }, "x", true) => { // Prints the player's inventory
            println!("{:?}", game.player.inventory.as_ref());
            DidntTakeTurn
        }
        ( Key { code: Text, .. }, "c", true) => {
//            Object::use_item(1, tcod, game, characters);
            DidntTakeTurn
        }

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
