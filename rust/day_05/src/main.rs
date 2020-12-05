use regex::Regex;
use std::io::{self, Read};


fn parse_lines() -> Vec<(String, String)> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    let mut vec = Vec::new();

    let pattern = Regex::new(r"([BF]{7})([LR]{3})").unwrap();
    for cap in pattern.captures_iter(&buffer) {
        vec.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
    }

    return vec;
}

fn parse_row(input: &String) -> usize {
    let bin = input.replace("F", "0").replace("B", "1");
    let intval = usize::from_str_radix(&bin, 2).unwrap();
    return intval;
}

fn parse_column(input: &String) -> usize {
    let bin = input.replace("L", "0").replace("R", "1");
    let intval = usize::from_str_radix(&bin, 2).unwrap();
    return intval;
}

fn parse_seats(inputs: &Vec<(String, String)>) -> Vec<usize> {
    let mut vec = Vec::new();

    for input in inputs {
        let seat = parse_row(&input.0) * 8 + parse_column(&input.1);
        vec.push(seat);
    }

    vec.sort();
    return vec;
}

fn find_missing_seat(inputs: &Vec<(String, String)>) -> usize {
    let mut last_seat = parse_seats(inputs).iter().min().unwrap() - 1;

    for seat in parse_seats(inputs) {
        if seat != (last_seat + 1) {
            return last_seat + 1;
        }
        else {
            last_seat = seat;
        }
    }
    return 0;
}

fn main() {
    let inputs = parse_lines();
    println!("A: {}", parse_seats(&inputs).iter().max().unwrap());
    println!("B: {}", find_missing_seat(&inputs));
}
