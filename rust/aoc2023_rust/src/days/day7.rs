use std::cmp::Ordering;
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::util::common_file;

lazy_static! {
    static ref RANKINGS: HashMap<char, u32> = HashMap::from([
        ('A', 1u32),
        ('K', 2u32),
        ('Q', 3u32),
        ('J', 4u32),
        ('T', 5u32),
        ('9', 6u32),
        ('8', 7u32),
        ('7', 8u32),
        ('6', 9u32),
        ('5', 10u32),
        ('4', 11u32),
        ('3', 12u32),
        ('2', 13u32),
    ]);

    static ref RANKINGS_PART2: HashMap<char, u32> = HashMap::from([
        ('A', 1u32),
        ('K', 2u32),
        ('Q', 3u32),
        ('T', 4u32),
        ('9', 5u32),
        ('8', 6u32),
        ('7', 7u32),
        ('6', 8u32),
        ('5', 9u32),
        ('4', 10u32),
        ('3', 11u32),
        ('2', 12u32),
        ('J', 13u32),
    ]);

}

#[derive(PartialOrd, PartialEq)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    OnePair,
    High
}

struct Bet {
    hand: String,
    hand_type: HandType,
    bid: u32
}

fn get_char_occurrences(hand: &str) -> HashMap<char, u32> {
    hand.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    })
}

fn determine_hand_type(hand: &str) -> HandType {

    let mut occurrences = get_char_occurrences(hand);



    if occurrences.keys().len() == 1 {
        return HandType::Five;
    } else if occurrences.keys().len() == 2 {
        if let Some((c, v)) = occurrences.iter().find(|x| *x.1 == 4) {
            return HandType::Four;
        } else if let Some((c, v)) = occurrences.iter().find(|x| *x.1 == 3) {
            return HandType::Full;
        }
    } else if occurrences.keys().len() == 3 {
        if let Some((c, v)) = occurrences.iter().find(|x| *x.1 == 3) {
            return HandType::Three;
        } else if let Some((c, v)) = occurrences.iter().find(|x| *x.1 == 2) {
            return HandType::TwoPair;
        }
    } else if occurrences.keys().len() == 4 {
        if let Some((c, v)) = occurrences.iter().find(|x| *x.1 == 2) {
            return HandType::OnePair;
        }
    } 
    HandType::High
}

fn determine_hand_type_part2(hand: &str) -> HandType {
    let mut occurrences = get_char_occurrences(hand);

    if occurrences.keys().len() == 1 {
        return HandType::Five;
    }


    let mut joker_occurrences = 0u32;
    if let Some((c, v)) = occurrences.iter().find(|x| *x.0 == 'J') {
        joker_occurrences = *v;
    }
    occurrences.remove_entry(&'J');

    let largest_item = occurrences.iter().reduce(|acc, e| {
        if acc.1 > e.1 {
            acc
        } else {
            e
        }
    }).unwrap();

    occurrences.entry(*largest_item.0).and_modify(|e| *e += joker_occurrences);

    if occurrences.keys().len() == 1 {
        return HandType::Five;
    } else if occurrences.keys().len() == 2 {
        if let Some((c, v)) = occurrences.iter().find(|x| *x.1 == 4) {
            return HandType::Four;
        } else if let Some((c, v)) = occurrences.iter().find(|x| *x.1 == 3) {
            return HandType::Full;
        }
    } else if occurrences.keys().len() == 3 {
        if let Some((c, v)) = occurrences.iter().find(|x| *x.1 == 3) {
            return HandType::Three;
        } else if let Some((c, v)) = occurrences.iter().find(|x| *x.1 == 2) {
            return HandType::TwoPair;
        }
    } else if occurrences.keys().len() == 4 {
        if let Some((c, v)) = occurrences.iter().find(|x| *x.1 == 2) {
            return HandType::OnePair;
        }
    } 

    HandType::High
}

fn parse_bet(line: &str, hand_type: fn(&str) -> HandType) -> Bet {
    let l: Vec<&str> = line.split(" ").collect();
    let (hand, bid) = (l[0].to_string(), l[1].parse::<u32>().unwrap());
    let hand_type = hand_type(hand.as_str());
    
    Bet { hand, hand_type, bid }
}

fn sort_bets(a: &Bet, b: &Bet) -> Ordering {

    if std::mem::discriminant(&a.hand_type) == std::mem::discriminant(&b.hand_type) {
        for (a, b) in a.hand.chars().zip(b.hand.chars()).into_iter() {
            let (a_rank, b_rank) = (RANKINGS.get(&a), RANKINGS.get(&b));
            if a_rank < b_rank {
                return Ordering::Less;
            }
            else if a_rank > b_rank{
                return Ordering::Greater;
            }
        }
    } else {
        if a.hand_type > b.hand_type {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
    panic!("Should not get here");
}

fn sort_bets_part2(a: &Bet, b: &Bet) -> Ordering {

    if std::mem::discriminant(&a.hand_type) == std::mem::discriminant(&b.hand_type) {
        for (a, b) in a.hand.chars().zip(b.hand.chars()).into_iter() {
            let (a_rank, b_rank) = (RANKINGS_PART2.get(&a), RANKINGS_PART2.get(&b));
            if a_rank < b_rank {
                return Ordering::Less;
            }
            else if a_rank > b_rank{
                return Ordering::Greater;
            }
        }
    } else {
        if a.hand_type > b.hand_type {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
    panic!("Should not get here");
}


pub fn part1(input: &str) {
    let bets = common_file::read_file_into_buffer(input);

    let mut bets: Vec<Bet> = bets.iter().map(|x| parse_bet(x.as_str(), determine_hand_type)).collect();

    bets.sort_by(sort_bets);
    let mut total: u32 = 0;
    let highest_rank = bets.len() as u32;
    for (i, bet) in bets.iter().enumerate() {
        total += (highest_rank - (i as u32)) * bet.bid;
    }

    println!("Total is {total}");
} //part 1 answer 248217452

pub fn part2(input: &str) {
    let bets = common_file::read_file_into_buffer(input);

    let mut bets: Vec<Bet> = bets.iter().map(|x| parse_bet(x.as_str(), determine_hand_type_part2)).collect();
    bets.sort_by(sort_bets_part2);
    let mut total = 0u32;
    let highest_rank = bets.len() as u32;
    for (i, bet) in bets.iter().enumerate() {
        total += (highest_rank - (i as u32)) * bet.bid;
    }
    println!("Total with joker is {total}");
}