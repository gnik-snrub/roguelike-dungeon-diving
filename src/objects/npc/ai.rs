use crate::Tcod;
use crate::environment::{ Game, Map };
use super::Object;

use rand::Rng;

use tcod::colors::*;

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

    pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, mut game: &mut Game, objects: &mut [Object]) {
        // A basic monster takes its turn. If you can see it, it can also see you!
        let (monster_x, monster_y) = objects[monster_id].pos();
        if tcod.fov.is_in_fov(monster_x, monster_y) {
            if objects[monster_id].distance_to(&game.player) >= 2.0 {
                // Move towards player if far away.
                let (player_x, player_y) = game.player.pos();
                Object::move_towards(monster_id, player_x, player_y, &game.map, objects);
            } else if game.player.fighter.unwrap().hp >= 0 {
                // Close enough - Attack! (If player is alive)
                objects[monster_id].monster_attack(&mut game);
            }
        }
    }

    fn monster_attack(&self, game: &mut Game) {
        let mut rng = rand::thread_rng();
        let attack = (self.fighter.map_or(0, |f| f.power)) as f32 + rng.gen_range(-1.0, 1.0);
        let defense = (game.player.fighter.map_or(0, |f| f.defense)) as f32 + rng.gen_range(-1.0, 1.0);
        let level_mod =
            (self.fighter.unwrap().level as f32).sqrt().powf((self.fighter.unwrap().level as f32) / 2.0) /
            (self.fighter.unwrap().level as f32).sqrt().powf((self.fighter.unwrap().level as f32) * 0.25);
        let damage = (attack / defense * level_mod).round() as i32;
        if damage > 0 {
            // Target takes damage.
            game.messages.add(
                format!(
                    "{} attacks {} dealing {} damage.",
                    self.name, game.player.name, damage
                ),
                self.color,
            );
            Object::player_damage(damage, game);
        } else {
            game.messages.add(
                format!(
                    "{} attacks {} but it has no effect!",
                    self.name, game.player.name
                ),
                WHITE,
            );
        }
    }
}
