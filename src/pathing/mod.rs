pub mod bfs;
/*
use crate::environment::{ Map, MAP_WIDTH, MAP_HEIGHT };
use crate::environment::map::tiles::Tile;

use crate::Point;

use std::collections::HashMap;

use rand::*;

fn draw_found(map: &mut Map, found_tiles: &HashMap<Point, Point>) {
    for (x, y) in found_tiles.keys() {
        map[*x as usize][*y as usize] = Tile::found(); // Paints tiles found in search.
    }
}

fn draw_path(map: &mut Map, found_tiles: &HashMap<Point, Point>, start: Point) {
    let mut breadcrumb = (0, 0);
    let mut found = false;
    while !found {
        let bcx = rand::thread_rng().gen_range(1, MAP_WIDTH - 1) as u32;
        let bcy = rand::thread_rng().gen_range(1, MAP_HEIGHT - 1) as u32;

        if map[bcx as usize][bcy as usize].empty == true {
            breadcrumb = (bcx, bcy);
            found = true;
        }
    }

//    let (bcx, bcy) = breadcrumb;
//    println!("{:?}", breadcrumb);
//    println!("{:?}", map[bcx as usize][bcy as usize]);
    let mut count = 0;

    while breadcrumb != start {
        let (bcx, bcy) = breadcrumb;
        map[bcx as usize][bcy as usize] = Tile::path();
        for (tile, came_from) in found_tiles {
//            println!("old: {:?} new: {:?}", breadcrumb, tile);
            if *came_from == breadcrumb {
                breadcrumb = *tile;
            }
        }

        count += 1;
        if count > 150 {
            breadcrumb = start;
        }
    }
}

fn mark_found(map: &mut Map, found_tiles: &HashMap<Point, Point>) {
    for (x, y) in found_tiles.keys() {
        map[*x as usize][*y as usize].found = true; // Paints tiles found in search.
    }
}
*/
