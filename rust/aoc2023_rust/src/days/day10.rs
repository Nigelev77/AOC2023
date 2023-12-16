use std::collections::{HashMap, HashSet};

use crate::util::common_file;
use lazy_static::lazy_static;

lazy_static! {
    static ref ACTIONS: HashMap<(char, Direction), Direction> = HashMap::from([
        (('-', Direction::RIGHT), Direction::RIGHT),
        (('-', Direction::LEFT), Direction::LEFT),
        (('|', Direction::UP), Direction::UP),
        (('|', Direction::DOWN), Direction::DOWN),
        (('L', Direction::DOWN), Direction::RIGHT),
        (('L', Direction::LEFT), Direction::UP),
        (('J', Direction::RIGHT), Direction::UP),
        (('J', Direction::DOWN), Direction::LEFT),
        (('7', Direction::UP), Direction::LEFT),
        (('7', Direction::RIGHT), Direction::DOWN),
        (('F', Direction::UP), Direction::RIGHT),
        (('F', Direction::LEFT), Direction::DOWN),
        (('S', Direction::LEFT), Direction::LEFT),
        (('S', Direction::DOWN), Direction::DOWN),
        (('S', Direction::RIGHT), Direction::RIGHT),
        (('S', Direction::UP), Direction::UP),
    ]);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

#[derive(Clone, Copy)]
pub struct Connection {
    pos: (usize, usize),
    next: (usize, usize),
    prev: (usize, usize),
}

pub fn check_direction(dir: &Direction, pos: (usize, usize), pipes: &Vec<Vec<char>>, bounds: Bounds) -> Option<(Connection, Direction)> {
    let dir = dir.clone();
    let (mut next_x, mut next_y) = (pos.0, pos.1);

    let mut next_connection = Connection {
        pos: (next_x, next_y),
        next: pos,
        prev: pos,
    };


    match dir {
        Direction::UP => { 
            if next_y <= 0 {
                return None;
            }
            next_y -= 1
        },
        Direction::RIGHT => {
            if next_x >= bounds.x_bounds.1 {
                return None;
            }
            next_x += 1
        },
        Direction::DOWN => {
            if next_y >= bounds.y_bounds.1 {
                return None;
            }
            next_y += 1
        },
        Direction::LEFT => {
            if next_x <= 0 {
                return None;
            }
            next_x -= 1
        }
    }

    let p = pipes[next_y][next_x];
    if !ACTIONS.contains_key(&(p, dir)) {
        return None;
    }

    next_connection.pos = (next_x, next_y);
    next_connection.prev = pos;
    next_connection.next = next_connection.pos; //by default have new connections just point back to itself

    let new_dir = ACTIONS.get(&(p, dir)).unwrap().clone();
    Some((next_connection, new_dir))
}

pub fn part1(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);
    let pipes = file_string.into_iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut s_x = 0usize;
    let mut s_y = 0usize;
    for (i, line) in pipes.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                s_x = j;
                s_y = i;
                break;
            }
        }
    }

    let start: Connection = Connection {
        pos: (s_x, s_y),
        next: (s_x, s_y),
        prev: (s_x, s_y),
    };

    let mut grid: HashMap<(usize, usize), Connection> = HashMap::new();

    grid.insert(start.pos, start);

    let s_x = s_x as usize;
    let s_y = s_y as usize;


    const DIRECTIONS: [Direction; 4] = [Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT];
    let mut dir = Direction::RIGHT;
    let mut current = grid.get(&(s_x, s_y)).unwrap().clone();
    let grid_bounds = Bounds {
        x_bounds: (0, pipes[0].len()),
        y_bounds: (0, pipes.len())
    };
    for direction in DIRECTIONS {
        if let Some((mut next_connection, new_dir)) = check_direction(&direction, current.pos, &pipes, grid_bounds) {
            current.next = next_connection.pos;
            next_connection.prev = current.pos;
            grid.insert(next_connection.pos, next_connection);
            grid.entry(current.pos).and_modify(|x| {
                x.next = current.next;
            });
            dir = new_dir;
            break;
        }
    }
    current = *grid.get_mut(&current.next).unwrap();

    while current.pos != (s_x, s_y) {
        //for each direction
        {
            let (mut new_connection, new_dir) = check_direction(&dir, current.pos, &pipes, grid_bounds).unwrap();
            current.next = new_connection.pos;
            new_connection.prev = current.pos;
            if grid.contains_key(&new_connection.pos) {
                grid.entry(new_connection.pos).and_modify(|x| {
                    x.prev = current.pos;
                });
            } else {
                grid.insert(new_connection.pos, new_connection);
            }
            grid.entry(current.pos).and_modify(|x| {
                x.next = current.next;
            });
            dir = new_dir;
        }
        current = *grid.get_mut(&current.next).unwrap();
    }

    let steps = grid.len() / 2;

    println!("Max steps is {steps}");


        
}

#[derive(Clone, Copy)]
pub struct Bounds {
    x_bounds: (usize, usize),
    y_bounds: (usize, usize)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Contains {
    In,
    Out
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum NodeType {
    Edge(Contains, Contains),
    Corner(Contains)
}

#[derive(Clone, Copy)]
pub struct DirectedConnection {
    node: Connection,
    node_type: NodeType
}


pub fn project_onto_directed_connection(pos: &(usize, usize), directed: &HashMap<(usize, usize), DirectedConnection>, bounds: Bounds) -> Contains {

    let mut next_pos = pos.clone();

    loop {
        next_pos.0 += 1;
        if next_pos.0 > bounds.x_bounds.1 {
            return Contains::Out;
        } else {
            if let Some(directed_conn) = directed.get(&next_pos) {
                if let NodeType::Edge(up_left, _) = directed_conn.node_type {
                    return up_left;
                } else if let NodeType::Corner(in_or_out) = directed_conn.node_type {
                    return in_or_out;
                }
            }
        }
        
    }
}

lazy_static!(
    static ref NODE_TYPE_MAP: HashMap<(char, char, NodeType), NodeType> = HashMap::from([
        (('|', 'F', NodeType::Corner(Contains::In)), NodeType::Edge(Contains::In, Contains::Out)),
        (('|', 'F', NodeType::Corner(Contains::Out)), NodeType::Edge(Contains::Out, Contains::In)),
        (('|', '7', NodeType::Corner(Contains::In)), NodeType::Edge(Contains::Out, Contains::In)),
        (('|', '7', NodeType::Corner(Contains::Out)), NodeType::Edge(Contains::In, Contains::Out)),
        (('|', 'J', NodeType::Corner(Contains::In)), NodeType::Edge(Contains::Out, Contains::In)),
        (('|', 'J', NodeType::Corner(Contains::Out)), NodeType::Edge(Contains::In, Contains::Out)),
        (('|', 'L', NodeType::Corner(Contains::In)), NodeType::Edge(Contains::In, Contains::Out)),
        (('|', 'L', NodeType::Corner(Contains::Out)), NodeType::Edge(Contains::Out, Contains::In)),
        (('-', 'F', NodeType::Corner(Contains::In)), NodeType::Edge(Contains::In, Contains::Out)),
        (('-', 'F', NodeType::Corner(Contains::Out)), NodeType::Edge(Contains::Out, Contains::In)),
        (('-', '7', NodeType::Corner(Contains::In)), NodeType::Edge(Contains::In, Contains::Out)),
        (('-', '7', NodeType::Corner(Contains::Out)), NodeType::Edge(Contains::Out, Contains::In)),
        (('-', 'J', NodeType::Corner(Contains::In)), NodeType::Edge(Contains::Out, Contains::In)),
        (('-', 'J', NodeType::Corner(Contains::Out)), NodeType::Edge(Contains::In, Contains::Out)),
        (('-', 'L', NodeType::Corner(Contains::In)), NodeType::Edge(Contains::Out, Contains::In)),
        (('-', 'L', NodeType::Corner(Contains::Out)), NodeType::Edge(Contains::In, Contains::Out)),
        (('F', 'J', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::Out)),
        (('F', 'J', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::In)),
        (('F', 'L', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::In)),
        (('F', 'L', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::Out)),
        (('F', '7', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::In)),
        (('F', '7', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::Out)),
        (('7', 'L', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::Out)),
        (('7', 'L', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::In)),
        (('7', 'J', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::In)),
        (('7', 'J', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::Out)),
        (('7', 'F', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::In)),
        (('7', 'F', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::Out)),
        (('J', 'F', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::Out)),
        (('J', 'F', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::In)),
        (('J', '7', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::In)),
        (('J', '7', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::Out)),
        (('J', 'L', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::Out)),
        (('J', 'L', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::In)),
        (('L', '7', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::Out)),
        (('L', '7', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::In)),
        (('L', 'F', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::In)),
        (('L', 'F', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::Out)),
        (('L', 'J', NodeType::Corner(Contains::In)), NodeType::Corner(Contains::In)),
        (('L', 'J', NodeType::Corner(Contains::Out)), NodeType::Corner(Contains::Out)),

    ]);
);

pub fn direct_grid(border_node: &Connection, grid: &HashMap<(usize, usize), Connection>, pipes: &Vec<Vec<char>>, bounds: Bounds) -> HashMap<(usize, usize), DirectedConnection> {
    //Determine first node's in and out
    let mut res: HashMap<(usize, usize), DirectedConnection> = HashMap::new();

    let border_pos = border_node.pos;
    let border_char = pipes[border_node.pos.1][border_node.pos.0];

    let node_type = match border_char {
        '-' => {
            if border_pos.1 == bounds.y_bounds.0 {
                NodeType::Edge(Contains::Out, Contains::In)
            } else {
                NodeType::Edge(Contains::In, Contains::Out)
            }
        },
        '|' => {
            if border_pos.0 == bounds.x_bounds.0 {
                NodeType::Edge(Contains::Out, Contains::In)
            } else {
                NodeType::Edge(Contains::In, Contains::Out)
            }
        },
        'F' => NodeType::Corner(Contains::Out),
        'L' => NodeType::Corner(Contains::Out),
        'J' => NodeType::Corner(Contains::Out),
        '7' => NodeType::Corner(Contains::Out),
        _ => panic!()
    };

    let directed_border_node = DirectedConnection {
        node: border_node.clone(),
        node_type
    };

    // res.push(directed_border_node);
    res.insert(directed_border_node.node.pos, directed_border_node);

    let mut current_node = directed_border_node;
    let grid_number = grid.len();
    //iterate over all connections
    for _ in 0..grid_number {
        let current_pos = current_node.node.pos;
        if current_pos == (6, 6){
            let a = 0;
            println!("{a}");
        }
        let current_char = pipes[current_pos.1][current_pos.0];

        let next_pos = current_node.node.next;
        let next_node = grid.get(&next_pos).unwrap();
        let next_char = pipes[next_pos.1][next_pos.0];
        let current_node_type = current_node.node_type;
        let next_node_type = match next_char {
            '|' => {
                if let NodeType::Edge(_, _) = current_node_type {
                    current_node_type
                } else {
                    *NODE_TYPE_MAP.get(&(next_char, current_char, current_node_type)).unwrap()
                } 
            },
            '-' => {
                if let NodeType::Edge(_, _) = current_node_type {
                    current_node_type
                } else {
                    *NODE_TYPE_MAP.get(&(next_char, current_char, current_node_type)).unwrap()
                } 
            },
            'F' => {
                if let NodeType::Edge(up_left, _) = current_node_type {
                    NodeType::Corner(up_left)
                } else {
                    *NODE_TYPE_MAP.get(&(next_char, current_char, current_node_type)).unwrap()
                    //if 7 take same value, if J take opposite value
                    
                }
            },
            'L' => {
                if let NodeType::Edge(up_left, down_right) = current_node_type {
                    if current_char == '-' {
                        NodeType::Corner(down_right)
                    } else {
                        NodeType::Corner(up_left)
                    }
                } else {
                    *NODE_TYPE_MAP.get(&(next_char, current_char, current_node_type)).unwrap()
                    //if J take same value, if F take opposite value
                }
            },
            'J' => {
                if let NodeType::Edge(_, down_right) = current_node_type {
                    NodeType::Corner(down_right)
                } else {
                    *NODE_TYPE_MAP.get(&(next_char, current_char, current_node_type)).unwrap()
                    //if 7 take same value, if J take opposite value
                }
            }
            '7' => {
                if let NodeType::Edge(up_left, down_right) = current_node_type {
                    if current_char == '-' {
                        NodeType::Corner(up_left)
                    } else {
                        NodeType::Corner(down_right)
                    }
                } else {
                    *NODE_TYPE_MAP.get(&(next_char, current_char, current_node_type)).unwrap()
                    //if 7 take same value, if J take opposite value
                }
            }
            _ => panic!()
        };

        let next_directed_connection = DirectedConnection {
            node: *next_node,
            node_type: next_node_type
        };

        current_node = next_directed_connection;
        // res.push(next_directed_connection);
        res.insert(next_directed_connection.node.pos, next_directed_connection);

    };

    res
}

pub fn part2(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);
    let mut pipes = file_string.into_iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let grid_max_x = pipes[0].len();
    let grid_max_y = pipes.len();

    let mut s_x = 0usize;
    let mut s_y = 0usize;
    for (i, line) in pipes.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                s_x = j;
                s_y = i;
                break;
            }
        }
    }

    let start: Connection = Connection {
        pos: (s_x, s_y),
        next: (s_x, s_y),
        prev: (s_x, s_y),
    };

    let mut grid: HashMap<(usize, usize), Connection> = HashMap::new();

    grid.insert(start.pos, start);

    let s_x = s_x as usize;
    let s_y = s_y as usize;
    let grid_bounds = Bounds {
        x_bounds: (0, pipes[0].len()),
        y_bounds: (0, pipes.len())
    };


    const DIRECTIONS: [Direction; 4] = [Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT];
    let mut dir = Direction::RIGHT;
    let mut current = grid.get(&(s_x, s_y)).unwrap().clone();
    for direction in DIRECTIONS {
        if let Some((mut next_connection, new_dir)) = check_direction(&direction, current.pos, &pipes, grid_bounds) {
            current.next = next_connection.pos;
            next_connection.prev = current.pos;
            grid.insert(next_connection.pos, next_connection);
            grid.entry(current.pos).and_modify(|x| {
                x.next = current.next;
            });
            dir = new_dir;
            break;
        }
    }
    current = *grid.get_mut(&current.next).unwrap();


    while current.pos != (s_x, s_y) {
        //for each direction
        {
            let (mut new_connection, new_dir) = check_direction(&dir, current.pos, &pipes, grid_bounds).unwrap();
            current.next = new_connection.pos;
            new_connection.prev = current.pos;
            if grid.contains_key(&new_connection.pos) {
                grid.entry(new_connection.pos).and_modify(|x| {
                    x.prev = current.pos;
                });
            } else {
                grid.insert(new_connection.pos, new_connection);
            }
            grid.entry(current.pos).and_modify(|x| {
                x.next = current.next;
            });
            dir = new_dir;
        }
        current = *grid.get_mut(&current.next).unwrap();
    }


    let pipe_set: HashSet<(usize, usize)> = grid.keys().fold(HashSet::new(), |mut acc, x| {
        acc.insert(*x);
        acc
    });

    let (min_x, max_x) = pipe_set.iter().fold((grid_max_x, 0usize), |acc, x| {
        let mut min_x = acc.0;
        let mut max_x = acc.1;
        if x.0 < min_x {
            min_x = x.0;
        } else if x.0 > max_x {
            max_x = x.0;
        }
        (min_x, max_x)
    });

    let (min_y, max_y) = pipe_set.iter().fold((grid_max_y, 0usize), |acc, x| {
        let mut min_y = acc.0;
        let mut max_y = acc.1;
        if x.1 < min_y {
            min_y = x.1;
        } else if x.1 > max_y {
            max_y = x.1;
        }
        (min_y, max_y)
    });
    
    let bounds = Bounds {
        x_bounds: (min_x, max_x),
        y_bounds: (min_y, max_y)
    };
    
    let mut potential_set: HashSet<(usize, usize)> = HashSet::new();

    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            potential_set.insert((x, y));
        }
    }

    let tile_set: HashSet<&(usize, usize)> = potential_set.difference(&pipe_set).collect();


    let start_node = grid.get(&(s_x, s_y)).unwrap();
    let (dx, dy) = (start_node.next.0 as i32 - start_node.prev.0 as i32, start_node.next.1 as i32 - start_node.prev.1 as i32);
    let arm1 = (start_node.next.0 as i32 - start_node.pos.0 as i32, start_node.next.1 as i32 - start_node.pos.1 as i32);
    let arm2 = (start_node.pos.0 as i32 - start_node.prev.0 as i32, start_node.pos.1 as i32 - start_node.prev.1 as i32);
    
    let actual_start_node = match (dx, dy) {
        (2, 0) | (-2, 0) => '-',
        (0, 2) | (0, -2) => '|',
        _ => {
            if arm1.0 == 0 { //arm1 is the vertical
                if arm1.1 > 0 {
                    if arm2.0 < 0 { 'F' } else { '7' }
                } else {
                    if arm2.0 < 0 { 'L' } else { 'J' }
                }
            } else { //arm2.0 == 0
                if arm2.1 < 0 {
                    if arm1.0 > 0 { 'F' } else { '7' }
                } else {
                    if arm1.0 > 0 { 'L' } else { 'J' }
                }
            }
        }
    };

    pipes[s_y][s_x] = actual_start_node;

    //Find outer boundary node

    let outer_bounds_pipe = grid.iter().find(|x| x.0.0 == bounds.x_bounds.0).unwrap();
    let directed_grid = direct_grid(outer_bounds_pipe.1, &grid, &pipes, bounds);

    let mut total_contains = 0usize;
    for pos in tile_set {
        if project_onto_directed_connection(pos, &directed_grid, bounds) == Contains::In {
            total_contains += 1;
        }
    }
    
    //A small optimisation would be that you cache the reuslts for each position and if in the projection onto directed grid,
    //it hits a position already in the cache, then it must be in since these contained and uncontained tiles occur in groups
    //Won't make much of a difference in terms of performance though
    println!("Total contained is {total_contains}");
}