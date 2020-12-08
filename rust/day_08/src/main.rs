use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Operation {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(input: &str) -> Result<Operation, Self::Err> {
        let mut split = input.split(" ");
        let com = split.next().unwrap();
        let val: i32 = split.next().unwrap().parse().unwrap();
        match com {
            "acc" => Ok(Operation::Acc(val)),
            "jmp" => Ok(Operation::Jmp(val)),
            "nop" => Ok(Operation::Nop(val)),
            _ => Err(()),
        }
    }
}

fn parse_lines() -> Vec<Operation> {
    let mut vec = Vec::new();
    let operation_regex = Regex::new(r"\w+ [+-](\d+)").unwrap();

    // Get the stdin and read it into a buffer.
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    // Iterate over all rules.
    for cap in operation_regex.captures_iter(&buffer) {
        vec.push(Operation::from_str(&cap[0]).unwrap());
    }

    return vec;
}

fn part_a(inputs: &Vec<Operation>) -> i32 {
    let mut accumulator: i32 = 0;
    let mut index: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();

    loop {
        visited.insert(index);

        match inputs.get(index as usize).unwrap() {
            Operation::Acc(i) => {
                accumulator += i;
                index += 1;
            }
            Operation::Jmp(i) => index += *i,
            Operation::Nop(_) => index += 1,
        }

        if visited.contains(&index) {
            return accumulator;
        }
    }
}

fn part_b(mut inputs: Vec<Operation>) -> i32 {
    for m in 0..inputs.len() {
        // Manipulate one instruction.
        match &inputs[m] {
            Operation::Acc(_) => continue,
            Operation::Jmp(i) => inputs[m] = Operation::Nop(*i),
            Operation::Nop(i) => inputs[m] = Operation::Jmp(*i),
        }

        // Set up for program execution.
        let mut accumulator: i32 = 0;
        let mut index: i32 = 0;
        let mut visited: HashSet<i32> = HashSet::new();

        while !visited.contains(&index) {
            visited.insert(index);

            match inputs.get(index as usize).unwrap() {
                Operation::Acc(i) => {
                    accumulator += i;
                    index += 1;
                }
                Operation::Jmp(i) => index += *i,
                Operation::Nop(_) => index += 1,
            }

            if index == (inputs.len() as i32) {
                return accumulator;
            }
        }

        // Revert manipulation.
        match &inputs[m] {
            Operation::Acc(_) => continue,
            Operation::Jmp(i) => inputs[m] = Operation::Nop(*i),
            Operation::Nop(i) => inputs[m] = Operation::Jmp(*i),
        }
    }
    return 0;
}

fn main() {
    let inputs = parse_lines();
    println!("A: {}", part_a(&inputs));
    println!("B: {}", part_b(inputs));
}
