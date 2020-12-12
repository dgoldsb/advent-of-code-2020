use regex::Regex;
use std::io::{self, Read};

pub fn parse_ints() -> Vec<isize> {
    let mut vec = Vec::new();
    let re = Regex::new(r"([-+]?\d+)\w?").unwrap();

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
