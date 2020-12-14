use aoc::parse_lines;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Mem((usize, usize)),
}

fn parse_inputs() -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let mask = Regex::new(r"mask = ([01X]+)").unwrap();
    let mem = Regex::new(r"mem\[(\d+)\] = ([\d]+)").unwrap();
    for line in parse_lines() {
        match mask.captures(&line) {
            Some(m) => instructions.push(Instruction::Mask(m[1].to_string())),
            None => {}
        }

        match mem.captures(&line) {
            Some(m) => instructions.push(Instruction::Mem((
                m[1].parse().unwrap(),
                m[2].parse().unwrap(),
            ))),
            None => {}
        }
    }

    return instructions;
}

fn to_bin(number: &usize) -> Vec<char> {
    return format!("{:0>36}", format!("{:b}", number))
        .chars()
        .collect();
}

fn from_bin(bin_number: &Vec<char>) -> usize {
    let bin_str: String = bin_number.into_iter().collect();
    return usize::from_str_radix(&bin_str, 2).unwrap();
}

fn apply_mask_a(number: &usize, mask: &Vec<char>) -> usize {
    let mut bin_number = to_bin(number);

    for i in 0..36 {
        match mask[i] {
            '0' => bin_number[i] = '0',
            '1' => bin_number[i] = '1',
            'X' => continue,
            _ => panic!("Invalid value in mask!"),
        }
    }

    return from_bin(&bin_number);
}

fn part_a(instructions: &Vec<Instruction>) -> HashMap<usize, usize> {
    let mut current_mask: Vec<char> = Vec::new();
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => current_mask = m.clone().chars().collect(),
            Instruction::Mem(t) => {
                match memory.insert(t.0.clone(), apply_mask_a(&t.1, &current_mask)) {
                    Some(_) => (),
                    None => (),
                }
            }
        }
    }

    return memory;
}

fn main() {
    let inputs = parse_inputs();

    println!("A: {}", part_a(&inputs).values().sum::<usize>());
    println!("B: {}", 0);
}
