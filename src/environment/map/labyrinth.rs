use crate::environment::map::create_room;
use crate::environment::*;

use rand::*;

pub fn labyrinth(
    rooms: &mut Vec<Rect>,
    mut map: &mut Map,
    colors: &[Color; 7],
    player: &mut Object
) {
    for _ in 0..18 {
        // Random width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // Random position without going outside the map boundaries
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);

        // Run through the other rooms and see if they interact with this one`
        let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

        // Adds in rooms according to world path value
        if !failed {
            // Paints room onto map tiles
            create_room(new_room, &mut map, &colors);

            // Center coordinates of the new room, will be used later
            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                // This is the first room, where the player starts at
                player.set_pos(new_x, new_y);
            }

            rooms.push(new_room)
        }
    }
}
