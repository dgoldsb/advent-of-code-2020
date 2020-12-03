use regex::Regex;
use std::io::{self, Read};

struct DatabaseEntry {
    x: usize,
    y: usize,
    character: String,
    password: String,
}

fn parse_lines() -> Vec<DatabaseEntry> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {} lines", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    let mut vec = Vec::new();

    let pattern = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    for cap in pattern.captures_iter(&buffer) {
        let entry = DatabaseEntry {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            character: cap[3].to_string(),
            password: cap[4].to_string(),
        };
        vec.push(entry);
    }

    return vec;
}

fn part_a(entries: &Vec<DatabaseEntry>) -> i32 {
    let mut counter = 0;

    for entry in entries {
        let char_count: usize = entry.password.matches(&entry.character).count();
        if entry.x <= char_count && char_count <= entry.y {
            counter += 1;
        }
    }

    return counter;
}

fn part_b(entries: &Vec<DatabaseEntry>) -> i32 {
    let mut counter = 0;

    for entry in entries {
        let first: String = entry.password.chars().nth(entry.x - 1).unwrap().to_string();
        let second: String = entry.password.chars().nth(entry.y - 1).unwrap().to_string();
        if (first == entry.character || second == entry.character) && !(first == second) {
            counter += 1;
        }
    }

    return counter;
}

fn main() {
    let inputs = parse_lines();
    println!("A: {}", part_a(&inputs));
    println!("B: {}", part_b(&inputs));
}
