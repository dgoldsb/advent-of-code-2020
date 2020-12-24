use aoc::parse_lines;
use std::collections::HashMap;

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

fn part_a(inputs: &Vec<String>) -> usize {
    let mut tiles: HashMap<(isize, isize), bool> = HashMap::new();

    for line in inputs {
        let tile = resolve_tile(line);
        let tiles_clone = tiles.clone();
        match tiles_clone.get(&tile) {
            Some(b) => tiles.insert(tile, !*b),
            None => tiles.insert(tile, true),
        };
    }

    return tiles.iter().filter(|&t| *t.1).count();
}

fn main() {
    let inputs = parse_lines();

    println!("A: {}", part_a(&inputs));
}
