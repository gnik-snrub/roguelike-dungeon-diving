pub mod bfs;

use bfs::Broadfs;

use crate::objects::Object;
use crate::environment::Map;
use crate::environment::map::tiles::Tile;

use tcod::colors::*;

pub fn remove_inaccessible_tiles(map: &mut Map, player: &Object, colors: &[Color; 7]) {
    let mut bfs = Broadfs::new();
    let (px, py) = player.pos();
    bfs.search(map, (px as u32, py as u32) , None);
    for node in &bfs.nodes {
        if !node.visited && map[node.x as usize][node.y as usize].empty {
            map[node.x as usize][node.y as usize] = Tile::wall(colors);
        }
    }
}
