
use crate::util::{common_file, common_regex};

struct Race {
    time: u32,
    distance: u32
}

pub fn part1(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);
    let times = common_regex::parse_nums_in_str(file_string[0].as_str());
    let distances = common_regex::parse_nums_in_str(file_string[1].as_str());

    let mut total = 1u32;
    
    for (time, distance) in times.iter().zip(distances.iter()) {
          let disc = time.pow(2) - 4 * distance;
          let disc = (disc as f64).sqrt();
          let upper = (((*time as f64) + disc) / 2f64).ceil() - 1f64;
          let lower = (((*time as f64) - disc) / 2f64).floor() + 1f64;
           let options = upper - lower;
          total *= options as u32 + 1u32;
    }

    println!("Total number of options is {total}");
}


pub fn part2(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);
    let time = common_regex::parse_digits_into_number(file_string[0].as_str());
    let distance = common_regex::parse_digits_into_number(file_string[1].as_str());

    let mut total = 1u32;
    
    let disc = time.pow(2) - 4 * distance;
    let disc = (disc as f64).sqrt();
    let upper = (((time as f64) + disc) / 2f64).ceil() - 1f64;
    let lower = (((time as f64) - disc) / 2f64).floor() + 1f64;
    let options = upper - lower;
    total *= options as u32 + 1u32;

    println!("Total number of options is {total}");
}

