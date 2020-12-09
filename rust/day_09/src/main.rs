use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read};
use std::iter::FromIterator;

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

fn part_a(ints: &Vec<i64>, preamble: usize) -> i64 {
    for i in preamble..ints.len() {
        let mut found: bool = false;
        for j in 1..(preamble + 1) {
            for k in 1..(preamble + 1) {
                if ints[i] == (ints[i - j] + ints[i - k]) {
                    found = true;
                }
            }
        }

        if !found {
            return ints[i];
        }
    }

    panic!("Did not find a solution");
}

fn find_set(ints: &Vec<i64>, target: i64, set_size: usize) -> Result<HashSet<i64>, String> {
    for comb in ints.into_iter().combinations(set_size) {
        let comb_iter = comb.iter().map(|i| **i);
        let sum: i64 = comb_iter.clone().sum();
        if sum == target {
            return Ok(HashSet::from_iter(comb_iter));
        }
    }
    Err("No set found of size".to_string())
}

fn part_b(ints: &Vec<i64>, target: i64) -> HashSet<i64> {
    for set_size in 2..ints.len() {
        match find_set(ints, target, set_size) {
            Ok(s) => return s,
            Err(_) => {}
        }
    }

    panic!("Did not find a contiguous set");
}

fn main() {
    let inputs = parse_ints();

    // Part A to determine the target number.
    let invalid_number = part_a(&inputs, 5);
    println!("A: {}", invalid_number);

    // Part B to find the contiguous set.
    let contiguous_set = part_b(&inputs, invalid_number);
    println!("{:?}", contiguous_set);
    println!(
        "B: {}",
        contiguous_set.iter().min().unwrap() + contiguous_set.iter().max().unwrap()
    );
}
