use aoc::parse_ints;
use std::collections::HashSet;
use std::iter::FromIterator;

fn part_a(ints: &Vec<isize>, preamble: usize) -> isize {
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

fn find_set(
    ints: &Vec<isize>,
    target: isize,
    start: usize,
    end: usize,
) -> Result<HashSet<isize>, bool> {
    let iter = ints[start..end].iter().map(|i| *i);
    let sum: isize = iter.clone().sum();
    if sum == target {
        return Ok(HashSet::from_iter(iter));
    }
    Err(sum > target)
}

fn part_b(ints: &Vec<isize>, target: isize) -> HashSet<isize> {
    for i in 0..(ints.len() - 1) {
        for j in i..ints.len() {
            match find_set(ints, target, i, j) {
                Ok(s) => return s,
                Err(b) => {
                    // If we overshot the target, break out of the loop.
                    if b {
                        break;
                    }
                }
            }
        }
    }

    panic!("Did not find a contiguous set");
}

fn main() {
    let inputs = parse_ints();

    // Part A to determine the target number.
    let invalid_number = part_a(&inputs, 25);
    println!("A: {}", invalid_number);

    // Part B to find the contiguous set.
    let contiguous_set = part_b(&inputs, invalid_number);
    println!(
        "B: {}",
        contiguous_set.iter().min().unwrap() + contiguous_set.iter().max().unwrap()
    );
}
