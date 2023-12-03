use std::collections::HashMap;

use crate::util::common_file;
use regex::Regex;

pub fn part1(input: &str) {
    let file_input = common_file::read_file(input);

    let special_re = Regex::new(r"[^\d|^\.]").unwrap();

    //let mut special_locations: Vec<(usize, usize)> = Vec::new();
    
    let grid: Vec<&str> = file_input.split("\r\n").collect();
    let mut numbers: Vec<u32> = Vec::new();
    
    // for (height, line) in grid.iter().enumerate(){
    //     for match_loc in special_re.find_iter(line){
    //         special_locations.push((match_loc.start(), height));
    //     }
    // }

    // for (col, row) in special_locations {

    // }
    
    let mut total: u32 = 0;

    let digit_re = Regex::new(r"\d+").unwrap();
    let height = grid.len();
    for (row, line) in grid.iter().enumerate(){
        for digit_match in digit_re.find_iter(line) {
            let (span, number) = (digit_match.range(), digit_match.as_str());
            let (start, end) = (span.start, span.end);
            let mut found = false;
            for l in (1.max(row) - 1)..(height.min(row + 2)) {
                let row_str = grid[l];
                for c in row_str[(1).max(start)-1..(line.len().min(end+1))].chars(){
                    if special_re.is_match(c.to_string().as_str()) {
                        total += number.parse::<u32>().unwrap();
                        numbers.push(number.parse::<u32>().unwrap());
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
                
            }
        }
    }

    println!("Total is {total}");
}

pub fn part2(input: &str){
    let mut gear_map : HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let file_input = common_file::read_file(input);

    let special_re = Regex::new(r"\*").unwrap();

    //let mut special_locations: Vec<(usize, usize)> = Vec::new();
    
    let grid: Vec<&str> = file_input.split("\r\n").collect();
    
    //Can either go through each number and find if they are adjacent to a * and add to a map. After go through each
    //entry in the map to see if there's exactly 2 numbers next to it 
    //OR
    //Iterate over each * and find if theres exactly two numbers adjacent. May be a bit messier but would 
    //technically be "faster"
    
    let mut total: u32 = 0;

    let digit_re = Regex::new(r"\d+").unwrap();
    let height = grid.len();
    for (row, line) in grid.iter().enumerate(){
        for digit_match in digit_re.find_iter(line) {
            let (span, number) = (digit_match.range(), digit_match.as_str());
            let (start, end) = (span.start, span.end);
            for l in (1.max(row) - 1)..(height.min(row + 2)) {
                let row_str = grid[l];
                let left = 1.max(start) - 1;
                let right = line.len().min(end+1);
                for char_idx in left..right{
                    let c = row_str.as_bytes()[char_idx];
                    if c == b'*' {
                        let number = number.parse::<u32>().unwrap();
                        if let Some(val) = gear_map.get_mut(&(char_idx, l)){
                            val.push(number);
                        } else {
                            let mut new_gear: Vec<u32> = Vec::new();
                            new_gear.push(number);
                            gear_map.insert((char_idx, l), new_gear);
                        }
                    }
                }
                
            }
        }
    }

    for (_, val) in gear_map.iter_mut() {
        if val.len() == 2 {
            total += val[0] * val[1];
        }
    }

    println!("Total gear score is {total}");
}
