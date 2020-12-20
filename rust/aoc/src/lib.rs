use regex::Regex;
use std::io::{self, BufRead, Read};

pub fn parse_ints() -> Vec<isize> {
    let mut vec = Vec::new();
    let re = Regex::new(r"([-+]?\d+)\D?").unwrap();

    // Get the stdin and read it into a buffer.
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    for cap in re.captures_iter(&buffer) {
        vec.push(cap[1].parse().unwrap());
    }

    return vec;
}

pub fn parse_lines() -> Vec<String> {
    let mut vec = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        vec.push(line);
    }

    return vec;
}

pub fn parse_blocks() -> Vec<Vec<String>> {
    let mut vec = Vec::new();

    // Get the stdin and read it into a buffer.
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    for block in buffer.split("\n\n") {
        let mut block_vec: Vec<String> = Vec::new();
        for line in block.split("\n") {
            block_vec.push(line.to_string());
        }
        vec.push(block_vec);
    }

    return vec;
}
