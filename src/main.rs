use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_line() -> Vec<String> {
    let file = "./temp.txt";
    let input = match File::open(file) {
        Ok(file_input) => file_input,
        Err(error) => panic!("Failed to open the file {}", error),
    };
    let buffered = BufReader::new(input);
    let mut vec = Vec::<String>::new();
    for line in buffered.lines() {
        vec.push(line.unwrap().trim().parse().unwrap());
    }
    vec
}

fn main() {
    let lines = read_line();
    println!("{:?}", lines);
}
