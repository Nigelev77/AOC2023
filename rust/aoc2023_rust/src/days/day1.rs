use std::{fs::File, io::Read};
use regex::Regex;

pub fn part1(input: &str){
    let mut file: File = match File::open(input) {
        Ok(f) => f,
        Err(_) => panic!(),
    };
    let mut file_string: String = String::new();
    let _ = file.read_to_string(&mut file_string);
    let mut total = 0;

    if !file_string.is_empty(){
        for calibration in file_string.split("\n"){
            let digits: String = calibration.chars().filter(|c| c.is_digit(10)).collect();
            let first_digit = digits.chars().nth(0).unwrap().to_digit(10).unwrap();
            let last_digit = digits.chars().nth_back(0).unwrap().to_digit(10).unwrap();
            total += first_digit*10 + last_digit;
        }

        println!("Total calibration value for first part is: {total}");
    }
}

pub fn part2(input: &str){
    let mut file: File = match File::open(input) {
        Ok(f) => f,
        Err(_) => panic!(),
    };
    let mut file_string: String = String::new();
    let _ = file.read_to_string(&mut file_string);
    let mut total = 0;
    let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    if !file_string.is_empty(){
        for calibration in file_string.split("\n"){
            let mut digits: Vec<u32> = Vec::new();
            
            let mut                left = 0;
            let mut right = 1;

            while right <= calibration.len(){
                let substr: &str = &calibration[left..right];
                let found_match = re.find(substr);
                 match found_match{
                    None => {
                        right += 1;
                        continue;
                    },
                    Some(matching_sym) =>{
                        let candidate = matching_sym.as_str();
                         let number = if candidate.len() == 1 {candidate.chars().nth(0).unwrap().to_digit(10)} else {
                            match candidate{
                                "one" => Some(1),
                                "two" => Some(2),
                                "three" => Some(3),
                                "four" => Some(4),
                                "five" => Some(5),
                                "six" => Some(6),
                                "seven" => Some(7),
                                "eight" => Some(8),
                                "nine" => Some(9),
                                _ => None,
                            }
                        };
                        match number{
                            Some(x) => digits.push(x),
                            None => continue
                        };
                        left = if candidate.len() != 1 {right -1} else {right};
                        right = if right < candidate.len() {left + 1} else {right + 1};

                    }
                }
            }

            let first_digit = digits[0];
            let last_digit = digits[digits.len()-1];
            total += first_digit*10 + last_digit;

            
        }

        println!("Total calibration value for second part is {total}");
    }
}