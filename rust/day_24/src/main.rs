use aoc::parse_lines;
use std::collections::{HashMap, HashSet};

fn resolve_tile(input: &String) -> (isize, isize) {
    let mut iter = input.chars();
    let mut x = 0;
    let mut y = 0;

    while let Some(a) = iter.next() {
        if a == 'e' {
            x += 1;
        } else if a == 'w' {
            x -= 1;
        } else {
            let b = iter.next().unwrap();
            if (a == 'n') && (b == 'e') {
                y += 1;
            } else if (a == 'n') && (b == 'w') {
                y += 1;
                x -= 1;
            } else if (a == 's') && (b == 'e') {
                y -= 1;
                x += 1;
            } else if (a == 's') && (b == 'w') {
                y -= 1;
            } else {
                panic!("Unknown instruction.")
            }
        }
    }

    return (x, y);
}

fn get_tiles(inputs: &Vec<String>) -> HashMap<(isize, isize), bool> {
    let mut tiles: HashMap<(isize, isize), bool> = HashMap::new();

    for line in inputs {
        let tile = resolve_tile(line);
        let tiles_clone = tiles.clone();
        match tiles_clone.get(&tile) {
            Some(b) => tiles.insert(tile, !*b),
            None => tiles.insert(tile, true),
        };
    }
    return tiles;
}

fn part_a(inputs: &Vec<String>) -> usize {
    return get_tiles(inputs).iter().filter(|&t| *t.1).count();
}

fn get_neighborhood(t: &(isize, isize)) -> Vec<(isize, isize)> {
    return vec![
        (t.0 + 1, t.1),
        (t.0 - 1, t.1),
        (t.0, t.1 + 1),
        (t.0, t.1 - 1),
        (t.0 + 1, t.1 - 1),
        (t.0 - 1, t.1 + 1),
    ];
}

fn next_state(old: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut new = HashSet::new();
    let mut candidates = HashSet::new();

    for tile in old {
        candidates.insert(tile.clone());
        for neighbor in get_neighborhood(tile) {
            candidates.insert(neighbor);
        }
    }

    for candidate in candidates {
        let mut count = 0;
        for n in get_neighborhood(&candidate) {
            if old.contains(&n) {
                count += 1;
            }
        }

        if (count == 2) || ((count == 1) && old.contains(&candidate)) {
            new.insert(candidate);
        }
    }

    return new;
}

fn part_b(inputs: &Vec<String>) -> usize {
    let mut tiles: HashSet<(isize, isize)> = get_tiles(inputs)
        .iter()
        .filter(|&t| *t.1)
        .map(|t| *t.0)
        .collect();

    for _ in 0..100 {
        tiles = next_state(&tiles);
    }

    return tiles.len();
}

fn main() {
    let inputs = parse_lines();

    println!("A: {}", part_a(&inputs));
    println!("B: {}", part_b(&inputs));
}
