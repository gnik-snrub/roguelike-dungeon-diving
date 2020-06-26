use crate::*;

const INVENTORY_WIDTH: i32 = 50;

pub fn menu<T: AsRef<str>>(
    header: &str,
    options: &[T],
    width: i32,
    root: &mut Root
) -> Option<usize> {

    // Ensure that the menu stays within the alphabet limit.
    assert!(options.len() <= 26, "Cannot have a menu with more than 26 options.");

    // Calculates the height of the header, and one line per option.
    let header_height = if header.is_empty() {
        0 // If there is no header, there is no height.
    } else {
        root.get_height_rect(0, 0, width, SCREEN_HEIGHT, header)
    };
    let height = options.len() as i32 + header_height;

    // Create an off-screen console representing the menu's window.
    let mut window = Offscreen::new(width, height);

    // Print the header with auto-wrap.
    window.set_default_foreground(WHITE);
    window.print_rect_ex(
        0,
        0,
        width,
        height,
        BackgroundFlag::None,
        TextAlignment::Left,
        header,
    );

    // Print menu options.
    for (index, option_text) in options.iter().enumerate() {
        // Establishes text to print option line, including the selection option.
        let menu_letter = (b'a' + index as u8) as char;
        let text = format!("({}) {}", menu_letter, option_text.as_ref());

        // Prints it to the window.
        window.print_ex(
            0,
            header_height + index as i32,
            BackgroundFlag::None,
            TextAlignment::Left,
            text,
        );
    }

    // Blit the contents of "window" to the root console.
    let x = SCREEN_WIDTH / 2 - width / 2;
    let y = SCREEN_HEIGHT / 2 - height / 2;
    blit(&window, (0, 0), (width, height), root, (x, y), 1.0, 0.7);

    // Present the root console to the player, and await a keypress.
    root.flush();
    let key = root.wait_for_keypress(true);

    // Convert the ASCII code to an index; If it references an option, return i.
    if key.printable.is_alphabetic() {
        let index = key.printable.to_ascii_lowercase() as usize - 'a' as usize;
        if index < options.len() {
            Some(index)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn inventory_menu(player: &Character, header: &str, root: &mut Root) -> Option<usize> {
    // Collects inventory items, using an empty vec, in case the inventory is inaccessible for some reason.
    let empty_vec = Vec::new();
    let inventory = match &player.inventory {
        Some(items) => items,
        None => &empty_vec,
    };

    // Show a menu with each inventory item as an option.
    let options = if inventory.len() == 0 {
            vec!["Inventory is empty.".into()]
        } else {
            inventory.iter().map(|item| item.name.clone()).collect()
    };

    // Creates a menu, and collects the choice made by the player.
    let inventory_index = menu(header, &options, INVENTORY_WIDTH, root);

    // If an item was chosen, return it.
    if inventory.len() > 0 {
        inventory_index
    } else {
        None
    }
}

// Uses the menu function to display a simple message box.
// Message displayed is the "text" variable msgbox takes.
pub fn msgbox(text: &str, width: i32, root: &mut Root) {
    let options: &[&str] = &[];
    menu(text, options, width, root);
}
