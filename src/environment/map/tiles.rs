use crate::*;

use tcod::colors::*;

use rand::*;

use serde::{ Serialize, Deserialize };

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub blocked: bool,
    pub explored: bool,
    pub block_sight: bool,
    pub color_light: Color,
    pub color_dark: Color,
}

impl Tile {
    pub fn empty(colors: &[Color; 7]) -> Tile {
        let max_chance = rand::thread_rng().gen_range(4, 15);
        let color_light = match rand::thread_rng().gen_range(1, max_chance) {
            1 => colors[LIGHT_GROUND_COLOR + V_ONE],
            2 => colors[LIGHT_GROUND_COLOR + V_TWO],
            _ => colors[LIGHT_GROUND_COLOR]
        };
        let color_dark = color_light - colors[DARKNESS_MODIFIER];

        Tile {
            blocked: false,
            explored: false,
            block_sight: false,
            color_light: color_light,
            color_dark: color_dark,
        }
    }

    pub fn wall(colors: &[Color; 7]) -> Tile {
        let color_light = colors[LIGHT_WALL_COLOR];
        let color_dark = color_light - colors[DARKNESS_MODIFIER];

        Tile {
            blocked: true,
            explored: false,
            block_sight: true,
            color_light: color_light,
            color_dark: color_dark,
        }
    }

    pub fn hidden_passage(colors: &[Color; 7]) -> Tile {
        let color_light = colors[LIGHT_WALL_COLOR];
        let color_dark = color_light - colors[DARKNESS_MODIFIER];

        Tile {
            blocked: false,
            explored: false,
            block_sight: true,
            color_light: color_light,
            color_dark: color_dark,
        }
    }
}
