use regex::Regex;
use std::io::{self, Read};

fn parse_inputs() -> Vec<isize> {
    let mut vec = Vec::new();
    let re = Regex::new(r"([-+]?\d+|x)\D?").unwrap();

    // Get the stdin and read it into a buffer.
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    for cap in re.captures_iter(&buffer) {
        vec.push(cap[1].replace("x", "0").parse().unwrap());
    }

    return vec;
}

fn part_a(earliest_leave: &isize, bus_ids: &Vec<isize>) -> isize {
    let soonest = bus_ids
        .iter()
        .filter(|b| **b != 0)
        .map(|b| (((((*earliest_leave - (*earliest_leave % b)) / b) + 1) * b), b.clone()))
        .min()
        .unwrap();

    return soonest.1 * (soonest.0 - *earliest_leave);
}

fn main() {
    let inputs = parse_inputs();

    println!("A: {}", part_a(&(inputs[0]), &inputs[1..].to_vec()));
}
