use regex::Regex;
use std::io::{self, Read};

const DIRS: [&'static char; 4] = [&'E', &'S', &'W', &'N'];

fn parse_inputs() -> Vec<(char, i64)> {
    let mut vec = Vec::new();
    let re = Regex::new(r"([NESWLRF])(\d+)").unwrap();

    // Get the stdin and read it into a buffer.
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    for cap in re.captures_iter(&buffer) {
        vec.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
    }

    return vec;
}

fn translate_inputs(inputs: &Vec<(char, i64)>) -> Vec<(i64, i64)> {
    let mut new_vec = Vec::new();

    let mut dir_pointer: usize = 0;

    for (ch, digits) in inputs {
        let mut direction = ch.clone();

        // Update the pointer and replace the direction.
        if direction == 'L' {
            let ticks: usize = (*digits / 90) as usize;
            dir_pointer = (dir_pointer + 4 - ticks) % 4;
            continue;
        } else if direction == 'R' {
            let ticks: usize = (*digits / 90) as usize;
            dir_pointer = (dir_pointer + ticks) % 4;
            continue;
        }

        // If we go forward, just replace the direction.
        if direction == 'F' {
            direction = DIRS[dir_pointer].clone();
        }

        // Push the newly improved (dx, dy)!
        if direction == 'N' {
            new_vec.push((*digits, 0));
        } else if direction == 'S' {
            new_vec.push((-*digits, 0));
        } else if direction == 'E' {
            new_vec.push((0, *digits));
        } else if direction == 'W' {
            new_vec.push((0, -*digits));
        }
    }

    return new_vec;
}

fn part_a(inputs: &Vec<(char, i64)>) -> i64 {
    let translated = translate_inputs(inputs);
    return translated.iter().map(|t| t.0).sum::<i64>().abs()
        + translated.iter().map(|t| t.1).sum::<i64>().abs();
}

fn main() {
    let inputs = parse_inputs();
    println!("{}", part_a(&inputs));
}
