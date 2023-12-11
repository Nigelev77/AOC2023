use regex::Regex;


pub fn parse_nums_in_str(input: &str) -> Vec<u32> {
    let number_re: Regex = Regex::new(r"\d+").unwrap();
    number_re.find_iter(&input)
             .map(|x| x.as_str().parse::<u32>().unwrap()).collect()
}

pub fn parse_digits_into_number(input: &str) -> u64 {
    let number_re: Regex = Regex::new(r"\d+").unwrap();
    number_re.find_iter(input).map(|x| x.as_str())
        .fold(String::new(), |acc, x| acc + x).parse().unwrap()
}


pub fn parse_digits_by_delimiter_u64(input: &str, delimiter: &str) -> Vec<u64> {
    input.split(delimiter).map(|x| x.parse::<u64>().unwrap()).collect()
}

pub fn parse_digits_by_delimiter_i64(input: &str, delimiter: &str) -> Vec<i64> {
    input.split(delimiter).map(|x| x.parse::<i64>().unwrap()).collect()
}