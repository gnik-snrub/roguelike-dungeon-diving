use crate::environment::{ Map, MAP_WIDTH, MAP_HEIGHT };
use crate::environment::map::tiles::{ Tile };

#[derive(Debug)]
pub struct Broadfs {
    pub nodes: Vec<Node>,
    frontier: Vec<Node>,
    path: Vec<Node>,
}

impl Broadfs {
    pub fn new() -> Broadfs {
        let mut bfs = Broadfs {
            nodes: Vec::new(),
            frontier: Vec::new(),
            path: Vec::new(),
        };
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                bfs.nodes.push(Node::new(x as u32, y as u32));
            }
        }
        bfs
    }

    pub fn search(
        &mut self,
        map: &mut Map,
        start: (u32, u32),
        goal: Option<(u32, u32)>,
    ) {
        let start_node = Node::new(start.0, start.1);
        self.visit(start_node);
        self.frontier.push(start_node);

        while !self.frontier.is_empty() {
            let current = self.frontier.remove(0);
            if let Some(g) = goal {
                if g.0 == current.x && g.1 == current.y {
                    break
                }
            }
            for next in self.get_neighbors(current, map) {
                let (x, y) = next;
                match self.get_node(x, y) {
                    Some(node) => {
                        self.from_set(node, current);
                        self.expand_frontier(node);
                    },
                    None => {},
                }
            }
        }
    }

    fn expand_frontier(&mut self, node: Node) {
        let (x, y) = node.get_xy();
        match self.get_node(x, y) {
            Some(n) => {
                self.visit(n);
                self.frontier.push(n);
            }
            None => {},
        }
    }

    fn get_node(&self, x: u32, y: u32) -> Option<Node> {
        let indexed = self.nodes.iter().enumerate();
        for (_, node) in indexed {
            if node.x == x && node.y == y {
                return Some(*node)
            }
        }
        None
    }

    fn visit(&mut self, node: Node) {
        let mut node_index = 0;
        let indexed = self.nodes.iter().enumerate();
        for (id, searching) in indexed {
            if searching.x == node.x && searching.y == node.y {
                node_index = id;
            }
        }
        self.nodes[node_index].visit();
    }

    fn from_set(&mut self, node: Node, from: Node) {
        let mut node_index = 0;
        let indexed = self.nodes.iter().enumerate();
        for (id, searching) in indexed {
            if searching.x == node.x && searching.y == node.y {
                node_index = id;
            }
        }
        self.nodes[node_index].comes_from(from.get_xy());
    }

    pub fn breadcrumb(&mut self, start: (u32, u32), goal: Option<(u32, u32)>) {
        self.path.clear();
        if let Some(goal) = goal {
            let goal_node = self.get_node(goal.0, goal.1).unwrap();
            let start_node = self.get_node(start.0, start.1).unwrap();
            let mut current = goal_node;
            while !(current.x == start_node.x && current.y == start_node.y) {
                self.path.push(current);
                match self.get_node(current.comes_from.0, current.comes_from.1) {
                    Some(next) => current = next,
                    None => {},
                }
            }
        }
    }

    fn get_neighbors(&self, node: Node, map: &Map) -> Vec<(u32, u32)> {
        let (x, y) = (node.x, node.y);
        let mut neighbors: Vec<(u32, u32)> = vec![];

        if (x - 1) > 0 &&
        !map[(x - 1) as usize][y as usize].wall {
            match self.get_node(node.x - 1, node.y) {
                Some(neighbor) => if !neighbor.visited {
                    neighbors.push(neighbor.get_xy());
                },
                None => {},
            }
        }

        if (x + 1) < (MAP_WIDTH - 1) as u32 &&
        !map[(x + 1) as usize][y as usize].wall {
            match self.get_node(node.x + 1, node.y) {
                Some(neighbor) => if !neighbor.visited {
                    neighbors.push(neighbor.get_xy());
                },
                None => {},
            }
        }

        if (y - 1) > 0 &&
        !map[x as usize][(y - 1) as usize].wall {
            match self.get_node(node.x, node.y - 1) {
                Some(neighbor) => if !neighbor.visited {
                    neighbors.push(neighbor.get_xy());
                },
                None => {},
            }
        }

        if (y + 1) < (MAP_HEIGHT - 1) as u32 &&
        !map[x as usize][(y + 1) as usize].wall {
            match self.get_node(node.x, node.y + 1) {
                Some(neighbor) => if !neighbor.visited {
                    neighbors.push(neighbor.get_xy());
                },
                None => {},
            }
        }
        neighbors
    }

    pub fn show_path(&self, map: &mut Map) {
        for node in &self.path {
            map[node.x as usize][node.y as usize] = Tile::path();
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Node {
    pub x: u32,
    pub y: u32,
    pub visited: bool,
    comes_from: (u32, u32),
}

impl Node {
    fn new(x: u32, y: u32) -> Node {
        Node {
            x,
            y,
            visited: false,
            comes_from: (x, y),
        }
    }

    fn comes_from(&mut self, previous: (u32, u32)) {
        if !self.visited {
            let (x, y) = previous;
            self.comes_from = (x, y);
        }
    }

    pub fn get_xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn visit(&mut self) {
        self.visited = true;
    }
}
