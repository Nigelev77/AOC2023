use crate::util::common_file;
use regex::Regex;


pub fn part1(input: &str){
    let file_string = common_file::read_file(input);
    let r_cubes = 12u32;
    let g_cubes = 13u32;
    let b_cubes = 14u32;

    let r_re: Regex = Regex::new(r"(?<number>[0-9]*)\s(red)(,?)").unwrap();
    let g_re: Regex = Regex::new(r"(?<number>[0-9]*)\s(green)(,?)").unwrap();
    let b_re: Regex = Regex::new(r"(?<number>[0-9]*)\s(blue)(,?)").unwrap();


    let mut total = 0usize;
    for (line_number, line) in file_string.split("\n").enumerate(){
        let game_number = line_number+1;
        let configurations: &str = &line[line.find(": ").unwrap()..];
        let mut possible = true;
        for configuration in configurations.split(";"){
            if let Some(r) = r_re.captures(configuration){
                let (_, [red, _, _]) = r.extract();
                if red.parse::<u32>().unwrap() > r_cubes {
                    possible = false;
                    break;
                }
            }
            if let Some(g) = g_re.captures(configuration){
                let (_, [green, _, _]) = g.extract();
                if green.parse::<u32>().unwrap() > g_cubes{
                    possible = false;
                    break;
                }
            }

            if let Some(b) = b_re.captures(configuration){
                let (_, [blue, _, _]) = b.extract();
                if blue.parse::<u32>().unwrap() > b_cubes{
                    possible = false;
                    break;
                }
            }
        }
        if possible{
            total += game_number;
        }
    }

    println!("Total indices = {total}");

}

pub fn part2(input: &str){
    let file_string = common_file::read_file(input);

    let r_re: Regex = Regex::new(r"(?<number>[0-9]*)\s(red)(,?)").unwrap();
    let g_re: Regex = Regex::new(r"(?<number>[0-9]*)\s(green)(,?)").unwrap();
    let b_re: Regex = Regex::new(r"(?<number>[0-9]*)\s(blue)(,?)").unwrap();

    let mut total = 0u32;
    for line in file_string.split("\n") {
        let configurations: &str = &line[line.find(": ").unwrap()..];
        let mut r_min = 0;
        let mut g_min = 0;
        let mut b_min = 0;
        for configuration in configurations.split(";"){
            if let Some(r) = r_re.captures(configuration){
                let (_, [red, _, _]) = r.extract();
                let red = red.parse::<u32>().unwrap();
                r_min = r_min.max(red);
            }
            if let Some(g) = g_re.captures(configuration){
                let (_, [green, _, _]) = g.extract();
                let green = green.parse::<u32>().unwrap();
                g_min = g_min.max(green);
            }

            if let Some(b) = b_re.captures(configuration){
                let (_, [blue, _, _]) = b.extract();
                let blue = blue.parse::<u32>().unwrap();
                b_min = b_min.max(blue);
            }
        }
        
        total += r_min * g_min * b_min;
    }

    println!("Total of power sets = {total}");
}