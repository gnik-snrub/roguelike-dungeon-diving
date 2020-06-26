use crate::graphics::render_map;
use crate::environment::*;
use crate::Tcod;

use std::cmp;

pub fn maze(
    map: &mut Map,
    colors: &[Color; 7],
    player: &mut Object,
    tcod: &mut Tcod,
    should_render: bool,
) {
    let mut maze = Maze::new(MAP_WIDTH as u32, MAP_HEIGHT as u32);
    // Creates a grid of empty tiles, and adds each point into a vector.
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            if x % 2 == 1 && y % 2 == 1 {
                map[x as usize][y as usize] = Tile::empty(colors);
                maze.points.push(Point::new(x as u32, y as u32));
            }
        }
        if should_render {
            render_map(tcod, map, 2);
        }
    }

    let mut counter = 0;

    loop {
        match maze.get_point(maze.get_x(), maze.get_y()) {
            Some(point) => {
                maze.visit(point);
                match maze.find_next(point) {
                    Some(new_point) => {
                        Maze::break_wall(point, new_point, map, colors);
                        maze.pos_move(new_point);
                        counter += 1;
                        if should_render && counter % 4 == 0 {
                            render_map(tcod, map, 2);
                        }
                    },
                    None => {
                        maze.backstep()
                    },
                }
            },
            None => {},
        }

        if maze.count_visited() == 0 {
            break
        }
    }

    // Place the player on a random empty tile.
    loop {
        let player_x = rand::thread_rng().gen_range(1, MAP_WIDTH - 1);
        let player_y = rand::thread_rng().gen_range(1, MAP_HEIGHT - 1);
        if map[player_x as usize][player_y as usize].empty == true {
            // Places player in the center of the room.
            player.set_pos(player_x, player_y);
            break;
        }
    }
}

#[derive(Debug)]
struct Maze {
    x: u32,
    y: u32,
    points: Vec<Point>,
    backtrack: Vec<(u32, u32)>,
    position: (u32, u32),
}
impl Maze {
    fn new(x: u32, y: u32) -> Maze {
        Maze {
            x,
            y,
            points: Vec::new(),
            backtrack: Vec::new(),
            position: (1, 1),
        }
    }

    fn pos_move(&mut self, point: Point) {
        let (x, y) = self.position;
        match self.get_point(point.x, point.y) {
            Some(point) => self.position = (point.x, point.y),
            None => self.position = (x, y),
        }
    }

    fn get_x(&self) -> u32 {
        let (x, _) = self.position;
        x
    }

    fn get_y(&self) -> u32 {
        let (_, y) = self.position;
        y
    }

    fn get_point(&self, x: u32, y: u32) -> Option<Point> {
        let indexed = self.points.iter().enumerate();
        for (_, point) in indexed {
            if point.x == x && point.y == y {
                return Some(*point)
            }
        }
        None
    }

    fn visit(&mut self, point: Point) {
        let mut p_index = 0;
        let indexed = self.points.iter().enumerate();
        for (id, searching) in indexed {
            if searching.x == point.x && searching.y == point.y {
                p_index = id;
            }
        }
        self.points[p_index].visit();
        self.backtrack.push(self.points[p_index].get_xy());
    }

    fn count_visited(&self) -> u32 {
        let mut count = 0;
        for point in &self.points {
            if !point.visited {
                count += 1;
            }
        }
        count
    }

    fn backstep(&mut self) {
        if !self.backtrack.is_empty() {
            self.position = self.backtrack[0];
            self.backtrack.remove(0);
        }
    }

    fn get_neighbors(&self, point: Point) -> Vec<Point> {
        let (x, y) = (point.x, point.y);
        let mut neighbors: Vec<Point> = vec![];

        if (x - 1) > 1 {
            match self.get_point(point.x - 2, point.y) {
                Some(neighbor) => if !neighbor.visited {
                    neighbors.push(neighbor);
                },
                None => {},
            }
        }

        if (x + 1) < (MAP_WIDTH) as u32 {
            match self.get_point(point.x + 2, point.y) {
                Some(neighbor) => if !neighbor.visited {
                    neighbors.push(neighbor)
                },
                None => {},
            }
        }

        if (y - 1) > 1 {
            match self.get_point(point.x, point.y - 2) {
                Some(neighbor) => if !neighbor.visited {
                    neighbors.push(neighbor)
                },
                None => {},
            }
        }

        if (y + 1) < (MAP_HEIGHT) as u32 {
            match self.get_point(point.x, point.y + 2) {
                Some(neighbor) => if !neighbor.visited {
                    neighbors.push(neighbor)
                },
                None => {},
            }
        }
        neighbors
    }

    fn find_next(&mut self, point: Point) -> Option<Point> {
        let neighbors = self.get_neighbors(point);
        if !neighbors.is_empty() {
            if neighbors.len() == 1 {
                return Some(neighbors[0])
            } else {
                return Some(neighbors[rand::thread_rng().gen_range(0, neighbors.len())])
            }
        }
        None
    }

    fn break_wall(one: Point, two: Point, map: &mut Map, colors: &[Color; 7]) {
        let (x1, y1) = one.get_xy();
        let (x2, y2) = two.get_xy();

        if x1 == x2 {
            for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
                map[x1 as usize][y as usize] = Tile::empty(colors);
            }
        } else if y1 == y2 {
            for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
                map[x as usize][y1 as usize] = Tile::empty(colors);
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: u32,
    y: u32,
    visited: bool,
}
impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point {
            x,
            y,
            visited: false,
        }
    }

    fn get_xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn visit(&mut self) {
        self.visited = true;
    }
}
