use std::{collections::HashMap, thread::current};
use num::integer;
use crate::util::common_file;
use regex::Regex;

fn parse_map(line: &str) -> (&str, &str, &str) {

    let node_re = Regex::new(r"[A-Z]+").unwrap();
    let nodes: Vec<&str> = node_re.find_iter(line).map(|x| x.as_str()).collect();
    (nodes[0], nodes[1], nodes[2])
}

pub fn part1(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);

    let instructions = &file_string[0];
    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();

    for mapping in &file_string[2..file_string.len()] {
        let (src, l, r) = parse_map(mapping);
        node_map.insert(src, (l, r));
    }

    let mut found = false;
    let mut steps = 0;
    let mut instruction_index = 0usize;
    let mut current_node = "AAA";
    while !found {
        let instruction = instructions.as_bytes()[instruction_index] as char;
        match instruction {
            'L' => current_node = node_map[current_node].0,
            'R' => current_node = node_map[current_node].1,
            _ => continue
        };
        instruction_index = (instruction_index + 1) % instructions.len();
        steps += 1;
        if current_node.contains("ZZZ") {
            found = true;
        }
    }

    println!("Total number of steps is {steps}");
}

pub fn part2(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);
    let instructions = &file_string[0];
    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut node_locations: HashMap<&str, &str> = HashMap::new();
    let mut node_steps: HashMap<&str, u32> = HashMap::new();
    let mut node_cycles: HashMap<&str, u32> = HashMap::new();

    for mapping in &file_string[2..file_string.len()] {
        let (src, l, r) = parse_map(mapping);
        node_map.insert(src, (l, r));
        if src.ends_with("A") {
            node_locations.insert(src, src);
            node_steps.insert(src, 0u32);
        }
    }


    let mut instruction_index = 0usize;
    while node_cycles.len() != node_locations.len() {
        let instruction = instructions.as_bytes()[instruction_index] as char;
        for (node, steps) in node_steps.iter_mut() {
            if node_cycles.contains_key(node) {
                continue;
            }
            match instruction {
                'L' => {
                    if let Some(x) = node_locations.get_mut(node) {
                        *x = node_map[x].0;
                    }
                    *steps += 1;
                    if node_locations[node].ends_with("Z") {
                        node_cycles.insert(node, *steps);
                    }
                },
                'R' => {
                    if let Some(x) = node_locations.get_mut(node) {
                        *x = node_map[x].1;
                    }
                    *steps += 1;
                    if node_locations[node].ends_with("Z") {
                        node_cycles.insert(&node, *steps);
                    }
                },
                _ => continue,
            }
        }
        instruction_index = (instruction_index + 1) % instructions.len();
    }

    let total = node_cycles.iter().fold(1u64, |acc, x| {
        num::integer::lcm(acc, *x.1 as u64)
    });

    println!("Total is {total}");
    
    

}