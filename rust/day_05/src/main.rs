use regex::Regex;
use std::io::{self, Read};

fn parse_lines() -> Vec<String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    let mut vec = Vec::new();

    let pattern = Regex::new(r"([BF]{7}[LR]{3})").unwrap();
    for cap in pattern.captures_iter(&buffer) {
        vec.push(cap[1].parse().unwrap());
    }

    return vec;
}

fn parse_bin(input: &String) -> usize {
    let bin = input
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");
    let intval = usize::from_str_radix(&bin, 2).unwrap();
    return intval;
}

fn find_missing_seat(seats: &Vec<usize>) -> usize {
    let mut last_seat = seats.iter().min().unwrap() - 1;

    for seat in seats {
        if *seat != (last_seat + 1) {
            return last_seat + 1;
        } else {
            last_seat = *seat;
        }
    }
    return 0;
}

fn main() {
    let inputs = parse_lines();

    // Convert the inputs to seat numbers and sort them.
    let mut seats: Vec<usize> = inputs.iter().map(|x| parse_bin(x)).collect();
    seats.sort();

    println!("A: {}", seats.iter().max().unwrap());
    println!("B: {}", find_missing_seat(&seats));
}
