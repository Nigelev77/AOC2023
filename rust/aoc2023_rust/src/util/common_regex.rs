use regex::Regex;


pub fn parse_nums_in_str(input: &str) -> Vec<u32> {
    let number_re: Regex = Regex::new(r"\d+").unwrap();
    number_re.find_iter(&input)
             .map(|x| x.as_str().parse::<u32>().unwrap()).collect()
}