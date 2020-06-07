use crate::environment::{ Map, MAP_WIDTH, MAP_HEIGHT };

use crate::Point;

use std::collections::HashMap;

#[derive(Debug)]
struct Queue {
    elements: Vec<Point>,
}

impl Queue {
    fn new(start: Point) -> Queue {
        Queue {
            elements: vec![start],
        }
    }
    fn get(&mut self) -> Point {
        self.elements.remove(self.elements.len() - 1)
    }
    fn put(&mut self, new: Point) {
        self.elements.push(new);
    }
}

fn neighbors(map: &Map, home: Point) -> Vec<Point> {
    let mut neighbors: Vec<Point> = vec![];
    let (home_x, home_y) = home;

    if 0 < home_y - 1
    && !map[home_x as usize][(home_y - 1) as usize].wall {
//        println!("Up");
        let up: Point = (home_x, home_y - 1);
        neighbors.push(up);
    }

    if MAP_HEIGHT as u32 > home_y + 1
    && !map[home_x as usize][(home_y + 1) as usize].wall {
//        println!("Down");
        let down: Point = (home_x, home_y + 1);
        neighbors.push(down);
    }

    if 0 < home_x - 1
    && !map[(home_x - 1) as usize][home_y as usize].wall {
//        println!("Left");
        let left: Point = (home_x - 1, home_y);
        neighbors.push(left);
    }

    if MAP_WIDTH as u32 > home_x + 1
    && !map[(home_x + 1) as usize][home_y as usize].wall {
//        println!("Right");
        let right: Point = (home_x + 1, home_y);
        neighbors.push(right);
    }

    if MAP_WIDTH as u32 > home_x + 1
    && MAP_HEIGHT as u32 > home_y + 1
    && !map[(home_x + 1) as usize][(home_y + 1) as usize].wall {
        let right_down: Point = (home_x + 1, home_y + 1);
        neighbors.push(right_down);
    }

    if MAP_WIDTH as u32 > home_x + 1
    && 0 < home_y - 1
    && !map[(home_x + 1) as usize][(home_y - 1) as usize].wall {
        let right_up: Point = (home_x + 1, home_y - 1);
        neighbors.push(right_up);
    }

    if 0 < home_x - 1
    && MAP_HEIGHT as u32 > home_y + 1
    && !map[(home_x - 1) as usize][(home_y + 1) as usize].wall {
        let left_down: Point = (home_x - 1, home_y + 1);
        neighbors.push(left_down);
    }

    if 0 < home_x - 1
    && 0 < home_y - 1
    && !map[(home_x - 1) as usize][(home_y - 1) as usize].wall {
        let left_up: Point = (home_x - 1, home_y - 1);
        neighbors.push(left_up);
    }



//    println!("{}", neighbors.len());

    neighbors
}

pub fn broad_first_search(map: &mut Map, start: Point) -> HashMap<Point, Point> {
    let mut frontier = Queue::new(start);
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    came_from.insert(start, start);

    while frontier.elements.len() > 0 {
        let current_point = frontier.get(); // Pulls from the top of the frontier stack
        for next in neighbors(map, current_point) { // Searches surrounding points
            // If neighbor is in visited vetor, it's discarded.
            // Otherwise, it gets added to the visited vector, and added to bottom of frontier stack
            match came_from.keys().find(|found| **found == next) {
                None => {
                    frontier.put(next);
                    came_from.insert(next, current_point);
                },
                Some(_) => {},
            }
        }
    }

    came_from
}




/*
MAKE A VECTOR OF THE START POINTS FOR THE DRUNK WALK
PUT CORRIDORS BETWEEN THEM
BAM, FIXED IT.

CHANGE THE CULL TILE ALGORITHM, SO IT JUST SEARCHES THROUGH THE FOUND TILES MAP
IF TILE ISNT FOUND, MAKE THE TILE A WALL.
REMOVE THE TILE ELEMENT FOUND.
*/
