use regex::Regex;
use std::io::{self, Read};

fn parse_ints() -> Vec<i64> {
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

fn part_a(inputs: &Vec<i64>) -> usize {
    let mut cloned_inputs = inputs.clone();

    cloned_inputs.push(0);
    cloned_inputs.push(inputs.iter().max().unwrap() + 3);
    cloned_inputs.sort();

    let mut differences = Vec::new();
    for i in 1..cloned_inputs.len() {
        differences.push(cloned_inputs[i] - cloned_inputs[i - 1]);
    }

    return differences.iter().filter(|x| **x == 1).count()
        * differences.iter().filter(|x| **x == 3).count();
}

fn main() {
    let inputs = parse_ints();
    println!("A: {}", part_a(&inputs));
}
