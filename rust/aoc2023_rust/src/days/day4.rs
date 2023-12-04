
use std::collections::{HashSet, HashMap};

use crate::util::common_file;
use regex::Regex;


pub fn part1(input: &str) {
    let cards: Vec<String> = common_file::read_file_into_buffer(input);
    let number_re = Regex::new(r"\d+").unwrap();
    let mut total = 0u32;
    
    for card_str in cards {
        let (colon, line) = (card_str.find(":").unwrap(), card_str.find("|").unwrap());
        let winning_list: Vec<u32> = number_re.find_iter(&card_str.as_str()[colon+1..line])
                                              .map(|x| x.as_str().parse::<u32>().unwrap()).collect();
        let card_list: Vec<u32> = number_re.find_iter(&card_str.as_str()[line..])
                                              .map(|x| x.as_str().parse::<u32>().unwrap()).collect();
        let winning_set: HashSet<u32> = winning_list.into_iter().collect();
        let card_set: HashSet<u32> = card_list.into_iter().collect();

        let winning_count = card_set.intersection(&winning_set).count() as u32;
        if winning_count > 0 {
            total += 2u32.pow(winning_count - 1);
        }

    }
    println!("Total points {total}");
}

pub fn part2(input: &str) {
    let cards: Vec<String> = common_file::read_file_into_buffer(input);
    let number_re = Regex::new(r"\d+").unwrap();

    let mut card_vec: Vec<u32> = vec![1; cards.len()];
    
    for (i, card_str) in cards.iter().enumerate() {
        let i: u32 = i as u32;
        let (colon, line) = (card_str.find(":").unwrap(), card_str.find("|").unwrap());

        let matches: u32;

        let winning_list: Vec<u32> = number_re.find_iter(&card_str.as_str()[colon+1..line])
        .map(|x| x.as_str().parse::<u32>().unwrap()).collect();
        let card_list: Vec<u32> = number_re.find_iter(&card_str.as_str()[line..])
                    .map(|x| x.as_str().parse::<u32>().unwrap()).collect();
        let winning_set: HashSet<u32> = winning_list.into_iter().collect();
        let card_set: HashSet<u32> = card_list.into_iter().collect();

        matches = card_set.intersection(&winning_set).count() as u32;

        for x in i+1..i+1+matches {
            card_vec[x as usize] += card_vec[i as usize];
        }
        

        
    }
    let total: u32 = card_vec.iter().sum();


    println!("Total number of scratchcards = {total}");
}