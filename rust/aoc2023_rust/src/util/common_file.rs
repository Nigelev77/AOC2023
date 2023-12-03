use std::{fs::File, io::{Read, self, BufRead}};
use std::path::Path;


pub fn read_file(path: &str) -> String {
    let mut file: File = match File::open(path) {
        Ok(f) => f,
        Err(_) => panic!(),
    };
    let mut file_string: String = String::new();
    let _ = file.read_to_string(&mut file_string);
    /*returns */file_string
}

pub fn read_file_into_iterator(path: &str) -> io::Lines<io::BufReader<File>>{
    let file: Result<File, io::Error> = File::open(path);
    match file {
        Ok(f) => io::BufReader::new(f).lines(),
        Err(_) => panic!("Could not find file"),
    }
}