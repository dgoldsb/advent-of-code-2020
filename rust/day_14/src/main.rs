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

fn apply_mask_a(number: &usize, mask: &String) -> usize {
    return (number | usize::from_str_radix(&mask.clone().replace("X", "0"), 2).unwrap())
        & usize::from_str_radix(&mask.clone().replace("X", "1"), 2).unwrap();
}

fn part_a(instructions: &Vec<Instruction>) -> HashMap<usize, usize> {
    let mut current_mask: String = "".to_string();
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => current_mask = m.clone(),
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

fn to_bin(number: &usize) -> Vec<char> {
    return format!("{:0>36}", format!("{:b}", number))
        .chars()
        .collect();
}

fn from_bin(bin_number: &Vec<char>) -> usize {
    let bin_str: String = bin_number.into_iter().collect();
    return usize::from_str_radix(&bin_str, 2).unwrap();
}

fn apply_floating_bits(input: &Vec<char>) -> Vec<Vec<char>> {
    match input.iter().position(|c| *c == 'X') {
        Some(i) => {
            let mut input_0 = input.clone();
            input_0[i] = '0';
            let mut input_1 = input.clone();
            input_1[i] = '1';
            let mut vec = apply_floating_bits(&input_0);
            vec.extend(apply_floating_bits(&input_1));
            vec
        }
        None => vec![input.clone()],
    }
}

fn apply_mask_b(number: &usize, mask: &Vec<char>) -> Vec<usize> {
    let mut bin_number = to_bin(number);

    for i in 0..36 {
        match mask[i] {
            '0' => continue,
            '1' => bin_number[i] = '1',
            'X' => bin_number[i] = 'X',
            _ => panic!("Invalid value in mask!"),
        }
    }

    let bin_numbers = apply_floating_bits(&bin_number);

    return bin_numbers.iter().map(|b| from_bin(&b)).collect();
}

fn part_b(instructions: &Vec<Instruction>) -> HashMap<usize, usize> {
    let mut current_mask: Vec<char> = Vec::new();
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => current_mask = m.clone().chars().collect(),
            Instruction::Mem(t) => {
                for mem in apply_mask_b(&t.0, &current_mask) {
                    memory.insert(mem, t.1.clone());
                }
            }
        }
    }

    return memory;
}

fn main() {
    let inputs = parse_inputs();

    println!("A: {}", part_a(&inputs).values().sum::<usize>());
    println!("B: {}", part_b(&inputs).values().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part_a(&vec![
                Instruction::Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
                Instruction::Mem((8, 11)),
                Instruction::Mem((7, 101)),
                Instruction::Mem((8, 0))
            ])
            .values()
            .sum::<usize>(),
            165
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part_b(&vec![
                Instruction::Mask("000000000000000000000000000000X1001X".to_string()),
                Instruction::Mem((42, 100)),
                Instruction::Mask("00000000000000000000000000000000X0XX".to_string()),
                Instruction::Mem((26, 1))
            ])
            .values()
            .sum::<usize>(),
            208
        );
    }
}
