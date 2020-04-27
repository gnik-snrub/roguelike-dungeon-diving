use crate::{ PLAYER, Tcod };
use crate::environment::{ Game, Map };
use super::Object;

#[derive(Debug)]
pub enum Ai {
    Basic,
}

impl Object {
    fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
        // Vector from this object to the target, and the distance.
        let dx = target_x - objects[id].x;
        let dy = target_y - objects[id].y;
        let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

        // Normalize to length 1 while keeping direction.
        // Then round, and convert to an integer so movement stays to map grid.
        let dx = (dx as f32 / distance).round() as i32;
        let dy = (dy as f32 / distance).round() as i32;
        Object::move_by(id, dx, dy, map, objects);
    }

    fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &mut Game, objects: &mut [Object]) {
        // A basic monster takes its turn. If you can see it, it can also see you!
        let (monster_x, monster_y) = objects[monster_id].pos();
        if tcod.fov.is_in_fov(monster_x, monster_y) {
            if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
                // Move towards player if far away.
                let (player_x, player_y) = objects[PLAYER].pos();
                Object::move_towards(monster_id, player_x, player_y, &game.map, objects);
            } else if objects[PLAYER].fighter.unwrap().hp >= 0 {
                // Close enough - Attack! (If player is alive)
                let (monster, player) = Object::mut_two(monster_id, PLAYER, objects);
                monster.attack(player, game);
            }
        }
    }
}
