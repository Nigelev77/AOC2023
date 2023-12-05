use std::{collections::HashMap, ops::Range, str::FromStr};
use regex::Regex;

use crate::util::common_file;
use crate::util::common_regex;

struct RangeMap {
    src_range: Range<u64>,
    dst_range: Range<u64>
}

struct SeedRange {
    start: u64,
    range: u64
}

fn parse_range(file: &Vec<String>, start: usize, end: usize) -> Vec<RangeMap> {

    let mut res: Vec<RangeMap> = Vec::new();

    for line in &file[start..end-1] {
        let values = common_regex::parse_nums_in_str(&line);
        let (dst, src, range) = (values[0] as u64, values[1] as u64, values[2] as u64);
        let src_range = Range{start: src, end: src+range};
        let dst_range = Range{start: dst, end: dst+range};
        res.push(RangeMap { src_range, dst_range });
    }
    
    res
}

fn parse_seed_range(seed_line: &Vec<u32>) -> Vec<SeedRange> {
    let mut res: Vec<SeedRange> = Vec::new();

    for i in (0..seed_line.len()).step_by(2) {
        let (start, range) = (seed_line[i] as u64, seed_line[i+1] as u64);
        res.push(SeedRange { start, range });
    }

    
    res
}

pub fn part1(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);

    let seeds = common_regex::parse_nums_in_str(&file_string[0]);
    

    let seed_soil_line = file_string.iter().position(|x| x.eq(&"seed-to-soil map:".to_string())).unwrap();
    let soil_fert_line = file_string.iter().position(|x| x.eq(&"soil-to-fertilizer map:".to_string())).unwrap();
    let fert_water_line = file_string.iter().position(|x| x.eq(&"fertilizer-to-water map:".to_string())).unwrap();
    let water_light_line = file_string.iter().position(|x| x.eq(&"water-to-light map:".to_string())).unwrap();
    let light_temp_line = file_string.iter().position(|x| x.eq(&"light-to-temperature map:".to_string())).unwrap();
    let temp_hum_line = file_string.iter().position(|x| x.eq(&"temperature-to-humidity map:".to_string())).unwrap();
    let hum_loc_line = file_string.iter().position(|x| x.eq(&"humidity-to-location map:".to_string())).unwrap();
    
    let seed_soil = parse_range(&file_string, seed_soil_line+1, soil_fert_line);
    let soil_fert = parse_range(&file_string, soil_fert_line+1, fert_water_line);
    let fert_water = parse_range(&file_string, fert_water_line+1, water_light_line);
    let water_light = parse_range(&file_string, water_light_line+1, light_temp_line);
    let light_temp = parse_range(&file_string, light_temp_line+1, temp_hum_line);
    let temp_hum = parse_range(&file_string, temp_hum_line+1, hum_loc_line);
    let hum_loc = parse_range(&file_string, hum_loc_line+1, file_string.len());

    let maps = vec![
        seed_soil,
        soil_fert,
        fert_water,
        water_light,
        light_temp,
        temp_hum,
        hum_loc
    ];

    let mut smallest_loc = u64::MAX;
        for seed in seeds {
            let mut loc = seed as u64;
            for map in &maps {
                if let Some(src_map) = map.iter().find(|x| x.src_range.contains(&loc)) {
                    loc = src_map.dst_range.start + (loc - src_map.src_range.start);
                }
            }
            if loc < smallest_loc {
                smallest_loc = loc;
            }
    }
    
    println!("Smallest loc is {smallest_loc}");
}

pub fn part2(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);

    let seeds = common_regex::parse_nums_in_str(&file_string[0]);
    
    let current_location_map = parse_seed_range(&seeds);

    let seed_soil_line = file_string.iter().position(|x| x.eq(&"seed-to-soil map:".to_string())).unwrap();
    let soil_fert_line = file_string.iter().position(|x| x.eq(&"soil-to-fertilizer map:".to_string())).unwrap();
    let fert_water_line = file_string.iter().position(|x| x.eq(&"fertilizer-to-water map:".to_string())).unwrap();
    let water_light_line = file_string.iter().position(|x| x.eq(&"water-to-light map:".to_string())).unwrap();
    let light_temp_line = file_string.iter().position(|x| x.eq(&"light-to-temperature map:".to_string())).unwrap();
    let temp_hum_line = file_string.iter().position(|x| x.eq(&"temperature-to-humidity map:".to_string())).unwrap();
    let hum_loc_line = file_string.iter().position(|x| x.eq(&"humidity-to-location map:".to_string())).unwrap();
    
    let seed_soil = parse_range(&file_string, seed_soil_line+1, soil_fert_line);
    let soil_fert = parse_range(&file_string, soil_fert_line+1, fert_water_line);
    let fert_water = parse_range(&file_string, fert_water_line+1, water_light_line);
    let water_light = parse_range(&file_string, water_light_line+1, light_temp_line);
    let light_temp = parse_range(&file_string, light_temp_line+1, temp_hum_line);
    let temp_hum = parse_range(&file_string, temp_hum_line+1, hum_loc_line);
    let hum_loc = parse_range(&file_string, hum_loc_line+1, file_string.len());

    let maps = vec![
        seed_soil,
        soil_fert,
        fert_water,
        water_light,
        light_temp,
        temp_hum,
        hum_loc
    ];

    //this way will be too long
    //The probably actual way is to transform the ranges themselves, between each map
    //So you get a singular vector of RangeMaps from seed to final location


    //create a queue of ranges
    //iterate through each range and find the intersections of that range with the next maps
    //to generate a new queue of ranges which have been mapped from the previous ranges
    //final output should be a vec of ranges. The smallest loc is the range with the smallest start
    let mut smallest_loc = u64::MAX;
    for SeedRange{start, range} in current_location_map {
        for seed in start..start+range {
            let mut loc = seed as u64;
            for map in &maps {
                if let Some(src_map) = map.iter().find(|x| x.src_range.contains(&loc)) {
                    loc = src_map.dst_range.start + (loc - src_map.src_range.start);
                }
            }
            if loc < smallest_loc {
                smallest_loc = loc;
            } 
        }
    }

    println!("Smallest location in ranges is {smallest_loc}");
}