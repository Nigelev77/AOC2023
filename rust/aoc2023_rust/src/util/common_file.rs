use std::{fs::File, io::Read};


pub fn read_file(path: &str) -> String {
    let mut file: File = match File::open(path) {
        Ok(f) => f,
        Err(_) => panic!(),
    };
    let mut file_string: String = String::new();
    let _ = file.read_to_string(&mut file_string);
    /*returns */file_string
}