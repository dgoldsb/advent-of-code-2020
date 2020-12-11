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
            map.insert((i as i64, j as i64), state);
        }
    }
    return map;
}

// Extended Von Neumann neighborhood.
fn count_neighbours_evn(map: &HashMap<(i64, i64), char>, key: &(i64, i64)) -> usize {
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

    return full_count;
}

// Look recursively, will return 1 or 0.
fn look(map: &HashMap<(i64, i64), char>, loc: &(i64, i64), dx: i64, dy: i64) -> usize {
    // Apply the delta.
    let new_loc = (loc.0 + dx, loc.1 + dy);
    match map.get(&new_loc) {
        Some(state) => {
            if *state == FULL {
                return 1;
            } else if *state == EMPTY {
                return 0;
            } else {
                return look(map, &new_loc, dx.clone(), dy.clone());
            }
        }
        None => return 0,
    }
}

// Line of sight.
fn count_los(map: &HashMap<(i64, i64), char>, key: &(i64, i64)) -> usize {
    let mut full_count: usize = 0;

    for i in -1..2 {
        for j in -1..2 {
            if (i == 0) && (j == 0) {
                continue;
            }
            full_count += look(map, key, i, j);
        }
    }

    return full_count;
}

fn play_round(map: &HashMap<(i64, i64), char>, part_a: &bool) -> HashMap<(i64, i64), char> {
    let mut new_map = HashMap::new();
    for (key, value) in map.iter() {
        if *value == FLOOR {
            new_map.insert(*key, *value);
            continue;
        }

        let full_count: usize;

        if part_a.clone() {
            full_count = count_neighbours_evn(map, key);
        } else {
            full_count = count_los(map, key);
        }

        if full_count == 0 {
            new_map.insert(*key, FULL);
        } else if part_a.clone() && (full_count >= 4) {
            new_map.insert(*key, EMPTY);
        } else if !part_a.clone() && (full_count >= 5) {
            new_map.insert(*key, EMPTY);
        } else {
            new_map.insert(*key, *value);
        }
    }
    return new_map;
}

fn solve(map: &HashMap<(i64, i64), char>, part_a: bool) -> usize {
    let mut old_count: usize = 0;
    let mut new_map = map.clone();

    loop {
        new_map = play_round(&new_map, &part_a);
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
    println!("{:?}", solve(&inputs, true));
    println!("{:?}", solve(&inputs, false));
}
