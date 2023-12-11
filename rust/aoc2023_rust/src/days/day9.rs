use crate::util::common_file;
use crate::util::common_math;
use crate::util::common_regex;

pub fn predict_next_value(seq: &Vec<i64>) -> i64 {

    let diff: Vec<i64> = common_math::get_first_differences_i64(seq);
    if diff.iter().all(|x| *x == 0i64) {
        return *diff.last().unwrap();
    } else {
        return diff.last().unwrap() + predict_next_value(&diff);
    }
}

pub fn predict_prev_value(seq: &Vec<i64>) -> i64 {

    let diff: Vec<i64> = common_math::get_first_differences_i64(seq).into_iter().map(|x| x * -1).collect();
    if diff.iter().all(|x| *x == 0i64) {
        return *diff.last().unwrap();
    } else {
        return diff.last().unwrap() - predict_prev_value(&diff);
    }
}

pub fn part1(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);

    let mut total = 0i64;
    for seq_str in file_string {
        let seq = common_regex::parse_digits_by_delimiter_i64(&seq_str, &" ");
        total += seq.last().unwrap() + predict_next_value(&seq);
    }

    println!("Total for next extrapolation is {total}");
}

pub fn part2(input: &str) {
    let file_string = common_file::read_file_into_buffer(input);

    let mut total = 0i64;
    for seq_str in file_string {
        let seq = common_regex::parse_digits_by_delimiter_i64(&seq_str, &" ");
        let seq: Vec<i64> = seq.into_iter().rev().collect();
        let prev_value: i64 = seq.last().unwrap() - predict_prev_value(&seq);
        total += prev_value;
    }

    println!("Total for prev extrapolation is {total}");
}