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

pub fn check_direction(dir: &Direction, pos: (usize, usize), pipes: &Vec<Vec<char>>) -> Option<(Connection, Direction)> {
    let dir = dir.clone();
    let (mut next_x, mut next_y) = (pos.0, pos.1);

    let mut next_connection = Connection {
        pos: (next_x, next_y),
        next: pos,
        prev: pos,
    };

    match dir {
        Direction::UP => next_y -= 1,
        Direction::RIGHT => next_x += 1,
        Direction::DOWN => next_y += 1,
        Direction::LEFT => next_x -= 1
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
    for direction in DIRECTIONS {
        if let Some((mut next_connection, new_dir)) = check_direction(&direction, current.pos, &pipes) {
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
            let (mut new_connection, new_dir) = check_direction(&dir, current.pos, &pipes).unwrap();
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

pub fn part2(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);
    let pipes = file_string.into_iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

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


    const DIRECTIONS: [Direction; 4] = [Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT];
    let mut dir = Direction::RIGHT;
    let mut current = grid.get(&(s_x, s_y)).unwrap().clone();
    for direction in DIRECTIONS {
        if let Some((mut next_connection, new_dir)) = check_direction(&direction, current.pos, &pipes) {
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
            let (mut new_connection, new_dir) = check_direction(&dir, current.pos, &pipes).unwrap();
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
    let (min_x, max_x) = pipe_set.iter().fold((0usize, grid_max_x), |acc, x| {
        let mut min_x =acc.0;
        let mut max_x = acc.1;
        if x.0 < min_x {
            min_x = x.0;
        } else if x.0 > max_x {
            max_x = x.0;
        }
        (min_x, max_x)
    });

    let (min_y, max_y) = pipe_set.iter().fold((0usize, grid_max_y), |acc, x| {
        let mut min_y = acc.0;
        let mut max_y = acc.1;
        if x.0 < min_y {
            min_y = x.0;
        } else if x.0 > max_y {
            max_y = x.0;
        }
        (min_y, max_y)
    });
    
    let mut potential_set: HashSet<(usize, usize)> = HashSet::new();

    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            potential_set.insert((x, y));
        }
    }

    let open_set: HashSet<&(usize, usize)> = potential_set.difference(&pipe_set).collect();

    //for each open space, figure out if its enclosed within the loop
}