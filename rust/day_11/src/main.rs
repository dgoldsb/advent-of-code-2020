use std::collections::HashMap;
use std::io::{self, BufRead};

const EMPTY: char = 'L';
const FLOOR: char = '.';
const FULL: char = '#';

fn parse_lines() -> HashMap<(i64, i64), char> {
    let mut map = HashMap::new();

    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        let line = line.expect("Could not read line from standard in");
        for j in 0..line.len() {
            let state = line.chars().nth(j).unwrap();
            if state != FLOOR {
                map.insert((i as i64, j as i64), state);
            }
        }
    }
    return map;
}

fn play_round(map: &HashMap<(i64, i64), char>) -> HashMap<(i64, i64), char> {
    let mut new_map = HashMap::new();
    for (key, value) in map.iter() {
        let mut full_count: usize = 0;

        for i in -1..2 {
            for j in -1..2 {
                if (i == 0) && (j == 0) {
                    continue;
                }
                match map.get(&(key.0 + i, key.1 + j)) {
                    Some(state) => {
                        if *state == FULL {
                            full_count += 1;
                        }
                    }
                    None => {}
                }
            }
        }

        if full_count == 0 {
            new_map.insert(*key, FULL);
        } else if full_count >= 4 {
            new_map.insert(*key, EMPTY);
        } else {
            new_map.insert(*key, *value);
        }
    }
    return new_map;
}

fn part_a(map: &HashMap<(i64, i64), char>) -> usize {
    let mut old_count: usize = 0;
    let mut new_map = map.clone();

    loop {
        new_map = play_round(&new_map);
        let new_count = new_map.values().filter(|c| **c == FULL).count();
        if new_count == old_count {
            return new_count;
        } else {
            old_count = new_count;
        }
    }
}

fn main() {
    let inputs = parse_lines();
    println!("{:?}", part_a(&inputs));
}
