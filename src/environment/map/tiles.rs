use crate::*;

use tcod::colors::*;

use rand::*;

use serde::{ Serialize, Deserialize };

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TileType {
    Empty,
    Wall,
    SecretPath,
    Debug,
}

// Tile struct definition.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub empty: bool,
    pub wall: bool,
    pub secret_path: bool,
    pub found: bool,
    pub blocked: bool,
    pub explored: bool,
    pub block_sight: bool,
    pub color_light: Color,
    pub color_dark: Color,
    pub tiletype: TileType,
}

impl Tile {

    // Used to create an empty tile.
    pub fn empty(colors: &[Color; 7]) -> Tile {
        // Max chance is used to vary the frequency at which color variants appear.
        let max_chance = rand::thread_rng().gen_range(4, 15);

        // A higher max chance means a lower likelihood of variants showing up.
        let color_light = match rand::thread_rng().gen_range(1, max_chance) {
            1 => colors[LIGHT_GROUND_COLOR + V_ONE],
            2 => colors[LIGHT_GROUND_COLOR + V_TWO],
            _ => colors[LIGHT_GROUND_COLOR]
        };

        // Dark darkness modifier is applied to light color to create the shaded variant.
        let color_dark = color_light - colors[DARKNESS_MODIFIER];

        // Tile is returned using these two colors.
        Tile {
            empty: true,
            wall: false,
            secret_path: false,
            found: false,
            blocked: false,
            explored: false,
            block_sight: false,
            color_light: color_light,
            color_dark: color_dark,
            tiletype: TileType::Empty,
        }
    }

    pub fn wall(colors: &[Color; 7]) -> Tile {
        // Wall colors are just the light version, and the light version with the darkness modifier attached.
        let color_light = colors[LIGHT_WALL_COLOR];
        let color_dark = color_light - colors[DARKNESS_MODIFIER];

        // Tile is returned.
        Tile {
            empty: false,
            wall: true,
            secret_path: false,
            found: false,
            blocked: true,
            explored: false,
            block_sight: true,
            color_light: color_light,
            color_dark: color_dark,
            tiletype: TileType::Wall,
        }
    }

    pub fn hidden_passage(colors: &[Color; 7]) -> Tile {
        // Secret path colors are the variants pulled from the base wall color.
        let color_light = match rand::thread_rng().gen_range(1, 3) {
            1 => colors[LIGHT_WALL_COLOR + V_ONE],
            _ => colors[LIGHT_WALL_COLOR + V_TWO],
        };
        let color_dark = color_light - colors[DARKNESS_MODIFIER];

        // Tile is returned.
        Tile {
            empty: false,
            wall: false,
            secret_path: true,
            found: false,
            blocked: false,
            explored: false,
            block_sight: true,
            color_light: color_light,
            color_dark: color_dark,
            tiletype: TileType::SecretPath,
        }
    }

    // Found, and Path are just debug tiles.
    pub fn found() -> Tile {
        Tile {
            empty: true,
            wall: false,
            secret_path: true,
            found: true,
            blocked: false,
            explored: false,
            block_sight: true,
            color_light: tcod::colors::DARK_BLUE,
            color_dark: tcod::colors::DARK_RED,
            tiletype: TileType::Debug,
        }
    }

    pub fn path() -> Tile {
        Tile {
            empty: true,
            wall: false,
            secret_path: true,
            found: true,
            blocked: false,
            explored: false,
            block_sight: true,
            color_light: tcod::colors::LIGHT_GREEN,
            color_dark: tcod::colors::LIGHT_GREEN,
            tiletype: TileType::Debug,
        }
    }
}
